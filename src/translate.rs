use std::rc::Rc;

use hexagon::{parser::{AstNode, Location, OpName, OpValue}, iota::hex_casting::null::NullIota};
use rhai::Shared;

use crate::flatten_ast::FlatNode;

pub fn translate_flattened_ast(ast: Vec<FlatNode>) {
    let mut translated_ast = vec![];

    for node in ast {
        match node {
            FlatNode::Op(op, position) => todo!(),

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
}
