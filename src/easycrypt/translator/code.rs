//!
//! Transpilation of the `code` block of an arbitrary YUL object.
//!

use anyhow::Result;

use era_yul::yul::visitor::implicit_code_function;

use crate::easycrypt::syntax::module::definition::TopDefinition;
use crate::easycrypt::translator::config::IConfig;
use crate::yul::path::tracker::PathTracker;
use era_yul::yul::parser::statement::code::Code as YulCode;

impl<C> super::Translator<C>
where
    C: IConfig,
{
    ///
    /// Transpile the `code` block of an arbitrary YUL object.
    ///
    pub fn translate_code(&mut self, code: &YulCode<C::Dialect>) -> Result<()> {
        self.yul_context.path_tracker.enter_code();
        let implicit_function = implicit_code_function(code);
        let translated_function = self.translate_function_definition(&implicit_function)?;

        match translated_function {
            super::function::Translated::Function(_) => {
                anyhow::bail!("Internal error: The `code` section can only be translated to a procedure, but not to a function.");
            }
            super::function::Translated::Proc(proc) => {
                self.result.add_def(TopDefinition::Proc(proc))
            }
        }

        self.yul_context.path_tracker.leave();
        Ok(())
    }
}
