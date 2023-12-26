use std::ops::Not;

use rhai::{ASTFlags, Dynamic, Expr, FlowControl, FnCallExpr, Ident, Position, Stmt};
use smallvec::SmallVec;

pub fn flatten_statements(ast: &[Stmt]) -> Vec<FlatNode> {
    let mut flattened_ast: Vec<FlatNode> = vec![];

    for statement in ast {
        let mut flattened_ast_statment: Vec<FlatNode> = vec![];

        match statement {
            Stmt::Expr(_) => todo!(),
            Stmt::Var(data, _, _) => {
                flattened_ast_statment.append(&mut flatten_var((data.0.clone(), data.1.clone())))
            }

            Stmt::Assignment(data) => {
                let (variable, position) = if let Expr::Variable(ref var_data, _, pos) = data.1.lhs
                {
                    (var_data.3.clone().to_string(), pos)
                } else {
                    panic!()
                };

                flattened_ast_statment.push(FlatNode::Op(Op::Store(variable.clone()), position));
                if let Some(op) = data.0.clone().get_op_assignment_info().map(|x| x.5) {
                    flattened_ast_statment.push(FlatNode::Op(Op::FnCall(op.to_string()), position));
                    flattened_ast_statment.append(&mut flatten_expression(data.1.rhs.clone()));
                    flattened_ast_statment.push(FlatNode::Op(Op::Push(variable), position));
                } else {
                    flattened_ast_statment.append(&mut flatten_expression(data.1.rhs.clone()));
                }
            }
            Stmt::FnCall(expr, position) => flattened_ast_statment
                .append(&mut flatten_fn_call_expression(*expr.clone(), *position)),
            Stmt::If(data, position) => {
                flattened_ast_statment.append(&mut flatten_if(data, *position))
            }
            Stmt::While(data, position) => {
                flattened_ast_statment.append(&mut flatten_while(true, false, data, *position))
            }
            Stmt::Do(data, flag, position) => {
                if let ASTFlags::NEGATED = *flag {
                    flattened_ast_statment.append(&mut flatten_while(true, true, data, *position))
                } else {
                    flattened_ast_statment.append(&mut flatten_while(true, false, data, *position))
                }
            }

            Stmt::Noop(_) => todo!(),
            Stmt::Switch(_, _) => todo!(),
            Stmt::For(_, _) => todo!(),
            Stmt::Block(_) => todo!(),
            Stmt::TryCatch(_, _) => todo!(),
            Stmt::BreakLoop(_, _, _) => todo!(),
            Stmt::Return(_, _, _) => todo!(),
            Stmt::Import(_, _) => todo!(),
            Stmt::Export(_, _) => todo!(),
            Stmt::Share(_) => todo!(),
            _ => todo!(),
        }

        flattened_ast_statment.reverse();
        flattened_ast.append(&mut flattened_ast_statment)
    }

    return flattened_ast;
}

fn flatten_if(data: &Box<FlowControl>, position: Position) -> Vec<FlatNode> {
    let condition = flatten_expression(data.expr.clone())
        .into_iter()
        .rev()
        .collect::<Vec<_>>();
    let succeed = flatten_statements(data.body.statements());

    let fail = if data.branch.is_empty().not() {
        Some(flatten_statements(data.branch.statements()))
    } else {
        None
    };

    return vec![
        FlatNode::Op(Op::FnCall("eval".to_string()), position),
        FlatNode::IfBlock {
            condition,
            succeed,
            fail,
            position,
        },
    ];
}

fn flatten_while(
    do_while: bool,
    negate_condition: bool,
    data: &Box<FlowControl>,
    position: Position,
) -> Vec<FlatNode> {
    let mut condition = flatten_expression(data.expr.clone())
        .into_iter()
        .rev()
        .collect::<Vec<_>>();

    if negate_condition {
        condition.push(FlatNode::Op(Op::FnCall("not".to_string()), position))
    }

    let block = flatten_statements(data.body.statements());

    return vec![FlatNode::WhileBlock {
        do_while,
        condition: condition,
        block: block,
        position,
    }];
}

fn flatten_var(data: (Ident, Expr)) -> Vec<FlatNode> {
    let mut flattened_ast: Vec<FlatNode> = vec![];

    let identifier = data.0;
    let expression = data.1;

    flattened_ast.push(FlatNode::Op(
        Op::Store(identifier.name.to_string()),
        identifier.pos,
    ));
    flattened_ast.append(&mut flatten_expression(expression));

    return flattened_ast;
}

fn flatten_expression(expression: Expr) -> Vec<FlatNode> {
    let mut flattened_ast: Vec<FlatNode> = vec![];

    match expression {
        Expr::FnCall(expr, position) => {
            flattened_ast.append(&mut flatten_fn_call_expression(*expr, position))
        }
        Expr::Variable(data, _, position) => {
            flattened_ast.push(FlatNode::Op(Op::Push(data.3.to_string()), position))
        }

        Expr::IntegerConstant(val, position) => {
            flattened_ast.push(FlatNode::NumberLiteral(val as f64, position))
        }
        Expr::FloatConstant(val, position) => {
            flattened_ast.push(FlatNode::NumberLiteral(*val, position))
        }
        Expr::BoolConstant(val, position) => {
            flattened_ast.push(FlatNode::BooleanLiteral(val, position))
        }
        Expr::CharConstant(val, position) => {
            flattened_ast.push(FlatNode::StringLiteral(val.to_string(), position))
        }
        Expr::StringConstant(val, position) => {
            flattened_ast.push(FlatNode::StringLiteral(val.to_string(), position))
        }
        Expr::Unit(position) => flattened_ast.push(FlatNode::Unit(position)),
        Expr::DynamicConstant(val, position) => {
            flattened_ast.push(FlatNode::DynamicConstant(val, position))
        }
        Expr::Array(val, position) => {
            flattened_ast.push(FlatNode::Op(
                Op::FnCall("last_n_list".to_string()),
                position,
            ));
            flattened_ast.push(FlatNode::NumberLiteral(val.len() as f64, position));
            val.into_iter()
                .rev()
                .for_each(|v| flattened_ast.append(&mut flatten_expression(v)));
        }
        Expr::InterpolatedString(val, position) => {
            flattened_ast.append(&mut flatten_interpolated_string(val, position));
        }

        Expr::Map(_, _) => todo!(),

        Expr::ThisPtr(_) => todo!(),
        Expr::Property(_, _) => todo!(),
        Expr::MethodCall(_, _) => todo!(),
        Expr::Stmt(_) => todo!(),
        Expr::Dot(_, _, _) => todo!(),
        Expr::Index(_, _, _) => todo!(),
        Expr::And(_, _) => todo!(),
        Expr::Or(_, _) => todo!(),
        Expr::Coalesce(_, _) => todo!(),
        Expr::Custom(_, _) => todo!(),
        _ => todo!(),
    }

    return flattened_ast;
}

fn flatten_interpolated_string(val: Box<SmallVec<[Expr; 5]>>, position: Position) -> Vec<FlatNode> {
    val.into_iter()
        .enumerate()
        .flat_map(|(i, v)| {
            let mut intrs = vec![];
            let expr = &mut flatten_expression(v);

            let mut is_string = false;
            if expr.len() == 1 {
                if let FlatNode::StringLiteral(..) = expr[0] {
                    is_string = true;
                }
            };

            intrs.append(expr);
            if i > 0 {
                if !is_string {
                    intrs.push(FlatNode::Op(
                        Op::FnCall("string/iota".to_string()),
                        position,
                    ));
                }
                intrs.push(FlatNode::Op(Op::FnCall("string/add".to_string()), position));
            }
            intrs
        })
        .rev()
        .collect()
}

fn flatten_fn_call_expression(expression: FnCallExpr, position: Position) -> Vec<FlatNode> {
    let mut flattened_ast: Vec<FlatNode> = vec![];

    flattened_ast.push(FlatNode::Op(
        Op::FnCall(expression.name.to_string()),
        position,
    ));

    expression
        .args
        .into_iter()
        .rev()
        .for_each(|arg| flattened_ast.append(&mut flatten_expression(arg.clone())));

    return flattened_ast;
}

#[derive(Debug)]
pub enum Op {
    FnCall(String),
    Store(String),
    Push(String),
}

#[derive(Debug)]
pub enum FlatNode {
    Op(Op, Position),
    IfBlock {
        condition: Vec<FlatNode>,
        succeed: Vec<FlatNode>,
        fail: Option<Vec<FlatNode>>,
        position: Position,
    },
    WhileBlock {
        do_while: bool,
        condition: Vec<FlatNode>,
        block: Vec<FlatNode>,
        position: Position,
    },
    NumberLiteral(f64, Position),
    BooleanLiteral(bool, Position),
    StringLiteral(String, Position),
    DynamicConstant(Box<Dynamic>, Position),
    Unit(Position),
}
