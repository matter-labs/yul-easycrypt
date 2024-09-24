//!
//! YUL to EasyCrypt transpiler
//!

#![allow(non_camel_case_types)]
// #![allow(clippy::upper_case_acronyms)]
// #![allow(clippy::enum_variant_names)]
// #![allow(clippy::too_many_arguments)]
// #![allow(clippy::should_implement_trait)]

pub mod easycrypt;

// pub use self::easycrypt::printer::ECPrinter;
// pub use self::easycrypt::translator::Translator;
// pub use self::easycrypt::visitor::Visitor as ECVisitor;
pub use era_yul::util::printer::write_printer::WritePrinter;
pub use era_yul::yul::visitor::Visitor as YulVisitor;

pub mod data_structures;
pub mod util;
pub mod yul;
