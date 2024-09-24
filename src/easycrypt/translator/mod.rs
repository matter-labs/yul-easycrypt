//!
//! Transpiler from YUL to EasyCrypt
//!

use anyhow::Result;
use era_yul::yul::parser::identifier::Identifier;
use std::collections::HashMap;
use std::marker::PhantomData;

use era_yul::yul::parser::statement::expression::function_call::name::Name as YulName;
use era_yul::yul::parser::statement::object::Object;
use era_yul::yul::visitor::Visitor;

use crate::yul::analyzers::collect_definitions::CollectDefinitions;
use crate::yul::path::full_name::FullName as YulFullName;
use crate::yul::path::tracker::PathTracker as _;

use self::config::IConfig;
use self::context::easycrypt::ECContext;
use self::context::yul::YulContext;
use self::pass::name_sanitizer::NameSanitizer;
use self::pass::SimplePass;
use self::state::definition_info::DefinitionInfo;
use self::state::definition_info::Description;
use self::state::definition_location::DefinitionLocation;
use self::state::name_generator::NameGenerator;

use super::syntax::definition::Definition;
use super::syntax::module::Module;

pub mod code;
pub mod config;
pub mod context;
pub mod expression;
pub mod function;
pub mod object;
pub mod pass;
pub mod standard_definitions;
pub mod state;
pub mod statement;
pub mod r#type;

#[derive(Debug)]
pub struct Translator<C>
where
    C: IConfig,
{
    tmp_var_generator: NameGenerator,

    preimages: HashMap<DefinitionLocation, YulFullName>,

    ec_context: ECContext,
    yul_context: YulContext<C::Dialect>,

    config: PhantomData<C>,

    result: Module,
}

impl<C: Default> Default for Translator<C>
where
    C: IConfig,
{
    fn default() -> Self {
        Self {
            tmp_var_generator: Default::default(),
            preimages: Default::default(),
            ec_context: Default::default(),
            yul_context: Default::default(),
            config: Default::default(),
            result: Default::default(),
        }
    }
}

impl<C> Translator<C>
where
    C: IConfig,
{
    pub fn new() -> Self {
        Self::default()
    }

    fn bindings_to_definitions(
        &mut self,
        idents: &[Identifier],
    ) -> Vec<(Definition, DefinitionLocation)> {
        idents
            .iter()
            .map(|identifier| self.transpile_identifier_definition(identifier))
            .collect()
    }

    ///
    /// Translate a YUL object into an EasyCrypt module.
    ///
    pub fn translate(mut self, yul_object: &Object<C::Dialect>) -> Result<Module> {
        self.yul_context.code_definitions = {
            let mut collector = CollectDefinitions::new();
            collector.visit_object(yul_object);
            collector.result
        };

        // Load standard definitions
        for (full_name, def_info) in C::standard_definitions() {
            self.yul_context.symbols.insert(&full_name, &def_info);
        }

        self.translate_object(yul_object, /* is_root = */ true)?;
        let mut sanitizer_pass = NameSanitizer::new();
        self.result = sanitizer_pass.transform_module(&self.result)?;
        Ok(self.result)
    }

    ///
    /// Resolves a name of a custom or predefined symbol relative to the current
    /// position in the YUL code and returns a corresponding definition in
    /// EasyCrypt code.
    ///
    pub fn get_definition_location(&self, name: YulName) -> Option<&DefinitionLocation> {
        match &self.yul_context.get_definition_info(name)?.description {
            Description::Builtin(_) => None,
            Description::Custom(custom) => Some(&custom.location),
        }
    }

    ///
    /// Resolves a name of a custom or predefined symbol relative to the current
    /// position in the YUL code and returns the description of an associated
    /// EasyCrypt definition.
    ///
    pub fn get_ec_definition_info(&self, name: YulName) -> Option<&DefinitionInfo> {
        self.yul_context.get_definition_info(name)
    }

    ///
    /// Resolves a name of a custom symbol relative to the current position in
    /// the YUL code and returns the description of an associated EasyCrypt
    /// definition.
    ///
    pub fn get_ec_definition_info_by_name(
        &self,
        name: impl Into<String>,
    ) -> Option<&DefinitionInfo> {
        self.get_ec_definition_info(YulName::UserDefined(name.into()))
    }

    ///
    /// Resolves a name of a custom symbol relative to the current position in
    /// the YUL code and returns the EasyCrypt syntax tree node containing the
    /// associated EasyCrypt definition.
    ///
    pub fn get_ec_definition_by_name(&self, name: impl Into<String>) -> Option<Definition> {
        let def_info = self.get_ec_definition_info_by_name(name)?;
        if let Description::Custom(description) = &def_info.description {
            Some(Definition {
                identifier: description.location.identifier.clone(),
                r#type: Some(def_info.r#type.clone()),
            })
        } else {
            None
        }
    }

    ///
    /// Returns current position in YUL syntax tree.
    ///
    fn get_yul_path(&self) -> crate::yul::path::Path {
        self.yul_context.path_tracker.here().clone()
    }
}
