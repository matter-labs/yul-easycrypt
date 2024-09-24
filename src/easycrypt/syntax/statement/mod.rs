//!
//! EasyCrypt AST nodes containing statements. Statements are a syntactic
//! category whose computations result in actions, potentially with
//! side-effects.
//!

pub mod block;
pub mod call;
pub mod if_conditional;
pub mod while_loop;

use self::block::Block;
use self::call::ProcCall;
use self::if_conditional::IfConditional;
use self::while_loop::WhileLoop;

use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::reference::Reference;

///
/// EasyCrypt AST nodes containing statements. Statements are a syntactic
/// category whose computations result in actions, potentially with
/// side-effects.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    /// Compute a single expression and discard its result.
    Expression(Expression),
    /// Block of sequentially executed statements between { curly braces }.
    Block(Block),
    /// `if` statement, conditionally executed depending on the expression value.
    IfConditional(IfConditional),
    /// Assignment of an expression evaluation result to a variable: `x <- expr`.
    EAssignment(Vec<Reference>, Box<Expression>),
    /// Assignment of a procedure return value to a variable: `x <@ proc`.
    PAssignment(Vec<Reference>, ProcCall),
    /// Return a value from a procedure.
    Return(Expression),
    /// Execute a block of statements while an expression value is true.
    WhileLoop(WhileLoop),
}

impl Statement {
    /// Returns `true` if the statement is [`Expression`].
    ///
    /// [`Expression`]: Statement::Expression
    #[must_use]
    pub fn is_expression(&self) -> bool {
        matches!(self, Self::Expression(..))
    }

    /// Returns `true` if the statement is [`Block`].
    ///
    /// [`Block`]: Statement::Block
    #[must_use]
    pub fn is_block(&self) -> bool {
        matches!(self, Self::Block(..))
    }

    /// Returns `true` if the statement is [`If`].
    ///
    /// [`If`]: Statement::If
    #[must_use]
    pub fn is_if(&self) -> bool {
        matches!(self, Self::IfConditional(..))
    }

    /// Returns `true` if the statement is [`EAssignment`].
    ///
    /// [`EAssignment`]: Statement::EAssignment
    #[must_use]
    pub fn is_eassignment(&self) -> bool {
        matches!(self, Self::EAssignment(..))
    }

    /// Returns `true` if the statement is [`PAssignment`].
    ///
    /// [`PAssignment`]: Statement::PAssignment
    #[must_use]
    pub fn is_passignment(&self) -> bool {
        matches!(self, Self::PAssignment(..))
    }

    /// Returns `true` if the statement is [`Return`].
    ///
    /// [`Return`]: Statement::Return
    #[must_use]
    pub fn is_return(&self) -> bool {
        matches!(self, Self::Return(..))
    }

    /// Returns `true` if the statement is [`WhileLoop`].
    ///
    /// [`WhileLoop`]: Statement::WhileLoop
    #[must_use]
    pub fn is_while_loop(&self) -> bool {
        matches!(self, Self::WhileLoop(..))
    }

    pub fn as_if(&self) -> Option<&IfConditional> {
        if let Self::IfConditional(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
