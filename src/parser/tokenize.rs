use crate::parser::token::Token;

/// Tokenizes an input string into a vector of tokens, which can be used for further parsing.
///
/// ## Usage:
/// This function scans through the input string and converts it into a list of tokens. The tokens
/// represent different components of a mathematical expression, such as numbers, operators, and
/// parentheses.
///
/// ### Example 1: Tokenizing a simple number
/// ```rust
/// let input = "123";
/// let result = tokenize(input);
/// assert_eq!(result, Ok(vec![Token::Number(123)]));
/// ```
///
/// ### Example 2: Tokenizing an increment operation
/// ```rust
/// let input = "++";
/// let result = tokenize(input);
/// assert_eq!(result, Ok(vec![Token::Increment]));
/// ```
///
/// ### Example 3: Handling invalid input
/// ```rust
/// let input = "+";
/// let result = tokenize(input);
/// assert!(result.is_err());
/// ```
///
/// ## Parameters:
/// - `input`: A string slice representing the input to be tokenized.
///
/// ## Returns:
/// A `Result` containing a vector of `Token`s if tokenization is successful, or a `String` error
/// message if an invalid character or token is encountered.
///
/// ## Errors:
/// - Returns an error if the input contains invalid characters.
/// - Returns an error if the input contains a standalone '+' or '-' instead of '++' or '--'.
pub(crate) fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_digit(10) {
                        num.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num.parse().unwrap()));
            }
            '+' => {
                chars.next(); // Consume the first '+'
                if chars.peek() == Some(&'+') {
                    chars.next(); // Consume the second '+'
                    tokens.push(Token::Increment);
                } else {
                    return Err("Invalid token: Expected '++'".to_string());
                }
            }
            '-' => {
                chars.next(); // Consume the first '-'
                if chars.peek() == Some(&'-') {
                    chars.next(); // Consume the second '-'
                    tokens.push(Token::Decrement);
                } else {
                    return Err("Invalid token: Expected '--'".to_string());
                }
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            c if c.is_whitespace() => {
                chars.next();
            }
            _ => {
                return Err(format!("Invalid character: {}", c));
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use expectest::prelude::*;
    use super::*;

    #[test]
    fn test_tokenize() {
        // Test number tokenization
        let result = tokenize("123").unwrap();
        expect!(result).to(be_equal_to(vec![Token::Number(123)]));

        // Test increment tokenization
        let result = tokenize("++").unwrap();
        expect!(result).to(be_equal_to(vec![Token::Increment]));

        // Test decrement tokenization
        let result = tokenize("--").unwrap();
        expect!(result).to(be_equal_to(vec![Token::Decrement]));

        // Test mixed tokens
        let result = tokenize("123 ++ -- ( )").unwrap();
        expect!(result).to(be_equal_to(vec![
            Token::Number(123),
            Token::Increment,
            Token::Decrement,
            Token::LParen,
            Token::RParen,
        ]));

        // Test invalid tokenization
        let result = tokenize("++-");
        expect!(result).to(be_err());

        let result = tokenize("++ +");
        expect!(result).to(be_err());

        let result = tokenize("-- -");
        expect!(result).to(be_err());

        let result = tokenize("abc");
        expect!(result).to(be_err());
    }
}
