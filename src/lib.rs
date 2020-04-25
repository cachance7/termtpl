#[macro_use]
extern crate lalrpop_util;

pub mod ast;
pub mod eval;
pub mod lexer;

lalrpop_mod!(pub parser);

pub fn compile(input: &str) -> Result<ast::Template, String> {
    // Our grammer requires a \n at the end of each line. just append it to
    // make happy
    let input = &String::from(format!("{}\n",input));
    match parser::TemplateParser::new().parse(input, lexer::Lexer::new(input)) {
        Ok(s) => Ok(ast::Template::new(s)),
        Err(e) => Err(format!("{:?}", e)),
    }
}

#[test]
fn parse_simple() {
    let input = lexer::Lexer::new("\n\n\n");
    let program = parser::TemplateParser::new().parse(input).expect("Oh no");
    // match (program.len(), program.first()) {
    //     (1, Some(&ast::Stmt::Exit)) => (),
    //     other => panic!("Well that didn't work: {:?}", other),
    // }
}
