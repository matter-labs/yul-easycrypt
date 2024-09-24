//!
//! EasyCrypt AST node containing a call to a function (not procedure).
//!

use crate::easycrypt::syntax::{expression::Expression, reference::Reference};

///
/// EasyCrypt AST node containing a call to a function (not procedure).
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionCall {
    pub target: Reference,
    pub arguments: Vec<Expression>,
}
