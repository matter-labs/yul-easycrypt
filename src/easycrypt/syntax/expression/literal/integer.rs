//!
//! EasyCrypt AST node containing an integer literal in decimal form.
//!

///
/// EasyCrypt AST node containing an integer literal in decimal form.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerLiteral {
    /// Integer literal in decimal form, like `123`. Hexadecimal literals are
    /// not supported.
    pub inner: String,
}
