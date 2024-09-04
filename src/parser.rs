use crate::lexer::Token;

#[derive(Debug)]
pub enum Expression{
    Number(i32),
    Variable(String),
    BinaryOperation(Box<Expression>, Operator, Box<Expression>)
}

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration(String, Expression),
    IfStatement(Expression, Box<Statement>),
    PrintStatement(String),
}
#[derive(Debug)]
pub enum Operator {
    Plus, Minus, Multiply, Divide, GreaterThan, LessThan
}

pub(crate) fn parse(tokens: &[Token]) -> Vec<Statement> {
    let mut statements = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Let => {
                if let Token::Identifier(var_name) = &tokens[i+1] {
                    if let Token::Equals = tokens[i+2]{
                        if let Token::Number(value) = tokens[i+3]{
                            statements.push(Statement::VariableDeclaration(var_name.clone(), Expression::Number(value)));
                            i+=4;
                        }
                    }
                }
            }
            /*
            यदि संख्या > 5 {
                तब("संख्या बड़ी है 5 से");
            }
             */
            Token::If => {
                if let Token::Identifier(var_name) = &tokens[i+1] {
                    if let Token::GreaterThan = tokens[i+2] {
                        if let Token::Number(value) = tokens[i+3] {
                            if let Token::PrintlnMacro = &tokens[i+5] {
                                if let Token::StringLiteral(msg) = &tokens[i+6] {
                                    statements.push(Statement::IfStatement(
                                        Expression::BinaryOperation(
                                            Box::new(Expression::Variable(var_name.clone())),
                                            Operator::GreaterThan,
                                            Box::new(Expression::Number(value)),
                                        ),
                                        Box::new(Statement::PrintStatement(msg.clone())),
                                    ));
                                    i += 7;
                                }
                            }
                        }

                    }
                }
            }
            _ => {
                i+=1;
            }
        }
    }
    statements
}