/// Represents an arithmetic expression.
///
/// The `Expression` enum is used to model basic arithmetic expressions that can be compiled into
/// assembly instructions or evaluated. It supports numeric literals as well as increment and
/// decrement operations, allowing for simple arithmetic operations to be represented in a
/// structured way.
#[derive(Debug, PartialEq)]
pub(crate) enum Expression {
    /// A numeric literal.
    ///
    /// The `Number` variant holds a 64-bit integer (`i64`) that represents a constant value in the
    /// expression. This variant is used for representing explicit numbers.
    ///
    /// # Example
    ///
    /// ```rust
    /// let expr = Expression::Number(42);
    /// ```
    Number(i64),

    /// An increment operation.
    ///
    /// The `Increment` variant represents an arithmetic operation that increases the value of the
    /// contained expression by one. The expression to be incremented is stored in a `Box` to allow
    /// for recursive structures.
    ///
    /// # Example
    ///
    /// ```rust
    /// let expr = Expression::Increment(Box::new(Expression::Number(42)));
    /// ```
    Increment(Box<Expression>),

    /// A decrement operation.
    ///
    /// The `Decrement` variant represents an arithmetic operation that decreases the value of the
    /// contained expression by one. The expression to be decremented is stored in a `Box` to allow
    /// for recursive structures.
    ///
    /// # Example
    ///
    /// ```rust
    /// let expr = Expression::Decrement(Box::new(Expression::Number(42)));
    /// ```
    Decrement(Box<Expression>),
}
