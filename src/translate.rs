use std::rc::Rc;

use hexagon::{
    iota::hex_casting::null::NullIota,
    parser::{AstNode, Location, OpName, OpValue},
};
use rhai::Position;

use crate::{
    flatten_ast::FlatNode, translate_dynamic::translate_dynamic_to_iota,
    translate_ops::translate_op,
};

pub fn translate_flattened_ast(ast: Vec<FlatNode>) -> Vec<AstNode> {
    let mut translated_ast = vec![];

    for node in ast {
        translated_ast.append(&mut translate_node(node));
    }

    println!("translated:\n{:?}", translated_ast);

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
        FlatNode::IfBlock {
            condition,
            succeed,
            fail,
            position,
        } => translated.push(translate_if(condition, succeed, fail, position)),
        FlatNode::WhileBlock {
            do_while,
            condition,
            block,
            position,
        } => translated.push(translate_while(do_while, condition, block, position)),
    };

    return translated;
}

fn translate_if(
    condition: Vec<FlatNode>,
    succeed: Vec<FlatNode>,
    fail: Option<Vec<FlatNode>>,
    position: Position,
) -> AstNode {
    AstNode::IfBlock {
        condition: Box::new(AstNode::Block {
            external: false,
            nodes: condition.into_iter().flat_map(translate_node).collect(),
        }),
        succeed: Box::new(AstNode::Block {
            external: false,
            nodes: succeed.into_iter().flat_map(translate_node).collect(),
        }),
        fail: fail.map(|f| {
            Box::new(AstNode::Block {
                external: false,
                nodes: f.into_iter().flat_map(translate_node).collect(),
            })
        }),
        location: position_to_location(position),
    }
}

fn translate_while(do_while: bool, condition: Vec<FlatNode>, block: Vec<FlatNode>, position: Position) -> AstNode {
    AstNode::WhileBlock {
        do_while,
        condition: Box::new(AstNode::Block {
            external: false,
            nodes: condition.into_iter().flat_map(translate_node).collect(),
        }),
        block: Box::new(AstNode::Block {
            external: false,
            nodes: block.into_iter().flat_map(translate_node).collect(),
        }),
        location: position_to_location(position),
    }
}

pub fn position_to_location(position: Position) -> Location {
    Location::Line(position.line().unwrap(), position.position().unwrap())
}
