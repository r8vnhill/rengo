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

/// Parses an expression from the token stream.
///
/// The `parse_expression` function is a core component of a recursive descent parser, responsible
/// for interpreting a sequence of tokens as an abstract syntax tree (AST). This function supports
/// both simple arithmetic expressions and more complex constructs like `let` bindings.
///
/// ## Usage:
/// This function identifies the type of expression starting at the given index in the token stream
/// and delegates the parsing to the appropriate helper function. If the expression begins with a
/// `let` keyword, it calls `parse_let` to handle the `let` binding. Otherwise, it processes the
/// expression as a term (a factor potentially followed by increment (`++`) or decrement (`--`)
/// operations).
///
/// ### Example 1: Parsing a simple numeric expression
/// ```rust
/// let tokens = vec![Token::Number(5)];
/// let result = parse_expression(&tokens, 0);
/// assert_eq!(result, Ok((Expression::Number(5), 1)));
/// ```
///
/// ### Example 2: Parsing an expression with increment
/// ```rust
/// let tokens = vec![Token::Number(5), Token::Increment];
/// let result = parse_expression(&tokens, 0);
/// assert_eq!(result, Ok((Expression::Increment(Box::new(Expression::Number(5))), 2)));
/// ```
///
/// ### Example 3: Parsing a `let` binding expression
/// ```rust
/// let tokens = vec![
///     Token::Let,
///     Token::Identifier("x".to_string()),
///     Token::Assign,
///     Token::Number(5),
///     Token::LineEnd,
///     Token::Identifier("x".to_string())
/// ];
/// let result = parse_expression(&tokens, 0);
/// assert_eq!(
///     result,
///     Ok((
///         Expression::Let(
///             "x".to_string(),
///             Box::new(Expression::Number(5)),
///             Box::new(Expression::Identifier("x".to_string()))
///         ),
///         6
///     ))
/// );
/// ```
///
/// ## Parameters:
/// - `tokens`: A slice of tokens representing the input expression to be parsed.
/// - `index`: The starting index in the token stream where the expression begins.
///
/// ## Returns:
/// A `Result` containing:
/// - A tuple with the parsed `Expression` and the index of the next token to parse, if successful.
/// - A `String` error message if parsing fails.
///
/// ## Errors:
/// - Returns an error if the token stream does not form a valid expression.
/// - Returns an error if the token stream contains syntax issues like missing `;`, `=`, or
///     parentheses.
fn parse_expression(tokens: &[Token], index: usize) -> Result<(Expression, usize), String> {
    if let Some(Token::Let) = tokens.get(index) {
        parse_let(tokens, index + 1)
    } else {
        parse_term(tokens, index)
    }
}

/// Parses a `let` expression from the token stream.
///
/// The `parse_let` function is responsible for parsing `let` bindings in the form of:
///
/// ```text
/// let x = <expression>;
/// ```
///
/// It processes the identifier, the assignment operator, the expression to be assigned, and the
/// body of the `let` binding. This function returns an `Expression::Let` variant containing the
/// parsed components and the index of the next token to be parsed.
///
/// ## Usage:
/// This function is typically called when a `let` keyword is encountered in the token stream
/// during the parsing process. It expects the `let` keyword to be followed by an identifier, an
/// assignment operator (`=`), an expression, a line-end (`;`), and then the body expression.
///
/// ### Example:
/// ```rust
/// let tokens = vec![
///     Token::Let,
///     Token::Identifier("x".to_string()),
///     Token::Assign,
///     Token::Number(5),
///     Token::LineEnd,
///     Token::Identifier("x".to_string())
/// ];
/// let result = parse_let(&tokens, 1);
/// assert_eq!(
///     result,
///     Ok((
///         Expression::Let(
///             "x".to_string(),
///             Box::new(Expression::Number(5)),
///             Box::new(Expression::Identifier("x".to_string()))
///         ),
///         6
///     ))
/// );
/// ```
///
/// ## Parameters:
/// - `tokens`: A slice of tokens representing the input to parse.
/// - `index`: The index in the token stream where the `let` expression starts.
///
/// ## Returns:
/// A `Result` containing a tuple with the parsed `Expression::Let` and the index of the next token
/// to be parsed, or a `String` error message if parsing fails.
///
/// ## Errors:
/// - Returns an error if the expected identifier is missing after the `let` keyword.
/// - Returns an error if the assignment operator (`=`) is missing after the identifier.
/// - Returns an error if the line-end (`;`) is missing after the assigned expression.
/// - Returns an error if there are issues parsing the expression or the body of the `let` binding.
fn parse_let(tokens: &[Token], index: usize) -> Result<(Expression, usize), String> {
    if let Some(Token::Identifier(ref name)) = tokens.get(index) {
        let next_index = index + 1;
        if let Some(Token::Assign) = tokens.get(next_index) {
            let (value_expr, body_start) = parse_expression(tokens, next_index + 1)?;
            if let Some(Token::LineEnd) = tokens.get(body_start) {
                let (body_expr, final_index) = parse_expression(tokens, body_start + 1)?;
                Ok((Expression::Let(name.clone(), Box::new(value_expr), Box::new(body_expr)), final_index))
            } else {
                Err("Expected ';' at the end of let binding".to_string())
            }
        } else {
            Err("Expected '=' in let binding".to_string())
        }
    } else {
        Err("Expected identifier after 'let'".to_string())
    }
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
        Some(Token::Identifier(ref name)) => Ok((Expression::Identifier(name.clone()), index + 1)),
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

        #[test]
        fn identifier() {
            let tokens = vec![Token::Identifier("x".to_string())];
            let (expression, next_index) = parse_factor(&tokens, 0).unwrap();
            expect!(expression).to(be_equal_to(Expression::Identifier("x".to_string())));
            expect!(next_index).to(be_equal_to(1));
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

        #[test]
        fn let_binding() {
            let tokens = vec![
                Token::Let,
                Token::Identifier("x".to_string()),
                Token::Assign,
                Token::Number(5),
                Token::LineEnd,
                Token::Identifier("x".to_string())
            ];
            let (expression, next_index) = parse_expression(&tokens, 0).unwrap();
            expect!(expression).to(be_equal_to(Expression::Let(
                "x".to_string(),
                Box::new(Expression::Number(5)),
                Box::new(Expression::Identifier("x".to_string()))
            )));
            expect!(next_index).to(be_equal_to(6));
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
