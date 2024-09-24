//!
//! Transpilation of YUL types.
//!

use anyhow::Error;

use super::config::IConfig;
use super::Translator;
use crate::easycrypt::syntax::r#type::Type;
use era_yul::yul::parser::r#type::Type as YulType;

impl<C> Translator<C>
where
    C: IConfig,
{
    /// Default type to fall back when the type in YUL syntax tree is unknown.
    pub const DEFAULT_TYPE: Type = Type::UInt(256);

    /// Transpile an arbitrary YUL type.
    pub fn translate_type(_type: &YulType) -> Result<Type, Error> {
        // At this time, all types are represented as U256
        Ok(Self::DEFAULT_TYPE)
    }
}
