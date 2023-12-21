use rhai::{Expr, FnCallExpr, Ident, Position, Stmt};

pub fn flattern_ast(ast: &[Stmt]) -> Vec<FlatNode> {
    let mut flattened_ast: Vec<FlatNode> = vec![];

    for statement in ast {
        let mut flattened_ast_statment: Vec<FlatNode> = vec![];

        match statement {
            Stmt::Expr(_) => todo!(),
            Stmt::Var(data, _, _) => {
                flattened_ast_statment.append(&mut flatten_var((data.0.clone(), data.1.clone())))
            }

            Stmt::Assignment(data) => {
                let (variable, position) = if let Expr::Variable(ref var_data, _, pos) = data.1.lhs {
                    (var_data.3.clone().to_string(), pos)
                } else {
                    panic!()
                };

                flattened_ast_statment.push(FlatNode::Op(Op::Store(variable), position));
                flattened_ast_statment.append(&mut flatten_expression(data.1.rhs.clone()));
            }
            Stmt::FnCall(expr, position) => {
                flattened_ast_statment.append(&mut flatten_fn_call_expression(*expr.clone(), *position))
            }

            Stmt::Noop(_) => todo!(),
            Stmt::If(_, _) => todo!(),
            Stmt::Switch(_, _) => todo!(),
            Stmt::While(_, _) => todo!(),
            Stmt::Do(_, _, _) => todo!(),
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

fn flatten_var(data: (Ident, Expr)) -> Vec<FlatNode> {
    let mut flattened_ast: Vec<FlatNode> = vec![];

    let identifier = data.0;
    let expression = data.1;

    flattened_ast.push(FlatNode::Op(Op::Store(identifier.name.to_string()), identifier.pos));
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
        Expr::InterpolatedString(_, _) => todo!(),
        Expr::DynamicConstant(_, _) => todo!(),

        Expr::Array(_, _) => todo!(),
        Expr::Map(_, _) => todo!(),
        Expr::Unit(_) => todo!(),

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

fn flatten_fn_call_expression(expression: FnCallExpr, position: Position) -> Vec<FlatNode> {
    let mut flattened_ast: Vec<FlatNode> = vec![];

    flattened_ast.push(FlatNode::Op(Op::FnCall(expression.name.to_string()), position));

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
    NumberLiteral(f64, Position),
    BooleanLiteral(bool, Position),
    StringLiteral(String, Position),
    Unit(Position),
}
