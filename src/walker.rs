//! The [Walker] enables the [Resource] returned by the [Parser](crate::parser::Parser)
//! to be traversed. This functionality requires the `walker` feature.
//!
//! The default [Visitor] implementation is a no-op, but selecting the `trace`
//! feature prints the AST to stderr as the tree is traverse. Selecting the
//! `trace` feature will also include the `walker` feature.
use super::ast::*;

/// [Visitor] enables key AST nodes to be visited. Implement [Visitor] and override
/// required methods to allow them to be visited during a walk of the AST.
/// If there's a build with the `trace` feature, the default [Visitor] will output
/// each node to stderr.
pub trait Visitor {
    /// Called before traversing each type instance.
    fn enter(&mut self) {}

    /// Called on exit from traversing each type instance.
    fn exit(&mut self) {}

    /// Visit a top-level resource (the root of the AST).
    fn visit_resource(&mut self, _resource: &Resource) {
        #[cfg(feature = "trace")]
        eprintln!("resource: {_resource}");
    }

    /// Visit any entry (message, term, comment, junk, etc.).
    fn visit_entry(&mut self, _entry: &Entry) {
        #[cfg(feature = "trace")]
        eprintln!("entry: {_entry}");
    }

    /// Visit a message definition.
    fn visit_message(&mut self, _message: &Message) {
        #[cfg(feature = "trace")]
        eprintln!("message: {_message}");
    }

    /// Visit a term definition.
    fn visit_term(&mut self, _term: &Term) {
        #[cfg(feature = "trace")]
        eprintln!("term: {_term}");
    }

    /// Visit a standalone comment line (resource, group, or section comment).
    fn visit_comment_line(&mut self, _comment_line: &CommentLine) {
        #[cfg(feature = "trace")]
        eprintln!("comment_line: {_comment_line}");
    }

    /// Visit junk (unparseable content preserved for round-tripping).
    fn visit_junk(&mut self, _junk: &Junk) {
        #[cfg(feature = "trace")]
        eprintln!("junk: {_junk}");
    }

    /// Visit an attribute attached to a message or term.
    fn visit_attribute(&mut self, _attribute: &Attribute) {
        #[cfg(feature = "trace")]
        eprintln!("attribute: {_attribute}");
    }

    /// Visit a pattern (sequence of text and placeables).
    fn visit_pattern(&mut self, _pattern: &Pattern) {
        #[cfg(feature = "trace")]
        eprintln!("pattern: {_pattern}");
    }

    /// Visit a single element within a pattern (text or placeable).
    fn visit_pattern_element(&mut self, _element: &PatternElement) {
        #[cfg(feature = "trace")]
        eprintln!("pattern_element: {_element}");
    }

    /// Visit an inline expression (inside `{}` placeables).
    fn visit_inline_expression(&mut self, _expression: &InlineExpression) {
        #[cfg(feature = "trace")]
        eprintln!("inline_expression: {_expression}");
    }

    /// Visit a string literal.
    fn visit_string_literal(&mut self, _literal: &StringLiteral) {
        #[cfg(feature = "trace")]
        eprintln!("string_literal: {_literal}");
    }

    /// Visit a number literal.
    fn visit_number_literal(&mut self, _literal: &NumberLiteral) {
        #[cfg(feature = "trace")]
        eprintln!("number_literal: {_literal}");
    }

    /// Visit a function reference (e.g., `NUMBER($num)`).
    fn visit_function_reference(&mut self, _reference: &FunctionReference) {
        #[cfg(feature = "trace")]
        eprintln!("function_reference: {_reference}");
    }

    /// Visit a message reference (e.g., `{ msg }` or `{ msg.attr }`).
    fn visit_message_reference(&mut self, _reference: &MessageReference) {
        #[cfg(feature = "trace")]
        eprintln!("message_reference: {_reference}");
    }

    /// Visit a term reference (e.g., `{-term}` or `{-term(attr: value)}`).
    fn visit_term_reference(&mut self, _reference: &TermReference) {
        #[cfg(feature = "trace")]
        eprintln!("term_reference: {_reference}");
    }

    /// Visit a variable reference (e.g., `$var`).
    fn visit_variable_reference(&mut self, _reference: &VariableReference) {
        #[cfg(feature = "trace")]
        eprintln!("variable_reference: {_reference}");
    }

    /// Visit an attribute accessor (`.attr` part).
    fn visit_attribute_accessor(&mut self, _accessor: &AttributeAccessor) {
        #[cfg(feature = "trace")]
        eprintln!("attribute_accessor: {_accessor}");
    }

    /// Visit call arguments (positional and named).
    fn visit_call_arguments(&mut self, _arguments: &CallArguments) {
        #[cfg(feature = "trace")]
        eprintln!("call_arguments: {_arguments}");
    }

    /// Visit a positional or named argument in a function/term call.
    fn visit_argument(&mut self, _argument: &Argument) {
        #[cfg(feature = "trace")]
        eprintln!("argument: {_argument}");
    }

    /// Visit a named (keyword) argument.
    fn visit_named_argument(&mut self, _argument: &NamedArgument) {
        #[cfg(feature = "trace")]
        eprintln!("named_argument {_argument}");
    }

    /// Visit a select expression (plural/other category selection).
    fn visit_select_expression(&mut self, _expression: &SelectExpression) {
        #[cfg(feature = "trace")]
        eprintln!("select_expression: {_expression}");
    }

    /// Visit a regular variant in a select expression.
    fn visit_variant(&mut self, _variant: &Variant) {
        #[cfg(feature = "trace")]
        eprintln!("variant: {_variant}");
    }

    /// Visit a variant key (the selector value).
    fn visit_variant_key(&mut self, _variant_key: &VariantKey) {
        #[cfg(feature = "trace")]
        eprintln!("variant_key: {_variant_key}");
    }

    /// Visit the default variant (marked with `*`).
    fn visit_default_variant(&mut self, _variant: &DefaultVariant) {
        #[cfg(feature = "trace")]
        eprintln!("default_variant: {_variant}");
    }

    /// Visit an identifier (names of messages, terms, variables, etc.).
    fn visit_identifier(&mut self, _identifier: &Identifier) {
        #[cfg(feature = "trace")]
        eprintln!("identifier: {_identifier}");
    }
}

/// AST nodes implement walkable, so any can be re-walked and / ot re-visited.
/// It is not expected that this trait will be implemented by an end user.
pub trait Walkable {
    fn walk(&self, visitor: &mut dyn Visitor);
}

/// A [Walker] enables an AST node to be walked with the `Walk::walk(node, visitor)`
/// function.
pub struct Walker;

impl Walker {
    /// Walk the AST.
    /// Each AST object is [Walkable], however, commonly the initial
    /// ojbect will be the parsed [Resource].
    pub fn walk(walkable: &dyn Walkable, visitor: &mut dyn Visitor) {
        visitor.enter();
        walkable.walk(visitor);
        visitor.exit();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::Parser;

    use pretty_assertions::assert_eq;
    use pretty_assertions::assert_ne;

    use std::collections::HashMap;

    #[derive(Default)]
    struct TestVisitor {
        enters: usize,
        exits: usize,
        counts: HashMap<String, usize>,
    }

    impl TestVisitor {
        fn bump(&mut self, visit: &str) {
            let count: usize = *self.counts.get(visit).unwrap_or(&0_usize);
            self.counts.insert(visit.into(), count + 1);
        }

        fn assert_expected(&self) {
            assert_ne!(self.enters, 0);
            assert_eq!(self.enters, self.exits);

            assert_ne!(*self.counts.get("visit_resource").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_entry").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_message").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_term").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_comment_line").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_attribute").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_pattern").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_pattern_element").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_inline_expression").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_string_literal").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_number_literal").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_function_reference").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_message_reference").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_term_reference").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_variable_reference").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_attribute_accessor").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_call_arguments").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_argument").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_named_argument").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_select_expression").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_variant").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_variant_key").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_default_variant").unwrap(), 0);
            assert_ne!(*self.counts.get("visit_identifier").unwrap(), 0);
        }

        fn assert_junk(&self, expected: Option<usize>) {
            assert_eq!(self.counts.get("visit_junk"), expected.as_ref());
        }
    }

    impl Visitor for TestVisitor {
        fn enter(&mut self) {
            self.enters += 1;
        }

        fn exit(&mut self) {
            self.exits += 1;
        }

        fn visit_resource(&mut self, _resource: &Resource) {
            self.bump("visit_resource");
        }

        fn visit_entry(&mut self, _entry: &Entry) {
            self.bump("visit_entry");
        }

        fn visit_message(&mut self, _message: &Message) {
            self.bump("visit_message");
        }

        fn visit_term(&mut self, _term: &Term) {
            self.bump("visit_term");
        }

        fn visit_comment_line(&mut self, _comment_line: &CommentLine) {
            self.bump("visit_comment_line");
        }

        fn visit_junk(&mut self, _junk: &Junk) {
            self.bump("visit_junk");
        }

        fn visit_attribute(&mut self, _attribute: &Attribute) {
            self.bump("visit_attribute");
        }

        fn visit_pattern(&mut self, _pattern: &Pattern) {
            self.bump("visit_pattern");
        }

        fn visit_pattern_element(&mut self, _element: &PatternElement) {
            self.bump("visit_pattern_element");
        }

        fn visit_inline_expression(&mut self, _expression: &InlineExpression) {
            self.bump("visit_inline_expression");
        }

        fn visit_string_literal(&mut self, _literal: &StringLiteral) {
            self.bump("visit_string_literal");
        }

        fn visit_number_literal(&mut self, _literal: &NumberLiteral) {
            self.bump("visit_number_literal");
        }

        fn visit_function_reference(&mut self, _reference: &FunctionReference) {
            self.bump("visit_function_reference");
        }

        fn visit_message_reference(&mut self, _reference: &MessageReference) {
            self.bump("visit_message_reference");
        }

        fn visit_term_reference(&mut self, _reference: &TermReference) {
            self.bump("visit_term_reference");
        }

        fn visit_variable_reference(&mut self, _reference: &VariableReference) {
            self.bump("visit_variable_reference");
        }

        fn visit_attribute_accessor(&mut self, _accessor: &AttributeAccessor) {
            self.bump("visit_attribute_accessor");
        }

        fn visit_call_arguments(&mut self, _arguments: &CallArguments) {
            self.bump("visit_call_arguments");
        }

        fn visit_argument(&mut self, _argument: &Argument) {
            self.bump("visit_argument");
        }

        fn visit_named_argument(&mut self, _argument: &NamedArgument) {
            self.bump("visit_named_argument");
        }

        fn visit_select_expression(&mut self, _expression: &SelectExpression) {
            self.bump("visit_select_expression");
        }

        fn visit_variant(&mut self, _variant: &Variant) {
            self.bump("visit_variant");
        }

        fn visit_variant_key(&mut self, _variant_key: &VariantKey) {
            self.bump("visit_variant_key");
        }

        fn visit_default_variant(&mut self, _variant: &DefaultVariant) {
            self.bump("visit_default_variant");
        }

        fn visit_identifier(&mut self, _identifier: &Identifier) {
            self.bump("visit_identifier");
        }
    }

    #[test]
    fn walker_will_visit_grammar_nodes() {
        let ftl = include_str!("../tests/data/full_grammar_example.ftl");
        let ast = Parser::parse(ftl).unwrap();

        let mut visitor = TestVisitor::default();
        Walker::walk(&ast, &mut visitor);
        visitor.assert_expected();
        visitor.assert_junk(None);
    }

    #[test]
    fn walker_will_visit_junk_nodes() {
        let ftl = r#"asdhj asdasdkjhk { &&*$%$% }
            dfsdfjh jhksdfh *($(*%&$&
"#;
        let ast = Parser::parse_with_junk(ftl).unwrap();

        let mut visitor = TestVisitor::default();
        Walker::walk(&ast, &mut visitor);
        visitor.assert_junk(Some(1));
    }

    struct TestDefaultVisitor;
    impl Visitor for TestDefaultVisitor {}

    #[test]
    fn default_visitor_will_be_visited() {
        let ftl = include_str!("../tests/data/full_grammar_example.ftl");
        let ast = Parser::parse(ftl).unwrap();

        let mut visitor = TestDefaultVisitor;
        Walker::walk(&ast, &mut visitor);
        // Should not panic; no other assertions necessary.
    }

    #[test]
    fn default_visitor_will_be_visited_with_junk() {
        let ftl = r#"asdhj asdasdkjhk { &&*$%$% }
            dfsdfjh jhksdfh *($(*%&$&
"#;
        let ast = Parser::parse_with_junk(ftl).unwrap();

        let mut visitor = TestDefaultVisitor;
        Walker::walk(&ast, &mut visitor);
        // Should not panic; no other assertions necessary.
    }
}
