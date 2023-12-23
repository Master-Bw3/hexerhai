use std::{collections::HashMap, fs};

use hexagon::{
    compiler::{compile_to_iotas, nbt::gen_give_cmd},
    parse_config::Config,
    pattern_registry::{PatternRegistry, PatternRegistryExt}, interpreter::{interpret, error::print_interpreter_error}, parser::AstNode, iota::Iota,
};
use rhai::{BinaryExpr, Engine, EvalAltResult, Expr, Expression, FnCallExpr, Ident, Stmt};
use translate::translate_flattened_ast;

use crate::flatten_ast::flattern_ast;

pub mod flatten_ast;
pub mod translate;
pub mod translate_ops;
pub mod translate_dynamic;

fn main() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    let source = fs::read_to_string("./test.rhai").unwrap();

    //will be implemented eventually
    engine.disable_symbol("&&");
    engine.disable_symbol("||");
    engine.disable_symbol("<<");
    engine.disable_symbol(">>");

    let ast = engine.compile(source)?;

    engine.eval_ast(&ast)?;


    println!("Ast: {:#?}", ast.statements());

    println!("Flattened Ast: {:?}", flattern_ast(ast.statements()));


    compile()?;

    run()?;

    Ok(())
}

fn compile() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    let source = fs::read_to_string("./test.rhai").unwrap();

    //will be implemented eventually
    engine.disable_symbol("&&");
    engine.disable_symbol("||");
    engine.disable_symbol("<<");
    engine.disable_symbol(">>");

    let ast = engine.compile(source)?;

    let translated_ast = translate_flattened_ast(flattern_ast(ast.statements()));

    let config = Config {
        libraries: HashMap::new(),
        entities: HashMap::new(),
        great_spell_sigs: PatternRegistry::gen_default_great_sigs(),
    };

    let pattern_registry = PatternRegistry::construct(&config.great_spell_sigs);

    let compile_result = compile_to_iotas(
        &hexagon::parser::AstNode::Program(translated_ast),
        None,
        &pattern_registry,
        &HashMap::new(),
    );

    match compile_result {
        // Ok(result) => println!("\nresult: {}", Vector::from(result).display()),
        Ok(result) => println!("\nresult: {}", gen_give_cmd(result)),

        Err(err) => {
            println!("e {:?}", err)
        }
    };

    Ok(())
}

fn run() -> Result<(), Box<EvalAltResult>> {

    let mut engine = Engine::new();

    let source = fs::read_to_string("./test.rhai").unwrap();

    //will be implemented eventually
    engine.disable_symbol("&&");
    engine.disable_symbol("||");
    engine.disable_symbol("<<");
    engine.disable_symbol(">>");

    let ast = engine.compile(source)?;

    let translated_ast = translate_flattened_ast(flattern_ast(ast.statements()));

    let config = Config {
        libraries: HashMap::new(),
        entities: HashMap::new(),
        great_spell_sigs: PatternRegistry::gen_default_great_sigs(),
    };

    let pattern_registry = PatternRegistry::construct(&config.great_spell_sigs);

    let interpreter_result = interpret(AstNode::Program(translated_ast), &config, HashMap::new(), "", "");

        match interpreter_result {
            Ok(result) => println!(
                "\nresult: {} \n {:?}",
                result.stack.display(),
                result.buffer
            ),
            Err(err) => {
                print_interpreter_error(err,"", "");
            }
        };

        Ok(())
}