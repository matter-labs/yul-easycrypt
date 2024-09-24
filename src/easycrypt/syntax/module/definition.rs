use crate::easycrypt::syntax::function::Function;
use crate::easycrypt::syntax::proc::Proc;
use crate::easycrypt::syntax::Name;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopDefinition {
    Proc(Proc),
    Function(Function),
}

impl TopDefinition {
    ///
    /// Returns the name of the top level definition.
    ///
    pub fn name(&self) -> Name {
        match self {
            TopDefinition::Proc(proc) => proc.name.clone(),
            TopDefinition::Function(fun) => fun.name.clone(),
        }
    }

    /// Returns `true` if the module definition is [`ProcDef`].
    ///
    /// [`ProcDef`]: ModuleDefinition::ProcDef
    #[must_use]
    pub fn is_proc_def(&self) -> bool {
        matches!(self, Self::Proc(..))
    }

    /// Returns `true` if the module definition is [`FunDef`].
    ///
    /// [`FunDef`]: ModuleDefinition::FunDef
    #[must_use]
    pub fn is_fun_def(&self) -> bool {
        matches!(self, Self::Function(..))
    }
}
