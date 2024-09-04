#![allow(warnings)]
mod lexer;
mod parser;
mod abstract_syntax_tree;

use std::os::linux::raw::stat;
use crate::lexer::lex;
use crate::parser::parse;
use crate::abstract_syntax_tree::evaluate;


fn main() {
    let code = r#"
        जब संख्या = 10;
        यदि संख्या > 5 {
            तब("संख्या बड़ी है 5 से");
        }"#;
    let tokens = lex(code);
    println!("--------- Tokens ---------");
    for token in tokens.iter().clone() {
        println!("{:?}", token);
    }
    let statements = parse(&tokens);
    println!("\n --------- Statements ---------");
    for statement in statements.iter().clone() {
        println!("{:?}", statement);
    }
    println!("\n--------- Output ---------");
    evaluate(statements);
}
