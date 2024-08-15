/// Represents the different types of tokens in an arithmetic expression.
///
/// The `Token` enum is used during the lexical analysis (tokenization) phase of parsing arithmetic
/// expressions. Each variant corresponds to a fundamental component of the expression, such as
/// numbers, operators, and parentheses.
#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    /// A numeric literal.
    ///
    /// This variant holds a 64-bit integer (`i64`) representing a constant value in the expression.
    /// Numeric tokens are created when the tokenizer encounters sequences of digits.
    ///
    /// # Example
    ///
    /// ```rust
    /// let token = Token::Number(42);
    /// ```
    Number(i64),

    /// The increment operator (`++`).
    ///
    /// This variant represents the increment operation in the expression. It is used to increase the
    /// value of an expression by one. This token is created when the tokenizer encounters the `++`
    /// sequence of characters.
    ///
    /// # Example
    ///
    /// ```rust
    /// let token = Token::Increment;
    /// ```
    Increment,

    /// The decrement operator (`--`).
    ///
    /// This variant represents the decrement operation in the expression. It is used to decrease the
    /// value of an expression by one. This token is created when the tokenizer encounters the `--`
    /// sequence of characters.
    ///
    /// # Example
    ///
    /// ```rust
    /// let token = Token::Decrement;
    /// ```
    Decrement,

    /// The left parenthesis (`(`).
    ///
    /// This variant represents the opening parenthesis in the expression. It is used to group
    /// expressions and control the order of operations. This token is created when the tokenizer
    /// encounters the `(` character.
    ///
    /// # Example
    ///
    /// ```rust
    /// let token = Token::LParen;
    /// ```
    LParen,

    /// The right parenthesis (`)`).
    ///
    /// This variant represents the closing parenthesis in the expression. It is used to close
    /// groups of expressions and control the order of operations. This token is created when the
    /// tokenizer encounters the `)` character.
    ///
    /// # Example
    ///
    /// ```rust
    /// let token = Token::RParen;
    /// ```
    RParen,
}
