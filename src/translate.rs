use std::rc::Rc;

use hexagon::{
    iota::hex_casting::null::NullIota,
    parser::{AstNode, Location, OpName, OpValue},
};
use rhai::Shared;

use crate::{flatten_ast::FlatNode, translate_ops::translate_op};

pub fn translate_flattened_ast(ast: Vec<FlatNode>) -> Vec<AstNode> {
    let mut translated_ast = vec![];

    for node in ast {
        match node {
            FlatNode::Op(op, position) => translated_ast.append(&mut translate_op(
                op,
                Location::Line(position.line().unwrap(), position.position().unwrap()),
            )),

            FlatNode::NumberLiteral(num, position) => translated_ast.push(AstNode::Op {
                location: Location::Line(position.line().unwrap(), position.position().unwrap()),
                name: OpName::IntroEmbed,
                arg: Some(OpValue::Iota(Rc::new(num))),
            }),
            FlatNode::BooleanLiteral(bool, position) => translated_ast.push(AstNode::Op {
                location: Location::Line(position.line().unwrap(), position.position().unwrap()),
                name: OpName::IntroEmbed,
                arg: Some(OpValue::Iota(Rc::new(bool))),
            }),
            FlatNode::StringLiteral(string, position) => translated_ast.push(AstNode::Op {
                location: Location::Line(position.line().unwrap(), position.position().unwrap()),
                name: OpName::IntroEmbed,
                arg: Some(OpValue::Iota(Rc::new(string))),
            }),
            FlatNode::Unit(position) => translated_ast.push(AstNode::Op {
                location: Location::Line(position.line().unwrap(), position.position().unwrap()),
                name: OpName::IntroEmbed,
                arg: Some(OpValue::Iota(Rc::new(NullIota))),
            }),
        }
    }

    return translated_ast;
}
