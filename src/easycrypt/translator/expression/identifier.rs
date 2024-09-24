use anyhow::anyhow;
use anyhow::Result;
use era_yul::yul::parser::identifier::Identifier as YulIdentifier;
use era_yul::yul::parser::statement::expression::function_call::name::Name as YulName;

use crate::easycrypt::path::tracker::PathTracker as _;
use crate::easycrypt::syntax::definition::Definition;
use crate::easycrypt::syntax::r#type::Type;
use crate::easycrypt::syntax::reference::Reference;
use crate::easycrypt::translator::config::IConfig;
use crate::easycrypt::translator::state::definition_info::DefinitionInfo;
use crate::easycrypt::translator::state::definition_info::Description;
use crate::easycrypt::translator::state::definition_location::DefinitionLocation;
use crate::easycrypt::translator::Translator;

use crate::yul::path::full_name::FullName;
use crate::yul::path::tracker::PathTracker as _;

impl<C> Translator<C>
where
    C: IConfig,
{
    fn new_definition_here(
        &mut self,
        name: &str,
        r#type: Type,
        yul_name: &str,
    ) -> (Definition, DefinitionLocation) {
        let ec_path = self.ec_context.path_tracker.here().clone();

        let yul_full_name = FullName {
            name: YulName::UserDefined(String::from(yul_name)),
            path: self.yul_context.path_tracker.here().clone(),
        };

        let new_definition = DefinitionLocation {
            identifier: String::from(name),
            path: ec_path,
        };

        let def_info = DefinitionInfo::variable(&new_definition, &r#type);

        self.yul_context.symbols.insert(&yul_full_name, &def_info);

        self.ec_context
            .definitions
            .insert(new_definition.clone(), def_info);

        self.preimages.insert(new_definition.clone(), yul_full_name);
        (
            Definition::new(new_definition.identifier.clone(), Some(r#type)),
            new_definition,
        )
    }

    pub fn new_tmp_definition_here(&mut self, r#type: Type) -> (Definition, DefinitionLocation) {
        let name = self.tmp_var_generator.new_variable();
        self.new_definition_here(&name, r#type, &name)
    }

    pub fn transpile_identifier_reference(
        &mut self,
        identifier: &YulIdentifier,
    ) -> Result<Reference> {
        let yul_full_name = FullName::custom(&identifier.inner, self.get_yul_path());

        let def_info = self.yul_context.symbols.get(&yul_full_name).ok_or(anyhow!(
            "Internal error: the symbol '{}' is supposed to be defined prior to referencing it.",
            &identifier.inner
        ))?;
        match &def_info.description {
            Description::Builtin(builtin) => {
                anyhow::bail!("Internal error: attempt to take a reference to the image of the symbol {}, which is a builtin {:?}", &identifier.inner, builtin)
            }
            Description::Custom(custom) => Ok(custom
                .location
                .reference(self.ec_context.path_tracker.here())),
        }
    }

    pub fn transpile_identifier_definition(
        &mut self,
        identifier: &YulIdentifier,
    ) -> (Definition, DefinitionLocation) {
        self.new_definition_here(
            &identifier.inner,
            identifier
                .r#type
                .as_ref()
                .and_then(|t| Self::translate_type(t).ok())
                .unwrap_or(Type::DEFAULT.clone()),
            &identifier.inner,
        )
    }
}
