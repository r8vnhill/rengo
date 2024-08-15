use crate::ast::expr::Expression;
use crate::parser::token::Token;

/// Parses a complete expression from the provided token stream and returns the corresponding
/// abstract syntax tree (AST) representation.
///
/// ## Usage:
/// This function serves as the entry point for parsing a full expression from a list of tokens.
/// It relies on `parse_expression` to handle the actual parsing of the expression and ensures
/// that the entire input is consumed correctly.
///
/// ### Example 1: Parsing a simple expression
/// ```rust
/// let tokens = vec![Token::Number(5)];
/// let result = parse(&tokens);
/// assert_eq!(result, Ok(Expression::Number(5)));
/// ```
///
/// ### Example 2: Parsing an expression with increment
/// ```rust
/// let tokens = vec![Token::Number(5), Token::Increment];
/// let result = parse(&tokens);
/// assert_eq!(result, Ok(Expression::Increment(Box::new(Expression::Number(5)))));
/// ```
///
/// ### Example 3: Handling an invalid expression
/// ```rust
/// let tokens = vec![Token::LParen, Token::Number(5)];
/// let result = parse(&tokens);
/// assert!(result.is_err());
/// ```
///
/// ## Parameters:
/// - `tokens`: A slice of tokens representing the input expression to parse.
///
/// ## Returns:
/// A `Result` containing the parsed `Expression` if successful, or a `String` error message
/// if parsing fails.
///
/// ## Errors:
/// - Returns an error if the token stream does not form a valid expression.
/// - Returns an error if the token stream contains unmatched parentheses or other syntax issues.
pub fn parse(tokens: &[Token]) -> Result<Expression, String> {
    let (expression, _) = parse_expression(tokens, 0)?;
    Ok(expression)
}

/// Parses an expression from the token stream. In this basic implementation, an expression is
/// equivalent to a term, which can be a factor followed by increment (`++`) or decrement (`--`)
/// operations.
///
/// ## Usage:
/// This function is typically used as part of a recursive descent parser to parse expressions
/// within a mathematical expression. Currently, the implementation is a placeholder that directly
/// delegates to `parse_term`.
///
/// ### Example 1: Parsing a simple term
/// ```rust
/// let tokens = vec![Token::Number(5)];
/// let result = parse_expression(&tokens, 0);
/// assert_eq!(result, Ok((Expression::Number(5), 1)));
/// ```
///
/// ### Example 2: Parsing a term with increment
/// ```rust
/// let tokens = vec![Token::Number(5), Token::Increment];
/// let result = parse_expression(&tokens, 0);
/// assert_eq!(result, Ok((Expression::Increment(Box::new(Expression::Number(5))), 2)));
/// ```
///
/// ### Example 3: Parsing a term with decrement
/// ```rust
/// let tokens = vec![Token::Number(5), Token::Decrement];
/// let result = parse_expression(& tokens, 0);
/// assert_eq!(result, Ok((Expression::Decrement(Box::new(Expression::Number(5))), 2)));
/// ```
///
/// ## Parameters:
/// - `tokens`: A slice of tokens representing the input to parse.
/// - `index`: The index in the token stream to start parsing from.
///
/// ## Returns:
/// A `Result` containing a tuple with the parsed `Expression` and the index of the next token to
/// parse, or a `String` error message if parsing fails.
fn parse_expression(tokens: &[Token], index: usize) -> Result<(Expression, usize), String> {
    parse_term(tokens, index)
}

/// Parses a term from the token stream, which can consist of a factor followed by increment (`++`)
/// or decrement (`--`) operations.
///
/// ## Usage:
/// This function is typically used as part of a recursive descent parser to parse terms within a
/// mathematical expression. A term is a factor that can be followed by zero or more increment or
/// decrement operations.
///
/// ### Example 1: Parsing a simple factor
/// ```rust
/// let tokens = vec![Token::Number(5)];
/// let result = parse_term(&tokens, 0);
/// assert_eq!(result, Ok((Expression::Number(5), 1)));
/// ```
///
/// ### Example 2: Parsing a factor with increment
/// ```rust
/// let tokens = vec![Token::Number(5), Token::Increment];
/// let result = parse_term(&tokens, 0);
/// assert_eq!(result, Ok((Expression::Increment(Box::new(Expression::Number(5))), 2)));
/// ```
///
/// ### Example 3: Parsing a factor with decrement
/// ```rust
/// let tokens = vec![Token::Number(5), Token::Decrement];
/// let result = parse_term(&tokens, 0);
/// assert_eq!(result, Ok((Expression::Decrement(Box::new(Expression::Number(5))), 2)));
/// ```
///
/// ### Example 4: Handling multiple increments and decrements
/// ```rust
/// let tokens = vec![Token::Number(5), Token::Increment, Token::Increment, Token::Decrement];
/// let result = parse_term(&tokens, 0);
/// assert_eq!(result, Ok((Expression::Decrement(Box::new(Expression::Increment(Box::new(Expression::Increment(Box::new(Expression::Number(5))))))), 4)));
/// ```
///
/// ## Parameters:
/// - `tokens`: A slice of tokens representing the input to parse.
/// - `index`: The index in the token stream to start parsing from.
///
/// ## Returns:
/// A `Result` containing a tuple with the parsed `Expression` and the index of the next token to parse,
/// or a `String` error message if parsing fails.
fn parse_term(tokens: &[Token], index: usize) -> Result<(Expression, usize), String> {
    let (mut expression, mut index) = parse_factor(tokens, index)?;

    while index < tokens.len() {
        match tokens[index] {
            Token::Increment => {
                index += 1; // consume '++'
                expression = Expression::Increment(Box::new(expression));
            }
            Token::Decrement => {
                index += 1; // consume '--'
                expression = Expression::Decrement(Box::new(expression));
            }
            _ => break,
        }
    }

    Ok((expression, index))
}

/// Parses a factor from the token stream, which can be either a number or a parenthesized
/// expression.
///
/// ## Usage:
/// This function is typically used as part of a recursive descent parser to parse individual
/// factors within a mathematical expression. A factor is either a standalone number or an
/// expression enclosed in parentheses.
///
/// ### Example 1: Parsing a number
/// ```kotlin
/// let tokens = vec![Token::Number(5)];
/// let result = parse_factor(&tokens, 0);
/// assert_eq!(result, Ok((Expression::Number(5), 1)));
/// ```
///
/// ### Example 2: Parsing a parenthesized expression
/// ```kotlin
/// let tokens = vec![Token::LParen, Token::Number(5), Token::RParen];
/// let result = parse_factor(&tokens, 0);
/// assert_eq!(result, Ok((Expression::Number(5), 3)));
/// ```
///
/// ### Example 3: Handling unexpected tokens
/// ```kotlin
/// let tokens = vec![Token::Plus];
/// let result = parse_factor(&tokens, 0);
/// assert!(result.is_err());
/// ```
///
/// ## Parameters:
/// - `tokens`: A slice of tokens representing the input to parse.
/// - `index`: The index in the token stream to start parsing from.
///
/// ## Returns:
/// A tuple containing the parsed expression and the index of the next token to parse.
fn parse_factor(tokens: &[Token], index: usize) -> Result<(Expression, usize), String> {
    match tokens.get(index) {
        Some(Token::Number(value)) => Ok((Expression::Number(*value), index + 1)),
        Some(Token::LParen) => {
            let (expression, next_index) = parse_expression(tokens, index + 1)?;
            match tokens.get(next_index) {
                Some(Token::RParen) => Ok((expression, next_index + 1)),
                _ => Err("Expected closing parenthesis".to_string()),
            }
        }
        Some(token) => Err(format!("Unexpected token: {:?}", token)),
        None => Err("Unexpected end of input".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use expectest::prelude::*;
    use super::*;

    mod parse_factor {
        use super::*;

        #[test]
        fn number() {
            let tokens = vec![Token::Number(42)];
            let (expression, next_index) = parse_factor(&tokens, 0).unwrap();
            expect!(expression).to(be_equal_to(Expression::Number(42)));
            expect!(next_index).to(be_equal_to(1));
        }

        #[test]
        fn parenthesized_expression() {
            let tokens = vec![Token::LParen, Token::Number(42), Token::RParen];
            let (expression, next_index) = parse_factor(&tokens, 0).unwrap();
            expect!(expression).to(be_equal_to(Expression::Number(42)));
            expect!(next_index).to(be_equal_to(3));
        }

        #[test]
        fn unexpected_token() {
            let tokens = vec![Token::Increment];
            let result = parse_factor(&tokens, 0);
            expect!(result).to(be_err().value("Unexpected token: Increment"));
        }

        #[test]
        fn unexpected_end_of_input() {
            let tokens = vec![];
            let result = parse_factor(&tokens, 0);
            expect!(result).to(be_err().value("Unexpected end of input"));
        }
    }

    mod parse_term {
        use super::*;

        #[test]
        fn increment() {
            let tokens = vec![Token::Number(42), Token::Increment];
            let (expression, next_index) = parse_term(&tokens, 0).unwrap();
            expect!(expression).to(be_equal_to(Expression::Increment(Box::new(Expression::Number(42)))));
            expect!(next_index).to(be_equal_to(2));
        }

        #[test]
        fn decrement() {
            let tokens = vec![Token::Number(42), Token::Decrement];
            let (expression, next_index) = parse_term(&tokens, 0).unwrap();
            expect!(expression).to(be_equal_to(Expression::Decrement(Box::new(Expression::Number(42)))));
            expect!(next_index).to(be_equal_to(2));
        }
    }

    mod parse_expression {
        use super::*;

        #[test]
        fn number() {
            let tokens = vec![Token::Number(42)];
            let (expression, next_index) = parse_expression(&tokens, 0).unwrap();
            expect!(expression).to(be_equal_to(Expression::Number(42)));
            expect!(next_index).to(be_equal_to(1));
        }

        #[test]
        fn increment() {
            let tokens = vec![Token::Number(42), Token::Increment];
            let (expression, next_index) = parse_expression(&tokens, 0).unwrap();
            expect!(expression).to(be_equal_to(Expression::Increment(Box::new(Expression::Number(42)))));
            expect!(next_index).to(be_equal_to(2));
        }

        #[test]
        fn decrement() {
            let tokens = vec![Token::Number(42), Token::Decrement];
            let (expression, next_index) = parse_expression(&tokens, 0).unwrap();
            expect!(expression).to(be_equal_to(Expression::Decrement(Box::new(Expression::Number(42)))));
            expect!(next_index).to(be_equal_to(2));
        }
    }

    mod parse {
        use super::*;

        #[test]
        fn number() {
            let tokens = vec![Token::Number(42)];
            let expression = parse(&tokens).unwrap();
            expect!(expression).to(be_equal_to(Expression::Number(42)));
        }

        #[test]
        fn increment() {
            let tokens = vec![Token::Number(42), Token::Increment];
            let expression = parse(&tokens).unwrap();
            expect!(expression).to(be_equal_to(Expression::Increment(Box::new(Expression::Number(42)))));
        }

        #[test]
        fn decrement() {
            let tokens = vec![Token::Number(42), Token::Decrement];
            let expression = parse(&tokens).unwrap();
            expect!(expression).to(be_equal_to(Expression::Decrement(Box::new(Expression::Number(42)))));
        }
    }
}
