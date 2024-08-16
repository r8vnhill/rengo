use crate::ast::expression::Expression;

/// Represents a `let` binding with an expression and a body in an abstract syntax tree (AST).
///
/// The `Let` struct models a `let` binding, which allows for the assignment of a value to a
/// variable and the subsequent evaluation of a body expression where the variable is available. It
/// contains the variable name, the expression assigned to that variable, the body expression where
/// the variable is used, and associated metadata of type `T`.
///
/// # Type Parameters:
/// - `T`: The type of the metadata associated with the `let` binding, such as type information or
///     location data.
struct Let<T> {
    name: String,
    expr: Box<dyn Expression<T>>,
    body: Box<dyn Expression<T>>,
    metadata: T,
}

impl<T> Let<T> {
    /// Creates a new `Let` binding with an expression and a body.
    ///
    /// # Parameters:
    /// - `name`: A `String` representing the name of the variable being bound.
    /// - `expr`: A boxed expression that represents the value being assigned to the variable.
    /// - `body`: A boxed expression that represents the body where the variable is used.
    /// - `metadata`: Metadata associated with this `let` binding, such as type information or
    ///     location in the source code.
    ///
    /// # Returns:
    /// A new `Let` instance representing the `let` binding in the AST.
    pub(crate) fn new(
        name: String,
        expr: Box<dyn Expression<T>>,
        body: Box<dyn Expression<T>>,
        metadata: T
    ) -> Self {
        Self { name, expr, body, metadata }
    }
}

impl<T> Expression<T> for Let<T> {
    /// Returns a reference to the metadata associated with this `let` binding.
    ///
    /// # Returns:
    /// A reference to the metadata of type `T`.
    fn metadata(&self) -> &T {
        &self.metadata
    }
}
