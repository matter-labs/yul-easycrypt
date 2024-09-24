//!
//! EasyCrypt AST node containing a definition of a module.
//!

pub mod definition;

use std::collections::HashMap;

use anyhow::Result;

use crate::easycrypt::syntax::module::definition::TopDefinition;
use crate::easycrypt::syntax::Name;

///
/// EasyCrypt AST node containing a definition of a module.
///
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Module {
    /// Name of the module, derived from the name of YUL object.
    pub name: Option<Name>,
    /// Definitions belonging to the module, including functions and procedures.
    pub definitions: HashMap<Name, TopDefinition>,
    //pub dependency_order: Vec<Name>,
}

impl Module {
    ///
    /// Creates a new empty instance of [`Module`].
    ///
    pub fn new(name: Option<Name>) -> Self {
        Self {
            definitions: HashMap::new(),
            name,
            //dependency_order: Vec::new(),
        }
    }

    ///
    /// Create an anonymous module populated with given definitions.
    ///
    pub fn from_definitions<T>(definitions: T) -> Self
    where
        T: Iterator<Item = (Name, TopDefinition)>,
    {
        Self {
            name: None,
            definitions: definitions.collect(),
            //dependency_order: Vec::new(),
        }
    }

    ///
    /// Merge this module with another, nameless module.
    ///
    pub fn merge(self, other: Self) -> Result<Module> {
        if self.name.is_some() && other.name.is_some() {
            anyhow::bail!("Internal error: merging two modules requires one anonymous module and one named module");
        }
        let name = other.name.or(self.name);

        let mut result_definitions = self.definitions;
        result_definitions.extend(other.definitions);
        Ok(Module {
            name,
            definitions: result_definitions,
        })
    }

    ///
    /// Adds a new top-level definition to the module.
    ///
    pub fn add_def(&mut self, module_def: TopDefinition) {
        self.definitions.insert(module_def.name(), module_def);
    }

    pub fn names_ordered(&self) -> Vec<Name> {
        let mut result: Vec<_> = self.definitions.keys().cloned().collect();
        result.sort();
        result
    }
}

impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::easycrypt::visitor::Visitor;

        let mut out: String = String::new();
        let mut printer = crate::easycrypt::printer::ECPrinter::new(&mut out);
        printer.visit_module(self).unwrap();
        f.write_str(&out)
    }
}
