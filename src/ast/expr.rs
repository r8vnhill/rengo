/// Represents an arithmetic expression.
///
/// The `Expression` enum models basic arithmetic expressions, which can be evaluated or compiled
/// into assembly instructions. It supports numeric literals, increment, decrement operations,
/// variable identifiers, and let-bindings, allowing for a structured and flexible representation
/// of arithmetic operations and simple expressions.
#[derive(Debug, PartialEq)]
pub(crate) enum Expression {
    /// A numeric literal.
    ///
    /// The `Number` variant stores a 64-bit integer (`i64`), representing a constant numeric value
    /// within the expression. This variant is essential for representing explicit numeric values
    /// in arithmetic expressions.
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
    /// contained expression by one. The expression to be incremented is stored in a `Box`, allowing
    /// for recursive expressions such as nested operations.
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
    /// contained expression by one. The expression to be decremented is stored in a `Box`, enabling
    /// complex and nested expressions within the arithmetic operation.
    ///
    /// # Example
    ///
    /// ```rust
    /// let expr = Expression::Decrement(Box::new(Expression::Number(42)));
    /// ```
    Decrement(Box<Expression>),

    /// A variable identifier.
    ///
    /// The `Identifier` variant stores a `String` representing the name of a variable. This variant
    /// is used to reference variables in expressions, enabling more dynamic and flexible arithmetic
    /// operations that depend on variable values.
    ///
    /// # Example
    ///
    /// ```rust
    /// let expr = Expression::Identifier("x".to_string());
    /// ```
    Identifier(String),

    /// A let-binding expression.
    ///
    /// The `Let` variant represents a let-binding, where a variable is assigned an expression's value
    /// and is available within a scope. The first `String` is the variable name, the first `Box<Expression>`
    /// is the value to be bound, and the second `Box<Expression>` is the expression where the variable
    /// is available.
    ///
    /// # Example
    ///
    /// ```rust
    /// let expr = Expression::Let(
    ///     "x".to_string(),
    ///     Box::new(Expression::Number(42)),
    ///     Box::new(Expression::Identifier("x".to_string()))
    /// );
    /// ```
    Let(String, Box<Expression>, Box<Expression>),
}
