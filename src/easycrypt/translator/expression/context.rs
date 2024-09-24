//!
//! Expression translation context.
//!

use crate::easycrypt::syntax::definition::Definition;
use crate::easycrypt::syntax::reference::Reference;
use crate::easycrypt::syntax::statement::call::ProcCall;
use crate::easycrypt::syntax::statement::Statement;

///
/// Expression translation context.
///
#[derive(Clone, Debug, Default)]
pub struct Context {
    ///
    /// When the root expression is finished translating, these statements will be
    /// prepended to the currently translated statement. Usualy, these
    /// statements are assignments to temporary variables.
    ///
    pub statements: Vec<Statement>,
    ///
    /// When the root expression is finished translating, these definitions
    /// will be appended to the current context; eventually, the corresponding
    /// variable definitions will be emitted in the parent procedure.
    ///
    pub locals: Vec<Definition>,
}

impl Context {
    ///
    /// Creates a new instance of [`Context`] with an empty state.
    ///
    pub fn new() -> Self {
        Self::default()
    }

    ///
    /// Add a new assignment to the context. When the root expression is
    /// finished translating, all such assignments will be prepended to the
    /// currently translated statement.
    ///
    pub fn add_assignment(&mut self, new_definition: &Definition, rhs: ProcCall) {
        self.statements.push(Statement::PAssignment(
            vec![new_definition.reference()],
            rhs,
        ));
        self.locals.push(new_definition.clone())
    }

    ///
    /// Add an assignment to
    ///
    pub fn add_multiple_assignment(&mut self, references: &[Reference], rhs: ProcCall) {
        self.statements
            .push(Statement::PAssignment(references.to_vec(), rhs));
    }
}
