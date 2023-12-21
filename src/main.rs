use std::fs;

use rhai::{Engine, EvalAltResult, Expr, FnCallExpr, Ident, Stmt, Expression, BinaryExpr};

use crate::flatten_ast::flattern_ast;

pub mod flatten_ast;
pub mod translate;
pub mod translate_ops;

fn main() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    let source = fs::read_to_string("./test.rhai").unwrap();

    //will be implemented eventually
    engine.disable_symbol("&&");
    engine.disable_symbol("||");


    let ast = engine.compile(source)?;

    println!("Ast: {:#?}", ast.statements());

    println!("Flattened Ast: {:?}", flattern_ast(ast.statements()));

    engine.eval_ast(&ast)?;

    Ok(())
}

