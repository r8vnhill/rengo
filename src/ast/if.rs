use crate::ast::expression::Expression;

/// Represents an `if` expression in an abstract syntax tree (AST).
///
/// The `If` struct models a conditional expression with an optional `else` branch. It contains a
/// condition expression, a `then` branch that is executed if the condition is true, and an optional
/// `else` branch that is executed if the condition is false. The `metadata` field can store
/// additional information about the operation, such as its location in the source code or type
/// information.
///
/// # Type Parameters:
/// - `T`: The type of the metadata associated with the expression.
struct If<T> {
    condition: Box<dyn Expression<T>>,
    then_branch: Box<dyn Expression<T>>,
    else_branch: Option<Box<dyn Expression<T>>>,
    metadata: T,
}

impl<T> If<T> {
    /// Creates a new `If` expression.
    ///
    /// # Parameters:
    /// - `condition`: A boxed expression that represents the condition of the `if` statement.
    /// - `then_branch`: A boxed expression that represents the branch executed when the condition
    ///     is true.
    /// - `else_branch`: An optional boxed expression that represents the branch executed when the
    ///     condition is false.
    /// - `metadata`: Metadata associated with this expression, such as type information or
    ///     location.
    ///
    /// # Returns:
    /// A new `If` instance representing a conditional expression in the AST.
    pub(crate) fn new(
        condition: Box<dyn Expression<T>>,
        then_branch: Box<dyn Expression<T>>,
        else_branch: Option<Box<dyn Expression<T>>>,
        metadata: T,
    ) -> Self {
        Self {
            condition,
            then_branch,
            else_branch,
            metadata,
        }
    }
}

impl<T> Expression<T> for If<T> {
    /// Returns a reference to the metadata associated with this expression.
    ///
    /// # Returns:
    /// A reference to the metadata of type `T`.
    fn metadata(&self) -> &T {
        &self.metadata
    }
}
