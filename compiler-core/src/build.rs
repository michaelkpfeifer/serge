#![allow(warnings)]

mod project_compiler;

pub use self::project_compiler::Built;
pub use self::project_compiler::Options;
pub use self::project_compiler::ProjectCompiler;

use strum::{Display, EnumIter, EnumString, EnumVariantNames};


#[derive(Debug, Display, EnumString, EnumVariantNames, EnumIter, Clone, Copy, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum Target {
    #[strum(serialize = "erlang", serialize = "erl")]
    Erlang,
    #[strum(serialize = "javascript", serialize = "js")]
    JavaScript,
}
