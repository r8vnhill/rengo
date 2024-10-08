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
            '0'..='9' => tokens.push(parse_number(&mut chars)?),
            '-' => tokens.push(parse_minus(&mut chars)?),
            '+' => tokens.push(parse_plus(&mut chars)?),
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            '=' => {
                tokens.push(Token::Assign);
                chars.next();
            }
            ';' => {
                tokens.push(Token::LineEnd);
                chars.next();
            }
            c if c.is_whitespace() => {
                chars.next(); // Skip whitespace
            }
            c if c.is_alphabetic() => tokens.push(parse_identifier_or_keyword(&mut chars)),
            _ => return Err(format!("Invalid character: {}", c)),
        }
    }

    Ok(tokens)
}

fn parse_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, String> {
    let mut num = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_digit(10) {
            num.push(c);
            chars.next();
        } else {
            break;
        }
    }

    // After parsing the number, check if the next character is alphabetic.
    if let Some(&next_char) = chars.peek() {
        if next_char.is_alphabetic() {
            return Err(format!(
                "Invalid sequence: Number '{}' followed by identifier starting with '{}'",
                num, next_char
            ));
        }
    }

    Ok(Token::Number(num.parse().unwrap()))
}

fn parse_minus(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, String> {
    chars.next(); // Consume the first '-'
    if chars.peek() == Some(&'-') {
        chars.next(); // Consume the second '-'
        Ok(Token::Decrement)
    } else if let Some('0'..='9') = chars.peek() {
        // Handle negative number
        let mut num = String::from("-");
        while let Some(&c) = chars.peek() {
            if c.is_digit(10) {
                num.push(c);
                chars.next();
            } else {
                break;
            }
        }
        Ok(Token::Number(num.parse().unwrap()))
    } else {
        Err("Invalid token: Expected '--' or a number".to_string())
    }
}

fn parse_plus(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, String> {
    chars.next(); // Consume the first '+'
    if chars.peek() == Some(&'+') {
        chars.next(); // Consume the second '+'
        Ok(Token::Increment)
    } else {
        Err("Invalid token: Expected '++'".to_string())
    }
}

fn parse_identifier_or_keyword(chars: &mut std::iter::Peekable<std::str::Chars>) -> Token {
    let mut identifier = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_alphanumeric() || c == '_' {
            identifier.push(c);
            chars.next();
        } else {
            break;
        }
    }
    match identifier.as_str() {
        "let" => Token::Let,
        _ => Token::Identifier(identifier),
    }
}

#[cfg(test)]
mod tests {
    use expectest::prelude::*;
    use proptest::prelude::*;
    use super::*;

    mod parse_number {
        use super::*;

        proptest!(
            #[test]
            fn parses_any_number(n in 0i64..1000) {
                let input = n.to_string();
                let result = parse_number(&mut input.chars().peekable()).unwrap();
                prop_assert_eq!(result, Token::Number(n));
            }
        );
    }

    mod parse_minus {
        use super::*;

        #[test]
        fn parses_decrement() {
            let input = "--";
            let result = parse_minus(&mut input.chars().peekable()).unwrap();
            expect!(result).to(be_equal_to(Token::Decrement));
        }

        #[test]
        fn parses_negative_number() {
            let input = "-123";
            let result = parse_minus(&mut input.chars().peekable()).unwrap();
            expect!(result).to(be_equal_to(Token::Number(-123)));
        }

        #[test]
        fn fails_on_invalid_token() {
            let input = "-+";
            let result = parse_minus(&mut input.chars().peekable());
            expect!(result).to(be_err());
        }
    }

    mod parse_plus {
        use super::*;

        #[test]
        fn parses_increment() {
            let input = "++";
            let result = parse_plus(&mut input.chars().peekable()).unwrap();
            expect!(result).to(be_equal_to(Token::Increment));
        }

        #[test]
        fn fails_on_invalid_token() {
            let input = "+-";
            let result = parse_plus(&mut input.chars().peekable());
            expect!(result).to(be_err());
        }
    }

    mod parse_identifier_or_keyword {
        use super::*;

        #[test]
        fn parses_let_keyword() {
            let input = "let";
            let result = parse_identifier_or_keyword(&mut input.chars().peekable());
            expect!(result).to(be_equal_to(Token::Let));
        }

        #[test]
        fn parses_identifier() {
            let input = "foo";
            let result = parse_identifier_or_keyword(&mut input.chars().peekable());
            expect!(result).to(be_equal_to(Token::Identifier("foo".to_string())));
        }
    }

    mod tokenize {
        use super::*;

        #[test]
        fn tokenizes_numbers() {
            let input = "123";
            let result = tokenize(input);
            expect!(result).to(be_ok().value(vec![Token::Number(123)]));
        }

        #[test]
        fn tokenizes_increment() {
            let input = "++";
            let result = tokenize(input);
            expect!(result).to(be_ok().value(vec![Token::Increment]));
        }

        #[test]
        fn tokenizes_decrement() {
            let input = "--";
            let result = tokenize(input);
            expect!(result).to(be_ok().value(vec![Token::Decrement]));
        }

        #[test]
        fn tokenizes_parentheses() {
            let input = "()";
            let result = tokenize(input);
            expect!(result).to(be_ok().value(vec![Token::LParen, Token::RParen]));
        }

        #[test]
        fn tokenizes_assignment() {
            let input = "=";
            let result = tokenize(input);
            expect!(result).to(be_ok().value(vec![Token::Assign]));
        }

        #[test]
        fn tokenizes_line_end() {
            let input = ";";
            let result = tokenize(input);
            expect!(result).to(be_ok().value(vec![Token::LineEnd]));
        }

        #[test]
        fn ignores_whitespace() {
            let input = "  123  ";
            let result = tokenize(input);
            expect!(result).to(be_ok().value(vec![Token::Number(123)]));
        }

        #[test]
        fn tokenizes_identifiers() {
            let input = "foo";
            let result = tokenize(input);
            expect!(result).to(be_ok().value(vec![Token::Identifier("foo".to_string())]));
        }

        #[test]
        fn tokenizes_let_keyword() {
            let input = "let";
            let result = tokenize(input);
            expect!(result).to(be_ok().value(vec![Token::Let]));
        }

        #[test]
        fn fails_on_invalid_character() {
            let input = "!";
            let result = tokenize(input);
            expect!(result).to(be_err());
        }

        #[test]
        fn fails_on_invalid_increment() {
            let input = "+";
            let result = tokenize(input);
            expect!(result).to(be_err());
        }

        #[test]
        fn fails_on_invalid_decrement() {
            let input = "-";
            let result = tokenize(input);
            expect!(result).to(be_err());
        }

        #[test]
        fn fails_on_invalid_number() {
            let input = "123a";
            let result = tokenize(input);
            expect!(result).to(be_err());
        }
    }
}
