pub mod effect;

use std::fmt::Display;

use self::effect::Effect;
use super::definition::Definition;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Type {
    Unknown,
    Unit,
    Bool,
    Integer,
    Int(usize),
    UInt(usize),
    Custom(String),
    Tuple(Vec<Type>),
    Arrow(Box<Type>, Box<Type>),
    Context(Effect),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Unit => f.write_str("unit"),
            Type::Bool => f.write_str("bool"),
            Type::Int(size) => f.write_fmt(format_args!("int{}", size)),
            Type::UInt(size) => f.write_fmt(format_args!("uint{}", size)),
            Type::Custom(name) => f.write_str(name),
            Type::Arrow(lhs, rhs) => f.write_fmt(format_args!("{}->{}", lhs, rhs)),
            Type::Tuple(inner) => {
                f.write_str("(")?;
                for (i, component) in inner.iter().enumerate() {
                    if i > 0 {
                        f.write_str(" * ")?;
                    }
                    component.fmt(f)?;
                }
                f.write_str(")")
            }
            Type::Unknown => f.write_str("Unknown"),
            Type::Context(ctx) => f.write_fmt(format_args!("{}", ctx)),
            Type::Integer => f.write_str("int"),
        }
    }
}
impl Type {
    ///
    /// Default type: currently only `UInt(256)` is used for all definitions, as
    /// this is the limitation of the current YUL dialect.
    /// Despite the similar name, this has different semantic from the
    /// [`Default`] trait.
    ///
    pub const DEFAULT: &'static Type = &Type::UInt(256);

    /// Returns either:
    /// - `Type::Unit`, if [`types`] is empty;
    /// - First type of [`types`] if there is only one type;
    /// - A tuple with all types otherwise.
    ///
    pub fn of_types(types: impl ExactSizeIterator<Item = Type>) -> Type {
        match types.len() {
            0 => Type::Unit,
            1 => types.peekable().peek().unwrap().clone(),
            _ => Type::Tuple(types.collect::<Vec<_>>()),
        }
    }

    /// Returns either:
    /// - `Type::Unit`, if [`definitions`] is empty;
    /// - The type of the first definition, if there is only one definition;
    /// - A tuple with types of all definitions otherwise.
    pub fn type_of_definitions(definitions: impl ExactSizeIterator<Item = Definition>) -> Type {
        let types = definitions.map(|d| d.get_effective_type());

        Self::of_types(types)
    }
}
