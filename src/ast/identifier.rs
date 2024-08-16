use crate::ast::expression::{Expression, Literal};

/// Represents an identifier in an abstract syntax tree (AST).
///
/// The `Identifier` struct models a variable or function name in the AST.
/// It contains the name of the identifier as a `String` and associated metadata of type `T`.
///
/// # Type Parameters:
/// - `T`: The type of the metadata associated with the identifier, such as type information or
///     location data.
pub(crate) struct Identifier<T> {
    name: String,
    metadata: T,
}

impl<T> Identifier<T> {
    /// Creates a new `Identifier`.
    ///
    /// # Parameters:
    /// - `name`: A `String` representing the name of the identifier.
    /// - `metadata`: Metadata associated with this identifier, such as type information or location
    ///     in the source code.
    ///
    /// # Returns:
    /// A new `Identifier` instance.
    pub(crate) fn new(name: String, metadata: T) -> Self {
        Self { name, metadata }
    }
}

impl<T> Expression<T> for Identifier<T> {
    /// Returns a reference to the metadata associated with this identifier.
    ///
    /// # Returns:
    /// A reference to the metadata of type `T`.
    fn metadata(&self) -> &T {
        &self.metadata
    }
}

impl<T> Literal<T> for Identifier<T> {}
