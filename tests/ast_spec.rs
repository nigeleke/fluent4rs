use std::any::{Any, TypeId};

use fluent4rs::prelude::*;
use pretty_assertions::assert_eq;

#[derive(Default)]
struct TestVisitor;

impl Visitor for TestVisitor {
    fn visit_attribute(&mut self, node: &Attribute) {
        assert_eq!(&node.identifier().type_id(), &TypeId::of::<Identifier>());
        assert_eq!(&node.identifier_name().type_id(), &TypeId::of::<String>());
        assert_eq!(&node.pattern().type_id(), &TypeId::of::<Pattern>());
    }

    fn visit_attribute_accessor(&mut self, node: &AttributeAccessor) {
        assert_eq!(&node.identifier().type_id(), &TypeId::of::<Identifier>());
    }

    fn visit_call_arguments(&mut self, node: &CallArguments) {
        assert_eq!(&node.arguments().type_id(), &TypeId::of::<[Argument]>());
    }

    fn visit_default_variant(&mut self, node: &DefaultVariant) {
        assert_eq!(&node.variant_key().type_id(), &TypeId::of::<VariantKey>());
        assert_eq!(&node.pattern().type_id(), &TypeId::of::<Pattern>());
    }

    fn visit_function_reference(&mut self, node: &FunctionReference) {
        assert_eq!(&node.identifier().type_id(), &TypeId::of::<Identifier>());
        assert_eq!(
            &node.call_arguments().type_id(),
            &TypeId::of::<CallArguments>()
        );
    }

    fn visit_message_reference(&mut self, node: &MessageReference) {
        assert_eq!(&node.identifier().type_id(), &TypeId::of::<Identifier>());
        assert_eq!(&node.identifier_name(), &node.identifier().to_string());
        if let Some(attribute_accessor) = node.attribute_accessor() {
            assert_eq!(
                &attribute_accessor.type_id(),
                &TypeId::of::<AttributeAccessor>()
            );
        }
    }

    fn visit_named_argument(&mut self, node: &NamedArgument) {
        assert_eq!(&node.identifier().type_id(), &TypeId::of::<Identifier>());
    }

    fn visit_pattern(&mut self, node: &Pattern) {
        assert_eq!(
            &node.pattern_elements().type_id(),
            &TypeId::of::<[PatternElement]>()
        );
    }

    fn visit_select_expression(&mut self, node: &SelectExpression) {
        assert_eq!(
            &node.inline_expression().type_id(),
            &TypeId::of::<InlineExpression>()
        );
    }

    fn visit_term_reference(&mut self, node: &TermReference) {
        assert_eq!(&node.identifier().type_id(), &TypeId::of::<Identifier>());
        assert_eq!(
            &node.identifier_name(),
            &format!("-{}", node.identifier().to_string())
        );
        if let Some(attribute_accessor) = node.attribute_accessor() {
            assert_eq!(
                &attribute_accessor.type_id(),
                &TypeId::of::<AttributeAccessor>()
            );
        }
        if let Some(call_arguments) = node.call_arguments() {
            assert_eq!(&call_arguments.type_id(), &TypeId::of::<CallArguments>());
        }
    }

    fn visit_variable_reference(&mut self, node: &VariableReference) {
        assert_eq!(&node.identifier().type_id(), &TypeId::of::<Identifier>());
    }

    fn visit_variant(&mut self, node: &Variant) {
        assert_eq!(&node.variant_key().type_id(), &TypeId::of::<VariantKey>());
        assert_eq!(&node.pattern().type_id(), &TypeId::of::<Pattern>());
    }
}

// These tests ensure methods that are made public in the linrary, but not accessed in
// core library code, remain accessible..

#[test]
fn attribute_public_methods() {
    let ftl = include_str!("data/full_grammar_example.ftl");
    let ast = Parser::parse(ftl).unwrap();

    let mut visitor = TestVisitor::default();
    Walker::walk(&ast, &mut visitor);
}
