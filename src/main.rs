use crate::lexer::lex;

mod lexer;

fn main() {
    let code = r#"
        जब संख्या = 10;
        यदि संख्या > 5 {
            तब("संख्या बड़ी है 5 से");
        }"#;
    let tokens = lex(code);
    for token in tokens {
        println!("{:?}", token);
    }
}
