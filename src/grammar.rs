//! Derived from [Project Fluent](https://github.com/projectfluent/) [fluent.ebnf](https://github.com/projectfluent/fluent/blob/master/spec/fluent.ebnf)

use crate::ast::prelude::*;

use pom::utf8::*;

/* An FTL file defines a Resource consisting of Entries. */
// Resource            ::= (Entry | blank_block | Junk)*
pub fn resource<'a>() -> Parser<'a, Resource> {
    (entry().map(ResourceItem::Entry)
        | blank_block().map(ResourceItem::BlankBlock)
        | junk().map(ResourceItem::Junk))
    .repeat(0..)
    .map(Resource::from)
}

/* Entries are the main building blocks of Fluent. They define translations and
 * contextual and semantic information about the translations. During the AST
 * construction, adjacent comment lines of the same comment type (defined by
 * the number of #) are joined together. Single-# comments directly preceding
 * Messages and Terms are attached to the Message or Term and are not
 * standalone Entries. */
// Entry               ::= (Message line_end)
//                       | (Term line_end)
//                       | CommentLine
fn entry<'a>() -> Parser<'a, Entry> {
    (message() + line_end()).map(|(m, _)| Entry::Message(m))
        | (term() + line_end()).map(|(t, _)| Entry::Term(t))
        | (comment_line()).map(Entry::CommentLine)
}

// Message             ::= Identifier blank_inline? "=" blank_inline? ((Pattern Attribute*) | (Attribute+))
fn message<'a>() -> Parser<'a, Message> {
    (identifier()
        + blank_inline().opt()
        + sym('=')
        + blank_inline().opt()
        + ((pattern() + attribute().repeat(0..)).map(|(p, a)| MessageAttributes::Patterned(p, a))
            | attribute().repeat(1..).map(MessageAttributes::Plain)))
    .map(|((((i, _), _), _), a)| Message::new(i, a))
}

// Term                ::= "-" Identifier blank_inline? "=" blank_inline? Pattern Attribute*
fn term<'a>() -> Parser<'a, Term> {
    (sym('-')
        + identifier()
        + blank_inline().opt()
        + sym('=')
        + blank_inline().opt()
        + pattern()
        + attribute().repeat(0..))
    .map(|((((((_, i), _), _), _), p), a)| Term::new(i, p, a))
}

/* Adjacent comment lines of the same comment type are joined together during
 * the AST construction. */
// CommentLine         ::= ("###" | "##" | "#") ("\u0020" comment_char*)? line_end
fn comment_line<'a>() -> Parser<'a, CommentLine> {
    ((seq("###") | seq("##") | seq("#"))
        + (sym('\u{0020}') + comment_char().repeat(0..)).opt()
        + line_end())
    .map(|((lead, space_comment), _)| {
        let comment = space_comment.map(|(_, c)| c.join(""));
        CommentLine::new(lead.into(), comment)
    })
}

// comment_char        ::= any_char - line_end
fn comment_char<'a>() -> Parser<'a, String> {
    (!line_end() * any()).map(String::from)
}

/* Junk represents unparsed content.
 *
 * Junk is parsed line-by-line until a line is found which looks like it might
 * be a beginning of a new message, term, or a comment. Any whitespace
 * following a broken Entry is also considered part of Junk.
 */
// Junk                ::= junk_line (junk_line - "#" - "-" - [a-zA-Z])*
fn junk<'a>() -> Parser<'a, Junk> {
    (junk_line()
        + (junk_line() - sym('#') - sym('-') - is_a(|c| c.is_ascii_alphabetic())).repeat(0..))
    .map(|(head, mut tail)| {
        let mut junk = Vec::from([head]);
        junk.append(tail.as_mut());
        Junk::from(junk.as_slice())
    })
}

// junk_line           ::= /[^\n]*/ ("\u000A" | EOF)
fn junk_line<'a>() -> Parser<'a, String> {
    (none_of("\n").repeat(0..) + (sym('\u{000a}')))
        .collect()
        .map(String::from)
}

/* Attributes of Messages and Terms. */
// Attribute           ::= line_end blank? "." Identifier blank_inline? "=" blank_inline? Pattern
fn attribute<'a>() -> Parser<'a, Attribute> {
    (line_end()
        + blank().opt()
        + sym('.')
        + identifier()
        + blank_inline().opt()
        + sym('=')
        + blank_inline().opt()
        + pattern())
    .map(|(((((((_, _), _), i), _), _), _), p)| Attribute::new(i, p))
}

/* Patterns are values of Messages, Terms, Attributes and Variants. */
// Pattern             ::= PatternElement+
fn pattern<'a>() -> Parser<'a, Pattern> {
    (pattern_element().repeat(1..)).map(|pes| Pattern::from(pes.as_slice()))
}

/* TextElement and Placeable can occur inline or as block.
 * Text needs to be indented and start with a non-special character.
 * Placeables can start at the beginning of the line or be indented.
 * Adjacent TextElements are joined in AST creation. */
// PatternElement      ::= inline_text
//                       | block_text
//                       | inline_placeable
//                       | block_placeable
fn pattern_element<'a>() -> Parser<'a, PatternElement> {
    inline_text().map(PatternElement::InlineText)
        | block_text().map(PatternElement::BlockText)
        | call(inline_placeable).map(PatternElement::InlinePlaceable)
        | block_placeable().map(PatternElement::BlockPlaceable)
}

// inline_text         ::= text_char+
fn inline_text<'a>() -> Parser<'a, InlineText> {
    (text_char().repeat(1..)).collect().map(InlineText::from)
}

// block_text          ::= blank_block blank_inline indented_char inline_text?
fn block_text<'a>() -> Parser<'a, BlockText> {
    (blank_block() + blank_inline() + indented_char() + inline_text().opt())
        .collect()
        .map(BlockText::from)
}

// inline_placeable    ::= "{" blank? (SelectExpression | InlineExpression) blank? "}"
fn inline_placeable<'a>() -> Parser<'a, InlinePlaceable> {
    (sym('{')
        + blank().opt()
        + (call(select_expression).map(InlinePlaceable::SelectExpression)
            | call(inline_expression).map(InlinePlaceable::InlineExpression))
        + blank().opt()
        + sym('}'))
    .map(|((((_, _), ip), _), _)| ip)
}

// block_placeable     ::= blank_block blank_inline? inline_placeable
fn block_placeable<'a>() -> Parser<'a, BlockPlaceable> {
    ((blank_block() + blank_inline().opt()).collect() + call(inline_placeable))
        .map(|(bb, ip)| BlockPlaceable::new(bb.into(), ip))
}

/* Rules for validating expressions in Placeables and as selectors of
 * SelectExpressions are documented in spec/valid.md and enforced in
 * syntax/abstract.js. */
// InlineExpression    ::= StringLiteral
//                       | NumberLiteral
//                       | FunctionReference
//                       | MessageReference
//                       | TermReference
//                       | VariableReference
//                       | inline_placeable
fn inline_expression<'a>() -> Parser<'a, InlineExpression> {
    string_literal().map(InlineExpression::StringLiteral)
        | number_literal().map(InlineExpression::NumberLiteral)
        | function_reference().map(InlineExpression::FunctionReference)
        | message_reference().map(InlineExpression::MessageReference)
        | term_reference().map(InlineExpression::TermReference)
        | variable_reference().map(InlineExpression::VariableReference)
        | call(inline_placeable).map(|ip| InlineExpression::InlinePlaceable(Box::new(ip)))
}

/* Literals */
// StringLiteral       ::= "\"" quoted_char* "\""
fn string_literal<'a>() -> Parser<'a, StringLiteral> {
    (sym('"') + quoted_char().repeat(0..).map(|cs| cs.join("")) + sym('"'))
        .map(|((_, cs), _)| StringLiteral::from(cs.as_str()))
}

// NumberLiteral       ::= "-"? digits ("." digits)?
fn number_literal<'a>() -> Parser<'a, NumberLiteral> {
    (sym('-').opt() + digits() + (sym('.') + digits()).opt())
        .collect()
        .map(NumberLiteral::from)
}

/* Inline Expressions */
// FunctionReference   ::= Identifier CallArguments
fn function_reference<'a>() -> Parser<'a, FunctionReference> {
    (identifier() + call_arguments()).map(|(i, ca)| FunctionReference::new(i, ca))
}

// MessageReference    ::= Identifier AttributeAccessor?
fn message_reference<'a>() -> Parser<'a, MessageReference> {
    (identifier() + attribute_accessor().opt()).map(|(i, aa)| MessageReference::new(i, aa))
}

// TermReference       ::= "-" Identifier AttributeAccessor? CallArguments?
fn term_reference<'a>() -> Parser<'a, TermReference> {
    (sym('-') + identifier() + attribute_accessor().opt() + call_arguments().opt())
        .map(|(((_, i), aa), ca)| TermReference::new(i, aa, ca))
}

// VariableReference   ::= "$" Identifier
fn variable_reference<'a>() -> Parser<'a, VariableReference> {
    (sym('$') + identifier()).map(|(_, i)| VariableReference::from(i))
}

// AttributeAccessor   ::= "." Identifier
fn attribute_accessor<'a>() -> Parser<'a, AttributeAccessor> {
    (sym('.') + identifier()).map(|(_, i)| AttributeAccessor::from(i))
}

// CallArguments       ::= blank? "(" blank? argument_list blank? ")"
fn call_arguments<'a>() -> Parser<'a, CallArguments> {
    (blank().opt() + sym('(') + blank().opt() + argument_list() + blank().opt() + sym(')'))
        .map(|(((((_, _), _), al), _), _)| CallArguments::from(al.as_slice()))
}

// argument_list       ::= (Argument blank? "," blank?)* Argument?
fn argument_list<'a>() -> Parser<'a, Vec<Argument>> {
    ((argument() + blank().opt() + sym(',') + blank().opt()).repeat(0..) + argument().opt()).map(
        |(args, arg)| {
            let mut args = Vec::from_iter(args.into_iter().map(|(((a, _), _), _)| a));
            if let Some(arg) = arg {
                args.push(arg);
            }
            args
        },
    )
}

// Argument            ::= NamedArgument
//                       | InlineExpression
fn argument<'a>() -> Parser<'a, Argument> {
    named_argument().map(Argument::NamedArgument)
        | call(inline_expression).map(Argument::InlineExpression)
}

// NamedArgument       ::= Identifier blank? ":" blank? (StringLiteral | NumberLiteral)
fn named_argument<'a>() -> Parser<'a, NamedArgument> {
    (identifier()
        + blank().opt()
        + sym(':')
        + blank().opt()
        + (string_literal().map(Literal::from) | number_literal().map(Literal::from)))
    .map(|((((i, _), _), _), l)| NamedArgument::new(i, l))
}

/* Block Expressions */
// SelectExpression    ::= InlineExpression blank? "->" blank_inline? variant_list
fn select_expression<'a>() -> Parser<'a, SelectExpression> {
    (call(inline_expression) + blank().opt() + seq("->") + blank_inline().opt() + variant_list())
        .map(|((((ie, _), _), _), vl)| SelectExpression::new(ie, vl))
}

// variant_list        ::= Variant* DefaultVariant Variant* line_end
fn variant_list<'a>() -> Parser<'a, VariantList> {
    (variant().repeat(0..) + default_variant() + variant().repeat(0..) + line_end())
        .map(|(((va, dv), vz), _)| VariantList::new(va, dv, vz))
}

// Variant             ::= line_end blank? VariantKey blank_inline? Pattern
fn variant<'a>() -> Parser<'a, Variant> {
    (line_end() + blank().opt() + variant_key() + blank_inline().opt() + pattern())
        .map(|((((_, _), vk), _), p)| Variant::new(vk, p))
}

// DefaultVariant      ::= line_end blank? "*" VariantKey blank_inline? Pattern
fn default_variant<'a>() -> Parser<'a, DefaultVariant> {
    (line_end() + blank().opt() + sym('*') + variant_key() + blank_inline().opt() + pattern())
        .map(|(((((_, _), _), vk), _), p)| DefaultVariant::new(vk, p))
}

// VariantKey          ::= "[" blank? (NumberLiteral | Identifier) blank? "]"
fn variant_key<'a>() -> Parser<'a, VariantKey> {
    (sym('[')
        + (number_literal().map(VariantKey::NumberLiteral)
            | identifier().map(VariantKey::Identifier))
        + blank().opt()
        + sym(']'))
    .map(|(((_, vk), _), _)| vk)
}

/* Identifier */
// Identifier          ::= [a-zA-Z] [a-zA-Z0-9_-]*
fn identifier<'a>() -> Parser<'a, Identifier> {
    (is_a(|c| c.is_ascii_alphabetic())
        + is_a(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-').repeat(0..))
    .collect()
    .map(Identifier::from)
}

/* Content Characters
 *
 * Translation content can be written using any Unicode characters. However,
 * some characters are considered special depending on the type of content
 * they're in. See text_char and quoted_char for more information.
 *
 * Some Unicode characters, even if allowed, should be avoided in Fluent
 * resources. See spec/recommendations.md.
 */
// any_char            ::= [\\u{0}-\\u{10FFFF}]
fn any_char<'a>() -> Parser<'a, String> {
    let range = '\u{0000}'..='\u{10ffff}';
    is_a(move |c| range.contains(&c))
        .collect()
        .map(String::from)
}

/* Text elements
 *
 * The primary storage for content are text elements. Text elements are not
 * delimited with quotes and may span multiple lines as long as all lines are
 * indented. The opening brace ({) marks a start of a placeable in the pattern
 * and may not be used in text elements verbatim. Due to the indentation
 * requirement some text characters may not appear as the first character on a
 * new line.
 */
// special_text_char   ::= "{"
//                       | "}"
fn special_text_char<'a>() -> Parser<'a, String> {
    (sym('{') | sym('}')).collect().map(String::from)
}

// text_char           ::= any_char - special_text_char - line_end
fn text_char<'a>() -> Parser<'a, String> {
    (!special_text_char() + !line_end()) * any_char()
}

// indented_char       ::= text_char - "[" - "*" - "."
fn indented_char<'a>() -> Parser<'a, String> {
    none_of("[*.") * text_char()
}

/* String literals
 *
 * For special-purpose content, quoted string literals can be used where text
 * elements are not a good fit. String literals are delimited with double
 * quotes and may not contain line breaks. String literals use the backslash
 * (\) as the escape character. The literal double quote can be inserted via
 * the \" escape sequence. The literal backslash can be inserted with \\. The
 * literal opening brace ({) is allowed in string literals because they may not
 * comprise placeables.
 */
// special_quoted_char ::= "\""
//                       | "\\"
fn special_quoted_char<'a>() -> Parser<'a, String> {
    (sym('"') | sym('\\')).map(|c| c.to_string())
}

// special_escape      ::= "\\" special_quoted_char
fn special_escape<'a>() -> Parser<'a, String> {
    (sym('\\') + special_quoted_char())
        .collect()
        .map(String::from)
}

// unicode_escape      ::= ("\\u" /[0-9a-fA-F]{4}/)
//                       | ("\\U" /[0-9a-fA-F]{6}/)
fn unicode_escape<'a>() -> Parser<'a, String> {
    ((seq("\\u") + is_a(|c| c.is_ascii_hexdigit()).repeat(4))
        | (seq("\\U") + is_a(|c| c.is_ascii_hexdigit()).repeat(6)))
    .collect()
    .map(String::from)
}

// quoted_char         ::= (any_char - special_quoted_char - line_end)
//                       | special_escape
//                       | unicode_escape
fn quoted_char<'a>() -> Parser<'a, String> {
    ((!special_quoted_char() + !line_end()) * any_char()) | special_escape() | unicode_escape()
}

/* Numbers */
// digits              ::= [0-9]+
fn digits<'a>() -> Parser<'a, String> {
    is_a(|c| c.is_ascii_digit())
        .repeat(1..)
        .collect()
        .map(String::from)
}

/* Whitespace */
// blank_inline        ::= "\u0020"+
fn blank_inline<'a>() -> Parser<'a, String> {
    sym('\u{0020}').repeat(1..).map(String::from_iter)
}

// line_end            ::= "\u000D\u000A"
//                       | "\u000A"
//                       | EOF
fn line_end<'a>() -> Parser<'a, String> {
    (sym('\u{000d}') + sym('\u{000a}'))
        .collect()
        .map(String::from)
        | sym('\u{000a}').collect().map(String::from)
}

// blank_block         ::= (blank_inline? line_end)+
fn blank_block<'a>() -> Parser<'a, String> {
    ((blank_inline().opt() + line_end()).repeat(1..))
        .collect()
        .map(String::from)
}

// blank               ::= (blank_inline | line_end)+
fn blank<'a>() -> Parser<'a, String> {
    ((blank_inline() | line_end()).repeat(1..)).map(|bs| bs.join(""))
}
