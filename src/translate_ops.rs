use std::rc::Rc;

use hexagon::parser::{ActionValue, AstNode, Location, OpName};

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

#[rustfmt::skip]
fn translate_fn_call(fn_name: String, location: Location) -> Vec<AstNode> {
    match fn_name.as_str() {

        //operators
        "==" => vec![AstNode::Action { location, name: "equals".to_string(), value: None }],
        "!=" => vec![AstNode::Action { location, name: "not_equals".to_string(), value: None }],
        ">" => vec![AstNode::Action { location, name: "greater".to_string(), value: None }],
        "<" => vec![AstNode::Action { location, name: "less".to_string(), value: None }],
        ">=" => vec![AstNode::Action { location, name: "greater_eq".to_string(), value: None }],
        "<=" => vec![AstNode::Action { location, name: "less_eq".to_string(), value: None }],
       
        "!" => vec![AstNode::Action { location, name: "not".to_string(), value: None }],
        "&" => translate_op_and(location),
        "|" => translate_op_or(location),
        "||" => todo!(),
        "&&" => todo!(),
        "??" => todo!(),

        "+" => vec![AstNode::Action { location, name: "add".to_string(), value: None }],
        "-" => vec![AstNode::Action { location, name: "subtract".to_string(), value: None }],
        "*" => vec![AstNode::Action { location, name: "mul_dot".to_string(), value: None }],
        "/" => vec![AstNode::Action { location, name: "div_cross".to_string(), value: None }],
        "%" => vec![AstNode::Action { location, name: "modulo".to_string(), value: None }],
        "**" => vec![AstNode::Action { location, name: "pow_proj".to_string(), value: None }],
        "^" => vec![AstNode::Action { location, name: "xor_bit".to_string(), value: None }],

        "<<" => todo!(),
        ">>" => todo!(),

        ".." => translate_op_range(location),
        "..=" => translate_op_range_inclusive(location),

        "in" => translate_op_in(location),

        //builtin functions
        "print" => translate_op_print(location),

        //TODO: handle user-defined functions
        _ => vec![AstNode::Action { location, name: fn_name, value: None }],
    }

}

#[rustfmt::skip]
fn translate_op_in(location: Location) -> Vec<AstNode> {
    let mut actions = vec![];

    actions.push(AstNode::Action { location, name: "index_of".to_string(), value: None });
    actions.push(AstNode::Action { location, name: "number".to_string(), value: Some(ActionValue::Iota(Rc::new(0.0))) });
    actions.push(AstNode::Action { location, name: "greater_eq".to_string(), value: None });


    return actions;
}

#[rustfmt::skip]
fn translate_op_and(location: Location) -> Vec<AstNode> {
    let mut actions = vec![];

    //check if 2nd argument is a boolean
    actions.push(AstNode::Action { location, name: "duplicate".to_string(), value: None });
    actions.push(AstNode::Action { location, name: "duplicate".to_string(), value: None });
    actions.push(AstNode::Action { location, name: "const/true".to_string(), value: None });
    actions.push(AstNode::Action { location, name: "equals".to_string(), value: None });
    actions.push(AstNode::Action { location, name: "swap".to_string(), value: None });
    actions.push(AstNode::Action { location, name: "const/false".to_string(), value: None });
    actions.push(AstNode::Action { location, name: "equals".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "or".to_string(), value:None });

    //if it is, use boolean operator. otherwise, use bitwise operator
    actions.push(AstNode::Action { location, name: "open_paren".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "and".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "and_bit".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "close_paren".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "splat".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "if".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "eval".to_string(), value:None });

    return actions;
}

#[rustfmt::skip]
fn translate_op_or(location: Location) -> Vec<AstNode> {
    let mut actions = vec![];

    //check if 2nd argument is a boolean
    actions.push(AstNode::Action { location, name: "duplicate".to_string(), value: None });
    actions.push(AstNode::Action { location, name: "duplicate".to_string(), value: None });
    actions.push(AstNode::Action { location, name: "const/true".to_string(), value: None });
    actions.push(AstNode::Action { location, name: "equals".to_string(), value: None });
    actions.push(AstNode::Action { location, name: "swap".to_string(), value: None });
    actions.push(AstNode::Action { location, name: "const/false".to_string(), value: None });
    actions.push(AstNode::Action { location, name: "equals".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "or".to_string(), value:None });

    //if it is, use boolean operator. otherwise, use bitwise operator
    actions.push(AstNode::Action { location, name: "open_paren".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "and".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "and_bit".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "close_paren".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "splat".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "if".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "eval".to_string(), value:None });

    return actions;
}

#[rustfmt::skip]
fn translate_op_range_inclusive(location: Location) -> Vec<AstNode> {
    let mut actions = vec![];
    
    actions.push(AstNode::Action { location, name: "number".to_string(), value: Some(ActionValue::Iota(Rc::new(1.0))) });
    actions.push(AstNode::Action { location, name: "add".to_string(), value: None });

    actions.append(&mut translate_op_range(location));

    return actions;
}

#[rustfmt::skip]
fn translate_op_range(location: Location) -> Vec<AstNode> {
    let mut actions = vec![];

    //Counter's Queue
    actions.push(AstNode::Action { location, name: "open_paren".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "swap".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "duplicate_n".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "close_paren".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "open_paren".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "duplicate".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "list_size".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "append".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "close_paren".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "singleton".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "for_each".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "mask".to_string(), value: Some(ActionValue::Bookkeeper("v-".to_string())) });
    actions.push(AstNode::Action { location, name: "open_paren".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "splat".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "close_paren".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "swap".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "for_each".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "empty_list".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "swap".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "eval".to_string(), value:None });

    return actions;
}

#[rustfmt::skip]
fn translate_op_print(location: Location) -> Vec<AstNode> {
    let mut actions = vec![];

    actions.push(AstNode::Action { location, name: "print".to_string(), value:None });
    actions.push(AstNode::Action { location, name: "mask".to_string(), value: Some(ActionValue::Bookkeeper("v".to_string())) });


    return actions;
}
