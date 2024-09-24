//!
//! Transpilation of YUL objects.
//!

use anyhow::Result;

use era_yul::yul::parser::statement::object::Object;

use crate::easycrypt::path::tracker::PathTracker as _;
use crate::easycrypt::translator::config::IConfig;
use crate::yul::path::tracker::PathTracker as _;

impl<C> super::Translator<C>
where
    C: IConfig,
{
    ///
    /// Transpile an arbitrary YUL object.
    ///
    pub fn translate_object(&mut self, object: &Object<C::Dialect>, is_root: bool) -> Result<()> {
        self.yul_context
            .path_tracker
            .enter_object(&object.identifier);

        self.ec_context
            .path_tracker
            .enter_module(&object.identifier);

        self.translate_code(&object.code)?;

        if is_root {
            self.result.name = Some(object.identifier.clone())
        };

        if let Some(inner_object) = &object.inner_object {
            self.translate_object(inner_object.as_ref(), false)?;
        }

        self.yul_context.path_tracker.leave();
        self.ec_context.path_tracker.leave();

        Ok(())
    }
}
