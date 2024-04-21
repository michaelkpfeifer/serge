use strum::{Display, EnumIter, EnumString};

#[derive(Debug, Display, EnumString, EnumIter, Clone, Copy, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum Target {
    #[strum(serialize = "erlang", serialize = "erl")]
    Erlang,
    #[strum(serialize = "javascript", serialize = "js")]
    JavaScript,
}
