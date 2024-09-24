//!
//! YUL-related part of translation context.
//!

use std::collections::HashMap;

use era_yul::yul::parser::dialect::Dialect;
use era_yul::yul::parser::statement::expression::function_call::name::Name;

use crate::easycrypt::translator::state::definition_info::DefinitionInfo;
use crate::yul::analyzers::Definition;
use crate::yul::path::builder::Builder;
use crate::yul::path::full_name::FullName;
use crate::yul::path::symbol_table::SymbolTable;
use crate::yul::path::tracker::PathTracker;

#[derive(Debug)]
pub struct YulContext<D>
where
    D: Dialect,
{
    pub path_tracker: Builder,
    pub symbols: SymbolTable<DefinitionInfo>,
    pub code_definitions: HashMap<FullName, Definition<D>>,
}

impl<D> YulContext<D>
where
    D: Dialect,
{
    ///
    /// Returns a definition point for the matching EasyCrypt definition.
    ///
    pub fn get_definition_info(&self, name: Name) -> Option<&DefinitionInfo> {
        let full_name = FullName {
            name,
            path: self.path_tracker.here().clone(),
        };
        self.symbols.get(&full_name)
    }
}

impl<D> Default for YulContext<D>
where
    D: Dialect,
{
    fn default() -> Self {
        Self {
            path_tracker: Default::default(),
            symbols: Default::default(),
            code_definitions: Default::default(),
        }
    }
}
