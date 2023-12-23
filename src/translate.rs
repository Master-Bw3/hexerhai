use std::rc::Rc;

use hexagon::{
    iota::hex_casting::null::NullIota,
    parser::{AstNode, Location, OpName, OpValue},
};
use rhai::Position;

use crate::{
    flatten_ast::FlatNode,
    translate_dynamic::{self, translate_dynamic_to_iota},
    translate_ops::translate_op,
};

pub fn translate_flattened_ast(ast: Vec<FlatNode>) -> Vec<AstNode> {
    let mut translated_ast = vec![];

    for node in ast {
        translated_ast.append(&mut translate_node(node));
    }

    return translated_ast;
}

pub fn translate_node(node: FlatNode) -> Vec<AstNode> {
    let mut translated = vec![];

    match node {
        FlatNode::Op(op, position) => {
            translated.append(&mut translate_op(op, position_to_location(position)))
        }

        FlatNode::NumberLiteral(num, position) => translated.push(AstNode::Op {
            location: position_to_location(position),
            name: OpName::IntroEmbed,
            arg: Some(OpValue::Iota(Rc::new(num))),
        }),
        FlatNode::BooleanLiteral(bool, position) => translated.push(AstNode::Op {
            location: position_to_location(position),
            name: OpName::IntroEmbed,
            arg: Some(OpValue::Iota(Rc::new(bool))),
        }),
        FlatNode::StringLiteral(string, position) => translated.push(AstNode::Op {
            location: position_to_location(position),
            name: OpName::IntroEmbed,
            arg: Some(OpValue::Iota(Rc::new(string))),
        }),
        FlatNode::Unit(position) => translated.push(AstNode::Op {
            location: position_to_location(position),
            name: OpName::IntroEmbed,
            arg: Some(OpValue::Iota(Rc::new(NullIota))),
        }),
        FlatNode::DynamicConstant(val, position) => translated.push(AstNode::Op {
            location: position_to_location(position),
            name: OpName::IntroEmbed,
            arg: Some(OpValue::Iota(translate_dynamic_to_iota(val, position))),
        }),
    };

    return translated;
}

pub fn position_to_location(position: Position) -> Location {
    Location::Line(position.line().unwrap(), position.position().unwrap())
}
