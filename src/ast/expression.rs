/// Trait representing an expression in an abstract syntax tree (AST).
///
/// The `Expression` trait is designed to be implemented by various types of expressions within an
/// AST. Each expression carries metadata of type `T`, which can be used to store additional
/// information related to the expression, such as source code locations, type annotations, or other
/// relevant data.
///
/// # Type Parameters
/// - `T`: The type of metadata associated with the expression.
///
/// # Required Methods
/// - `metadata`: Returns the metadata associated with the expression.
pub(crate) trait Expression<T> {
    /// Returns the metadata associated with the expression.
    ///
    /// # Returns
    /// - `T`: The metadata associated with the expression.
    fn metadata(&self) -> T;
}

/// Trait representing a literal expression in an abstract syntax tree (AST).
///
/// The `Literal` trait is intended to be implemented by types that represent literal values within
/// an AST. Literals are the basic building blocks of expressions, such as numeric values, strings,
/// or boolean constants. This trait can be extended to include metadata of type `T`, which can
/// provide additional context or information related to the literal expression.
///
/// # Type Parameters
/// - `T`: The type of metadata associated with the literal.
pub(crate) trait Literal<T> {}

/// A trait representing a unary expression in an abstract syntax tree (AST).
///
/// The `UnaryExpression` trait is a marker trait that can be implemented by types that represent
/// unary expressions in an abstract syntax tree (AST). Unary expressions are operations that
/// involve a single operand, such as negation, increment, or decrement operations.
///
/// ## Type Parameters:
/// - `T`: The type of the operand or the result of the unary expression. This could represent a
///   numeric type, a boolean type, or any other type that unary operations can be applied to.
pub(crate) trait UnaryExpression<T> {
    /// Returns a reference to the operand of the unary expression.
    fn expr(&self) -> &dyn Expression<T>;
}
