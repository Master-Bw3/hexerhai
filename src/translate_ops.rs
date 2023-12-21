use std::rc::Rc;

use hexagon::parser::{AstNode, Location, OpName, ActionValue};

use crate::flatten_ast::Op;

//takes rhai operators and compiles them to hex casting actions
pub fn translate_op(op: Op, location: Location) -> Vec<AstNode> {
    match op {
        Op::FnCall(fn_name) => translate_fn_call(fn_name, location),
        Op::Store(var) => vec![AstNode::Op {
            location,
            name: OpName::Store,
            arg: Some(hexagon::parser::OpValue::Var(var)),
        }],
        Op::Push(var) => vec![AstNode::Op {
            location,
            name: OpName::Push,
            arg: Some(hexagon::parser::OpValue::Var(var)),
        }],
    }
}

fn translate_fn_call(fn_name: String, location: Location) -> Vec<AstNode> {
    match fn_name.as_str() {
        "==" => vec![AstNode::Action { location, name: "equals".to_string(), value: None }],
        "!=" => vec![AstNode::Action { location, name: "not_equals".to_string(), value: None }],
        ">" => vec![AstNode::Action { location, name: "greater".to_string(), value: None }],
        "<" => vec![AstNode::Action { location, name: "less".to_string(), value: None }],
        ">=" => vec![AstNode::Action { location, name: "greater_eq".to_string(), value: None }],
        "<=" => vec![AstNode::Action { location, name: "less_eq".to_string(), value: None }],
        "!" => vec![AstNode::Action { location, name: "not".to_string(), value: None }],
        "&" => vec![AstNode::Action { location, name: "and".to_string(), value: None }],
        "|" => vec![AstNode::Action { location, name: "or".to_string(), value: None }],
        "||" => todo!(),
        "&&" => todo!(),

        "in" => translate_op_in(location),



        _ => unreachable!(),
    }

}

fn translate_op_in(location: Location) -> Vec<AstNode> {
    let mut actions = vec![];

    actions.push(AstNode::Action { location, name: "index_of".to_string(), value: None });
    actions.push(AstNode::Action { location, name: "number".to_string(), value: Some(ActionValue::Iota(Rc::new(0.0))) });
    actions.push(AstNode::Action { location, name: "greater_eq".to_string(), value: None });


    return actions;
}

