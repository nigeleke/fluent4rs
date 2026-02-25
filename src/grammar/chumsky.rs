use chumsky::prelude::*;

use crate::{ast::*, error::Fluent4rsError};

pub fn parse_resource(text: &str) -> Result<Resource, Fluent4rsError> {
    resource().parse(text).into_result().map_err(|es| {
        Fluent4rsError::ChumskyParser(
            es.iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        )
    })
}

pub fn resource<'a>() -> impl Parser<'a, &'a str, Resource, extra::Err<Simple<'a, char>>> {
    choice((
        entry().map(ResourceItem::Entry),
        blank_block().map(|s| {
            let s = String::from(s);
            ResourceItem::BlankBlock(s)
        }),
        junk().map(ResourceItem::Junk),
    ))
    .repeated()
    .collect::<Vec<_>>()
    .map(Resource::from)
    .labelled("resource")
}

fn entry<'a>() -> impl Parser<'a, &'a str, Entry, extra::Err<Simple<'a, char>>> {
    choice((
        message().then_ignore(line_end()).map(Entry::Message),
        term().then_ignore(line_end()).map(Entry::Term),
        comment_line().map(Entry::CommentLine),
    ))
    .labelled("entry")
}

fn message<'a>() -> impl Parser<'a, &'a str, Message, extra::Err<Simple<'a, char>>> {
    identifier()
        .then_ignore(blank_inline().or_not())
        .then_ignore(just('='))
        .then_ignore(blank_inline().or_not())
        .then(
            pattern()
                .then(attribute().repeated().collect())
                .map(|(p, a)| MessageArguments::Patterned(p, a))
                .or(attribute()
                    .repeated()
                    .at_least(1)
                    .collect()
                    .map(MessageArguments::Plain)),
        )
        .map(|(i, a)| Message::new(i, a))
        .labelled("message")
}

fn term<'a>() -> impl Parser<'a, &'a str, Term, extra::Err<Simple<'a, char>>> {
    just('-')
        .ignore_then(identifier())
        .then_ignore(blank_inline().or_not())
        .then_ignore(just('='))
        .then_ignore(blank_inline().or_not())
        .then(pattern())
        .then(attribute().repeated().collect())
        .map(|((id, p), attrs)| Term::new(id, p, attrs))
        .labelled("term")
}

fn comment_line<'a>() -> impl Parser<'a, &'a str, CommentLine, extra::Err<Simple<'a, char>>> {
    choice((just("###"), just("##"), just("#")))
        .then(
            just('\u{0020}')
                .ignore_then(
                    comment_char()
                        .repeated()
                        .collect::<Vec<_>>()
                        .map(|vs| vs.join("")),
                )
                .or_not(),
        )
        .then_ignore(line_end())
        .map(|(lead, comment)| CommentLine::new(lead.into(), comment))
        .labelled("comment_line")
}

fn comment_char<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Simple<'a, char>>> {
    line_end().not().ignore_then(any_char())
}

fn junk<'a>() -> impl Parser<'a, &'a str, Junk, extra::Err<Simple<'a, char>>> {
    junk_line()
        .then(
            (just('#')
                .or(just('-'))
                .or(any().filter(|c: &char| c.is_ascii_alphabetic())))
            .not()
            .ignore_then(junk_line())
            .repeated()
            .collect::<Vec<_>>(),
        )
        .map(|(head, tail)| {
            let lines = Vec::from_iter(std::iter::once(head).chain(tail).map(str::to_string));
            Junk::from(lines.as_slice())
        })
        .labelled("junk")
}

fn junk_line<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Simple<'a, char>>> {
    none_of('\n').repeated().then(just('\u{000a}')).to_slice()
}

fn attribute<'a>() -> impl Parser<'a, &'a str, Attribute, extra::Err<Simple<'a, char>>> {
    line_end()
        .ignore_then(blank().or_not())
        .ignore_then(just('.'))
        .ignore_then(identifier())
        .then_ignore(blank_inline().or_not())
        .then_ignore(just('='))
        .then_ignore(blank_inline().or_not())
        .then(pattern())
        .map(|(id, p)| Attribute::new(id, p))
        .labelled("attribute")
}

fn pattern<'a>() -> impl Parser<'a, &'a str, Pattern, extra::Err<Simple<'a, char>>> {
    recursive(|pattern| {
        pattern_element(pattern)
            .repeated()
            .at_least(1)
            .collect::<Vec<_>>()
            .map(|ps| Pattern::from(ps.as_slice()))
    })
    .labelled("pattern")
}

fn pattern_element<'a>(
    pattern: impl Parser<'a, &'a str, Pattern, extra::Err<Simple<'a, char>>> + Clone + 'a,
) -> impl Parser<'a, &'a str, PatternElement, extra::Err<Simple<'a, char>>> + Clone {
    choice((
        inline_text().map(PatternElement::InlineText).boxed(),
        block_text().map(PatternElement::BlockText).boxed(),
        inline_placeable(pattern.clone())
            .map(PatternElement::InlinePlaceable)
            .boxed(),
        block_placeable(pattern)
            .map(PatternElement::BlockPlaceable)
            .boxed(),
    ))
    .labelled("pattern_element")
}

fn inline_text<'a>() -> impl Parser<'a, &'a str, InlineText, extra::Err<Simple<'a, char>>> {
    text_char()
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>()
        .map(|vs| vs.join(""))
        .map(|s| InlineText::from(s.as_str()))
        .labelled("inline_text")
}

fn block_text<'a>() -> impl Parser<'a, &'a str, BlockText, extra::Err<Simple<'a, char>>> {
    blank_block()
        .then(blank_inline())
        .then(indented_char())
        .then(inline_text().or_not())
        .map(|(((blank_block, blank_inline), indent), inline)| {
            let s = format!(
                "{blank_block}{blank_inline}{indent}{}",
                inline.map_or(String::default(), |c| c.to_string())
            );
            BlockText::from(s.as_str())
        })
        .labelled("block_text")
}

fn inline_placeable<'a>(
    pattern: impl Parser<'a, &'a str, Pattern, extra::Err<Simple<'a, char>>> + Clone + 'a,
) -> impl Parser<'a, &'a str, InlinePlaceable, extra::Err<Simple<'a, char>>> + Clone + 'a {
    recursive(|inline_placeable| {
        ((blank().or_not())
            .ignore_then(
                select_expression(pattern.clone(), inline_placeable.clone())
                    .map(InlinePlaceable::SelectExpression)
                    .or(inline_expression(inline_placeable).map(InlinePlaceable::InlineExpression)),
            )
            .then_ignore(blank().or_not()))
        .delimited_by(just('{'), just('}'))
        .boxed()
    })
    .labelled("inline_placeable")
}

fn block_placeable<'a>(
    pattern: impl Parser<'a, &'a str, Pattern, extra::Err<Simple<'a, char>>> + Clone + 'a,
) -> impl Parser<'a, &'a str, BlockPlaceable, extra::Err<Simple<'a, char>>> {
    blank_block()
        .then(blank_inline().or_not())
        .map(|(bb, bi_opt)| format!("{}{}", bb, bi_opt.map_or(String::default(), String::from)))
        .then(inline_placeable(pattern))
        .map(|(prefix, placeable)| BlockPlaceable::new(prefix, placeable))
        .labelled("block_placeable")
}

fn inline_expression<'a>(
    inline_placeable: impl Parser<'a, &'a str, InlinePlaceable, extra::Err<Simple<'a, char>>>
    + Clone
    + 'a,
) -> impl Parser<'a, &'a str, InlineExpression, extra::Err<Simple<'a, char>>> + Clone + 'a {
    recursive(|inline_expression| {
        choice((
            string_literal().map(InlineExpression::StringLiteral),
            number_literal().map(InlineExpression::NumberLiteral),
            function_reference(inline_expression.clone()).map(InlineExpression::FunctionReference),
            message_reference().map(InlineExpression::MessageReference),
            term_reference(inline_expression.clone()).map(InlineExpression::TermReference),
            variable_reference().map(InlineExpression::VariableReference),
            inline_placeable.map(|ip| InlineExpression::InlinePlaceable(Box::new(ip))),
        ))
        .boxed()
    })
    .labelled("inline_expression")
}

fn string_literal<'a>() -> impl Parser<'a, &'a str, StringLiteral, extra::Err<Simple<'a, char>>> {
    quoted_char()
        .repeated()
        .to_slice()
        .map(String::from)
        .delimited_by(just('"'), just('"'))
        .map(|s| StringLiteral::from(s.as_str()))
        .labelled("string_literal")
}

fn number_literal<'a>() -> impl Parser<'a, &'a str, NumberLiteral, extra::Err<Simple<'a, char>>> {
    (just('-').or_not())
        .then(digits())
        .then(just('.').then(digits()).or_not())
        .map(|((sign, int_part), frac_opt)| {
            let s = format!(
                "{}{}{}",
                sign.map_or_else(String::default, |s| s.to_string()),
                int_part,
                frac_opt.map_or_else(String::default, |(dot, frac)| format!("{dot}{frac}"))
            );
            NumberLiteral::from(s.as_str())
        })
        .labelled("number_literal")
}

fn function_reference<'a>(
    inline_expression: impl Parser<'a, &'a str, InlineExpression, extra::Err<Simple<'a, char>>>
    + Clone
    + 'a,
) -> impl Parser<'a, &'a str, FunctionReference, extra::Err<Simple<'a, char>>> {
    identifier()
        .then(call_arguments(inline_expression))
        .map(|(id, args)| FunctionReference::new(id, args))
        .labelled("function_reference")
}

fn message_reference<'a>()
-> impl Parser<'a, &'a str, MessageReference, extra::Err<Simple<'a, char>>> {
    identifier()
        .then(attribute_accessor().or_not())
        .map(|(id, attr)| MessageReference::new(id, attr))
        .labelled("message_reference")
}

fn term_reference<'a>(
    inline_expression: impl Parser<'a, &'a str, InlineExpression, extra::Err<Simple<'a, char>>>
    + Clone
    + 'a,
) -> impl Parser<'a, &'a str, TermReference, extra::Err<Simple<'a, char>>> {
    just('-')
        .ignore_then(identifier())
        .then(attribute_accessor().or_not())
        .then(call_arguments(inline_expression).or_not())
        .map(|((id, attr), args)| TermReference::new(id, attr, args))
        .labelled("term_reference")
}

fn variable_reference<'a>()
-> impl Parser<'a, &'a str, VariableReference, extra::Err<Simple<'a, char>>> {
    just('$')
        .ignore_then(identifier())
        .map(VariableReference::from)
        .labelled("variable_reference")
}

fn attribute_accessor<'a>()
-> impl Parser<'a, &'a str, AttributeAccessor, extra::Err<Simple<'a, char>>> {
    just('.')
        .ignore_then(identifier())
        .map(AttributeAccessor::from)
        .labelled("attribute_accessor")
}

fn call_arguments<'a>(
    inline_expression: impl Parser<'a, &'a str, InlineExpression, extra::Err<Simple<'a, char>>>
    + Clone
    + 'a,
) -> impl Parser<'a, &'a str, CallArguments, extra::Err<Simple<'a, char>>> {
    (blank().or_not())
        .ignore_then(
            ((blank().or_not())
                .ignore_then(argument_list(inline_expression))
                .then_ignore(blank().or_not()))
            .delimited_by(just('('), just(')')),
        )
        .map(|args| CallArguments::from(args.as_slice()))
        .labelled("call_arguments")
}

fn argument_list<'a>(
    inline_expression: impl Parser<'a, &'a str, InlineExpression, extra::Err<Simple<'a, char>>>
    + Clone
    + 'a,
) -> impl Parser<'a, &'a str, Vec<Argument>, extra::Err<Simple<'a, char>>> {
    argument(inline_expression)
        .separated_by((blank().or_not()).then(just(',')).then(blank().or_not()))
        .collect::<Vec<_>>()
        .labelled("argument_list")
}

fn argument<'a>(
    inline_expression: impl Parser<'a, &'a str, InlineExpression, extra::Err<Simple<'a, char>>>
    + Clone
    + 'a,
) -> impl Parser<'a, &'a str, Argument, extra::Err<Simple<'a, char>>> {
    (named_argument().map(Argument::NamedArgument).boxed())
        .or(inline_expression.map(Argument::InlineExpression).boxed())
        .labelled("argument")
}

fn named_argument<'a>() -> impl Parser<'a, &'a str, NamedArgument, extra::Err<Simple<'a, char>>> {
    identifier()
        .then_ignore(blank().or_not())
        .then_ignore(just(':'))
        .then_ignore(blank().or_not())
        .then(choice((
            string_literal().map(Literal::String),
            number_literal().map(Literal::Number),
        )))
        .map(|(id, lit)| NamedArgument::new(id, lit))
        .labelled("named_argument")
}

fn select_expression<'a>(
    pattern: impl Parser<'a, &'a str, Pattern, extra::Err<Simple<'a, char>>> + Clone + 'a,
    inline_placeable: impl Parser<'a, &'a str, InlinePlaceable, extra::Err<Simple<'a, char>>>
    + Clone
    + 'a,
) -> impl Parser<'a, &'a str, SelectExpression, extra::Err<Simple<'a, char>>> + Clone + 'a {
    inline_expression(inline_placeable)
        .then_ignore(blank().or_not())
        .then_ignore(just("->"))
        .then_ignore(blank_inline().or_not())
        .then(variant_list(pattern))
        .map(|(expr, variants)| SelectExpression::new(expr, variants))
        .boxed()
        .labelled("select_expression")
}

fn variant_list<'a>(
    pattern: impl Parser<'a, &'a str, Pattern, extra::Err<Simple<'a, char>>> + Clone + 'a,
) -> impl Parser<'a, &'a str, VariantList, extra::Err<Simple<'a, char>>> {
    (variant(pattern.clone()).repeated().collect::<Vec<_>>())
        .then(default_variant(pattern.clone()))
        .then(variant(pattern).repeated().collect::<Vec<_>>())
        .then_ignore(line_end())
        .map(|((variants, default), more)| VariantList::new(variants, default, more))
        .labelled("variant_list")
}

fn variant<'a>(
    pattern: impl Parser<'a, &'a str, Pattern, extra::Err<Simple<'a, char>>>,
) -> impl Parser<'a, &'a str, Variant, extra::Err<Simple<'a, char>>> {
    line_end()
        .ignore_then(blank().or_not())
        .ignore_then(variant_key())
        .then_ignore(blank_inline().or_not())
        .then(pattern)
        .map(|(key, pat)| Variant::new(key, pat))
        .labelled("variant")
}

fn default_variant<'a>(
    pattern: impl Parser<'a, &'a str, Pattern, extra::Err<Simple<'a, char>>>,
) -> impl Parser<'a, &'a str, DefaultVariant, extra::Err<Simple<'a, char>>> {
    line_end()
        .ignore_then(blank().or_not())
        .ignore_then(just('*'))
        .ignore_then(variant_key())
        .then_ignore(blank_inline().or_not())
        .then(pattern)
        .map(|(key, pat)| DefaultVariant::new(key, pat))
        .labelled("default_variant")
}

fn variant_key<'a>() -> impl Parser<'a, &'a str, VariantKey, extra::Err<Simple<'a, char>>> {
    ((blank().or_not())
        .ignore_then(
            (number_literal().map(VariantKey::NumberLiteral).boxed())
                .or(identifier().map(VariantKey::Identifier).boxed()),
        )
        .then_ignore(blank().or_not()))
    .delimited_by(just('['), just(']'))
    .labelled("variant_key")
}

fn identifier<'a>() -> impl Parser<'a, &'a str, Identifier, extra::Err<Simple<'a, char>>> {
    (any().filter(|c: &char| c.is_ascii_alphabetic()))
        .then(
            any()
                .filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
                .repeated()
                .collect::<String>(),
        )
        .map(|(head, tail)| format!("{head}{tail}"))
        .map(|s| Identifier::from(s.as_str()))
}

fn any_char<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Simple<'a, char>>> {
    let range = '\u{0000}'..='\u{10ffff}';
    any()
        .filter(move |c| range.contains(c))
        .to_slice()
        .labelled("any_char")
}

fn special_text_char<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Simple<'a, char>>> {
    choice((just("{"), just("}"))).labelled("special_text_char")
}

fn text_char<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Simple<'a, char>>> {
    (special_text_char().or(line_end()))
        .not()
        .ignore_then(any_char())
        .labelled("text_char")
}

fn indented_char<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Simple<'a, char>>> {
    (one_of("[*.").not())
        .ignore_then(text_char())
        .to_slice()
        .labelled("indented_char")
}

fn special_quoted_char<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Simple<'a, char>>> {
    one_of("\"\\").to_slice().labelled("special_quoted_char")
}

fn special_escape<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Simple<'a, char>>> {
    just('\\')
        .ignore_then(special_quoted_char())
        .labelled("special_escape")
}

fn unicode_escape<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Simple<'a, char>>> {
    choice((
        just("\\u").then(text::digits(16).exactly(4)),
        just("\\U").then(text::digits(16).exactly(6)),
    ))
    .to_slice()
    .labelled("unicode escape")
}

fn quoted_char<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Simple<'a, char>>> {
    ((special_quoted_char().or(line_end()))
        .not()
        .ignore_then(any_char()))
    .or(special_escape())
    .or(unicode_escape())
    .to_slice()
}

fn digits<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Simple<'a, char>>> {
    text::digits(10).to_slice().labelled("digits")
}

fn blank_inline<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Simple<'a, char>>> {
    just('\u{0020}').repeated().at_least(1).to_slice()
}

fn line_end<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Simple<'a, char>>> {
    choice((just("\u{000d}\u{000a}"), just("\u{000a}")))
}

fn blank_block<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Simple<'a, char>>> {
    (blank_inline().or_not().then(line_end()))
        .repeated()
        .at_least(1)
        .to_slice()
}

fn blank<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Simple<'a, char>>> {
    choice((blank_inline(), line_end()))
        .repeated()
        .at_least(1)
        .to_slice()
}
