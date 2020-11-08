mod ast;
mod codegen;
mod env;
mod semantic;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar);

extern crate clap;
use clap::{App, Arg};

use crate::grammar::ModuleParser;
// use crate::codegen::LLVMGenerator;
// use crate::semantic::semantic_check;

fn main() {
    let matches = App::new("jackc")
        .version("0.1.0")
        .author("tkclimb")
        .about("Jack Programming Language Compiler")
        .arg(Arg::with_name("source_file").required(true))
        // .arg(Arg::with_name("source").short("s").required(true))
        .get_matches();

    let source_file_path = matches
        .value_of("source_file")
        .expect("source file missing...");

    println!("source file path: {}", source_file_path);

    let contents = std::fs::read_to_string(source_file_path).expect("[error] read_to_string");
    let stmt_list = ModuleParser::new().parse(&contents.to_string()).unwrap();
    println!("{:?}", stmt_list);

    // let typed_ast = semantic_check(stmts);
    // unsafe {
    //     let mut generator = LLVMGenerator::new();
    //     generator.run(&fname.to_string(), &typed_ast);
    // }
}
