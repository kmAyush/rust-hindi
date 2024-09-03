#[derive(Debug)]
pub enum Token {
    Let, Fn, If, Else, Identifier(String),
    Number(i32), LeftParen, RightParen, Semicolon,
    Plus, Minus, Multiply, Divide, Equals,
    LeftAngle, RightAngle, LeftBrace, RightBrace, PrintlnMacro,
    EndOfInput, StringLiteral(String),
}
pub fn lex(input: &str) -> Vec<Token>{
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\n' | '\t' => {
                chars.next();
            }
            'ज' => {
                let collected: String = chars.clone().take(3).collect();
                if collected.starts_with("जब") {
                    // Advance the iterator by 3 characters
                    chars.nth(2); // Skips 3 characters, so it moves the iterator past 'जब'
                    tokens.push(Token::Let);  // Replace with your actual token
                    continue; // Skip to the next iteration
                }
            }
            'क' => {
                let collected: String = chars.clone().take(4).collect();
                if collected.starts_with("कार्य") {
                    chars.nth(3);
                    tokens.push(Token::Fn);
                    continue;
                }
            }
            'य' => {
                let collected: String = chars.clone().take(3).collect();
                if collected.starts_with("यदि") {
                    chars.nth(2);
                    tokens.push(Token::If);
                    continue;
                }
            }
            'व' => {
                let collected: String = chars.clone().take(4).collect();
                if collected.starts_with("वरना") {
                    chars.nth(3);
                    tokens.push(Token::Fn);
                    continue;
                }
            }
            'त' => {
                let collected: String = chars.clone().take(3).collect();
                if collected.starts_with("तब") {
                    chars.nth(2);
                    tokens.push(Token::If);
                    continue;
                }
            }

            '0' ..='9' => {
                let mut number = 0;
                while let Some(c) = chars.peek() {
                    if c.is_digit(10){
                        number = number * 10 + c.to_digit(10).unwrap() as i32;
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(number));
            }
            c if is_valid_identifier(c) => {
                let mut identifier = String::new();
                while let Some(&c) = chars.peek(){
                    if is_valid_identifier(c) {
                        identifier.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Identifier(identifier));
            }

            '=' => {
                chars.next();
                tokens.push(Token::Equals);
            }
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(Token::Minus);
            }
            '*' => {
                chars.next();
                tokens.push(Token::Multiply);
            }
            '/' => {
                chars.next();
                tokens.push(Token::Divide);
            }
            '(' => {
                chars.next();
                tokens.push(Token::LeftParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RightParen);
            }
            ';' => {
                chars.next();
                tokens.push(Token::Semicolon);
            }
            '>' => {
                chars.next();
                tokens.push(Token::LeftAngle);
            }
            '<' => {
                chars.next();
                tokens.push(Token::RightAngle);
            }
            '{' => {
                chars.next();
                tokens.push(Token::LeftBrace);
            }
            '}' => {
                chars.next();
                tokens.push(Token::RightBrace);
            }
            '"' => {
                chars.next();
                let mut string_literal = String::new();
                while let Some(&c) = chars.peek() {
                    if c == '"' {
                        chars.next();
                        break;
                    }
                    string_literal.push(c);
                    chars.next();
                }
                tokens.push(Token::StringLiteral(string_literal));
            }
            _ => {
                panic!("Unexpected character : {} ",c);
            }
        }
    }
    tokens.push(Token::EndOfInput);
    tokens
}
fn is_valid_identifier(c: char) -> bool {
    let is_hindi = c >= '\u{0900}' && c <= '\u{097F}';
    let is_latin = c.is_alphanumeric() || c == '_';
    is_hindi || is_latin
}