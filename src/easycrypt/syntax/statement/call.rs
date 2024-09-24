//!
//! An EasyCrypt AST Node containing a procedure call, which is a kind of a statement.
//!

use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::reference::Reference;

/// An EasyCrypt AST Node containing a procedure call, which is a kind of a statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcCall {
    /// Which procedure to call.
    pub target: Reference,
    /// Arguments for the call.
    pub arguments: Vec<Expression>,
}
