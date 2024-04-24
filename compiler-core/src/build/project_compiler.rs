use crate::build::Target;
use crate::Result;

#[derive(Debug)]
pub struct Options {
    pub target: Option<Target>,
    pub warnings_as_errors: bool,
}

#[derive(Debug)]
pub struct Built {}

#[derive(Debug)]
pub struct ProjectCompiler {
    options: Options,
}

impl ProjectCompiler {
    pub fn new(options: Options) -> Self {
        Self { options }
    }

    pub fn compile(mut self) -> Result<Built> {
        Ok(Built {})
    }
}
