//!
//! Part of the translation context related to the EasyCrypt code.
//!

use std::collections::HashMap;

use crate::easycrypt::path::builder::Builder;
use crate::easycrypt::translator::state::definition_info::DefinitionInfo;
use crate::easycrypt::translator::state::definition_location::DefinitionLocation;

/// Part of the translation context related to the EasyCrypt code.
#[derive(Debug, Default)]
pub struct ECContext {
    pub definitions: HashMap<DefinitionLocation, DefinitionInfo>,
    pub path_tracker: Builder,
}

impl ECContext {
    pub fn new(
        definitions: HashMap<DefinitionLocation, DefinitionInfo>,
        path_tracker: Builder,
    ) -> Self {
        Self {
            definitions,
            path_tracker,
        }
    }
}
