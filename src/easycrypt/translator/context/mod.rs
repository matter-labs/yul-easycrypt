//!
//! Transpilation context.
//!

// FIXME: move to a dedicated module

pub mod easycrypt;
pub mod yul;

use anyhow::Result;
use std::iter;

use crate::easycrypt::syntax::definition::Definition;

///
/// Collects the result of the translation and the locals to be emitted in the
/// currently translated procedure.
///
#[derive(Clone, Debug)]
pub struct StatementContext {
    ///
    /// In EasyCrypt, variables are only defined in the procedure scope.
    /// In YUL, variables can be defined in the block scope as well.
    /// Therefore, the translator needs to recursively collect all definitions
    /// of YUL variables from the inner lexical scopes of a YUL function, then
    /// emit all these definitions in the EasyCrypt procedure scope.
    /// This field is used to collect the locals defined in the current function.
    ///
    pub locals: Vec<Definition>,
}

impl StatementContext {
    /// Creates a new empty context.
    pub fn new() -> StatementContext {
        StatementContext { locals: vec![] }
    }

    pub fn merge(self, other: StatementContext) -> Result<StatementContext> {
        let StatementContext { locals } = self;
        let StatementContext {
            locals: other_locals,
        } = other;
        let new_locals = locals.iter().chain(other_locals.iter()).cloned().collect();
        Ok(StatementContext { locals: new_locals })
    }

    pub fn add_locals<'a, I>(&mut self, definitions: I)
    where
        I: IntoIterator<Item = &'a Definition>,
    {
        self.locals.extend(definitions.into_iter().cloned());
    }

    pub fn add_local(&mut self, definition: Definition) {
        self.locals.push(definition)
    }

    pub fn with_locals<'a, I>(&self, definitions: I) -> Self
    where
        I: IntoIterator<Item = &'a Definition>,
    {
        Self {
            locals: self
                .locals
                .iter()
                .cloned()
                .chain(definitions.into_iter().cloned())
                .collect(),
        }
    }

    #[allow(dead_code)]
    pub fn with_local(&self, definition: Definition) -> Self {
        self.with_locals(iter::once(&definition))
    }

    pub fn clear_locals(&self) -> StatementContext {
        Self { locals: vec![] }
    }
}

impl Default for StatementContext {
    fn default() -> Self {
        Self::new()
    }
}
