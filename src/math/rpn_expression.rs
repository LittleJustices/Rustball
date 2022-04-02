use std::{
    collections::{ HashMap, VecDeque },
    str::FromStr,
};
use lazy_static::lazy_static;
use super::math_errors::MathError;

lazy_static! { 
    static ref PRECEDENCE: HashMap<char, u8> = HashMap::from([
        ('^', 6),
        ('%', 6),
        ('*', 5),
        ('x', 5),
        ('/', 5),
        ('+', 4),
        ('-', 4),
        ('(', 0),
        (')', 0),
        ]);
}

#[derive(Debug)]
pub struct RpnExpression {
    infix_expression: String,
    postfix_expression: VecDeque<String>,
}

impl RpnExpression {
    fn shunting_yard_conversion(&mut self) -> Result<(), MathError> {
        let infix_vector = self.expression_to_vec();
        let mut operator_stack: Vec<char> = vec![];

        for item in infix_vector {
            if let Ok(_number) = item.parse::<f64>() {
                self.postfix_expression.push_back(item);
            } else if let Ok(operator) = item.parse::<char>() {
                if operator == '(' {
                    operator_stack.push(operator);
                } else if operator == ')' {
                    let mut operator_in_parens = ' ';
                    while operator_in_parens != '(' {
                        if let Some(op) = operator_stack.pop() {
                            operator_in_parens = op;
                            if operator_in_parens != '(' {
                                self.postfix_expression.push_back(operator_in_parens.to_string())
                            }
                        } else { return Err(MathError::ExpressionError("I think you're missing a parenthesis somewhere!".to_owned())) /* malformed expression error (no matching close paren) */ }
                    }
                } else if let Some(priority_right) = PRECEDENCE.get(&operator) {
                    while operator_stack.len() > 0 {
                        if let Some(priority_left) = PRECEDENCE.get(&operator_stack[operator_stack.len()-1]) {
                            if priority_left >= priority_right {
                                if let Some(op) = operator_stack.pop() {
                                    self.postfix_expression.push_back(op.to_string());
                                }
                            } else {
                                break;
                            }
                        } else { return Err(MathError::PlaceholderError) /* should be impossible to actually reach */ }
                    }
                    operator_stack.push(operator);
                }
            } else { return Err(MathError::SymbolError(item)) /* illegal input error (not a number or an operator) */ }
        }

        while operator_stack.len() > 0 {
            if let Some(item) = operator_stack.pop() {
                if item == '(' { return Err(MathError::ExpressionError("I think you're missing a parenthesis somewhere!".to_owned())) /* malformed expression error (no matching close paren) */ }
                self.postfix_expression.push_back(item.to_string());
            }
        }

        Ok(())
    }

    fn expression_to_vec(&self) -> Vec<String> {
        let mut infix_processed = self.infix_expression.replace(" ", "");
        for key in PRECEDENCE.keys() {
            infix_processed = infix_processed.replace(*key, &format!(" {} ", key));
        }

        let mut infix_vector = vec![];
        for symbol in infix_processed.split(' ') {
            infix_vector.push(symbol.to_owned());
        }

        infix_vector
    }

    pub fn get_rpn_expression(&self) -> &VecDeque<String> {
        &self.postfix_expression
    }
}

impl FromStr for RpnExpression {
    type Err = MathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rpn_expression = RpnExpression { infix_expression: s.to_owned(), postfix_expression: VecDeque::new() };

        rpn_expression.shunting_yard_conversion()?;

        Ok(rpn_expression)
    }
}