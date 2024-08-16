use crate::ast::expression::{Expression, UnaryExpression};

/// Represents a decrement operation in an abstract syntax tree (AST).
///
/// The `Decrement` struct wraps an expression and applies a decrement operation to it.
/// The `metadata` field can store additional information about the operation, such as its location
/// in the source code or type information.
///
/// # Type Parameters:
/// - `T`: The type of the metadata associated with the expression.
pub(crate) struct Decrement<T> {
    expr: Box<dyn Expression<T>>,
    metadata: T,
}

impl<T> Decrement<T> {
    /// Creates a new `Decrement` expression.
    ///
    /// # Parameters:
    /// - `expr`: A boxed expression to which the decrement operation will be applied.
    /// - `metadata`: Metadata associated with this expression, such as type information or
    ///     location.
    ///
    /// # Returns:
    /// A new `Decrement` instance.
    pub(crate) fn new(expr: Box<dyn Expression<T>>, metadata: T) -> Self {
        Self { expr, metadata }
    }
}

impl<T> Expression<T> for Decrement<T> {
    /// Returns a reference to the metadata associated with this expression.
    ///
    /// # Returns:
    /// A reference to the metadata of type `T`.
    fn metadata(&self) -> &T {
        &self.metadata
    }
}

impl<T> UnaryExpression<T> for Decrement<T> {
    /// Returns a reference to the wrapped expression.
    ///
    /// # Returns:
    /// A reference to the expression that this `Decrement` operation is applied to.
    fn expr(&self) -> &dyn Expression<T> {
        self.expr.as_ref()
    }
}
