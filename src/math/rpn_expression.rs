use super::{
    math_errors::MathError,
    math_re::MATH_TOKEN_RE,
    rpn_token::RpnToken,
};

#[derive(Debug)]
pub struct RpnExpression;

impl RpnExpression {
    pub fn tokenize_expression(infix_expression: &str) -> Result<Vec<RpnToken>, MathError> {
        let whitespace_cleaned = infix_expression.replace(" ", "");
        let infix_processed = MATH_TOKEN_RE.replace_all(&whitespace_cleaned, " $token ");

        let mut infix_vector = vec![];
        for symbol in infix_processed.split_whitespace() {
            infix_vector.push(symbol.parse()?);
        }

        Ok(infix_vector)
    }

    pub fn shunting_yard(infix_vector: &[RpnToken]) -> Result<Vec<RpnToken>, MathError> {
        let mut postfix_queue = vec![];
        let mut token_stack: Vec<RpnToken> = vec![];

        for token in infix_vector.to_vec() {
            match &token {
                RpnToken::Number(_) => postfix_queue.push(token),
                // When/if functions are implemented: If token is a function, push onto stack
                RpnToken::Operator(right_operator) => {
                    while let Some(RpnToken::Operator(left_operator)) = token_stack.last() {
                        if (left_operator.precedence() > right_operator.precedence()) | 
                            (left_operator.precedence() == right_operator.precedence()) && (right_operator.left_associative()) {
                            postfix_queue.push(token_stack.pop().ok_or(MathError::PlaceholderError)?);
                        } else {
                            break;
                        }
                    }
                    token_stack.push(token);
                },
                RpnToken::LParen => token_stack.push(token),
                RpnToken::RParen => {
                    while let Some(operator) = token_stack.last() {
                        if operator == &RpnToken::LParen { break; }
                        postfix_queue.push(token_stack.pop().ok_or(MathError::ImpossibleError)?);
                    }
                    if token_stack.last() != Some(&RpnToken::LParen) {
                        return Err(MathError::ExpressionError("I think you're missing a parenthesis somewhere!".into()));
                    } else {
                        token_stack.pop();
                    }
                    // If there is a function token at the top of the stack, pop it onto the queue
                }
            }
        }

        while let Some(token) = token_stack.pop() {
            match token {
                RpnToken::LParen | RpnToken::RParen => return Err(MathError::ExpressionError("I wasn't expecting a parenthesis there!".into())),
                other => postfix_queue.push(other)
            }
        }

        Ok(postfix_queue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::rpn_token::*;

    #[test]
    fn test_tokenize() {
        let expression = "(1+2)-3*(4/5)^6.7";
        let token_vector = vec![
            RpnToken::LParen,
            RpnToken::Number(1.0),
            RpnToken::Operator(Operator::Add),
            RpnToken::Number(2.0),
            RpnToken::RParen,
            RpnToken::Operator(Operator::Sub),
            RpnToken::Number(3.0),
            RpnToken::Operator(Operator::Mul),
            RpnToken::LParen,
            RpnToken::Number(4.0),
            RpnToken::Operator(Operator::Div),
            RpnToken::Number(5.0),
            RpnToken::RParen,
            RpnToken::Operator(Operator::Pow),
            RpnToken::Number(6.7)
        ];

        assert_eq!(RpnExpression::tokenize_expression(expression).unwrap(), token_vector);
        // TODO more tests
    }

    #[test]
    fn test_shunting() {
        let expression = "3+4*2/(1-5)^2^3";
        let token_vector = RpnExpression::tokenize_expression(expression).unwrap();

        let infix = vec![
            RpnToken::Number(3.0),
            RpnToken::Operator(Operator::Add),
            RpnToken::Number(4.0),
            RpnToken::Operator(Operator::Mul),
            RpnToken::Number(2.0),
            RpnToken::Operator(Operator::Div),
            RpnToken::LParen,
            RpnToken::Number(1.0),
            RpnToken::Operator(Operator::Sub),
            RpnToken::Number(5.0),
            RpnToken::RParen,
            RpnToken::Operator(Operator::Pow),
            RpnToken::Number(2.0),
            RpnToken::Operator(Operator::Pow),
            RpnToken::Number(3.0),
        ];

        let postfix = vec![
            RpnToken::Number(3.0),
            RpnToken::Number(4.0),
            RpnToken::Number(2.0),
            RpnToken::Operator(Operator::Mul),
            RpnToken::Number(1.0),
            RpnToken::Number(5.0),
            RpnToken::Operator(Operator::Sub),
            RpnToken::Number(2.0),
            RpnToken::Number(3.0),
            RpnToken::Operator(Operator::Pow),
            RpnToken::Operator(Operator::Pow),
            RpnToken::Operator(Operator::Div),
            RpnToken::Operator(Operator::Add),
        ];

        assert_eq!(token_vector, infix);
        assert_eq!(RpnExpression::shunting_yard(&token_vector).unwrap(), postfix);
    }
}
