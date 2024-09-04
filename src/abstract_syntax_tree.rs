use crate::parser::*;
use std::collections::HashMap;

pub(crate) fn evaluate(statements: Vec<Statement>) {
    let mut variables = HashMap::new();

    for s in statements {
        match s {
            Statement::VariableDeclaration(var_name, expression) => {
                let value = evaluate_exp(&expression, &variables);
                variables.insert(var_name, value);
            }

            Statement::IfStatement(condition, body) => {
                if evaluate_exp(&condition, &variables) > 0 {
                    evaluate(vec![*body]);
                }
            }

            Statement::PrintStatement(msg) => {
                println!("{}", msg);
            }
        }
    }
}

fn evaluate_exp(expr: &Expression, vars: &HashMap<String, i32>) -> i32 {
    match expr {
        Expression::Number(n) => *n,
        Expression::Variable(name) => *vars.get(name).expect("Variable not found"),
        Expression::BinaryOperation( lhs, op, rhs) => {
            let left_value = evaluate_exp(lhs, vars); // Get variable value
            let right_value = evaluate_exp(rhs, vars);

            match op {
                Operator:: Plus => left_value + right_value,
                Operator:: Minus => left_value - right_value,
                Operator:: Multiply => left_value*right_value,
                Operator:: Divide => left_value / right_value,
                Operator:: GreaterThan => (left_value > right_value) as i32,
                Operator:: LessThan => (left_value < right_value) as i32,
                _ => left_value
            }
        }
    }
}