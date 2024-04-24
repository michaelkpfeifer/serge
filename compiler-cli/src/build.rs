use serge_core::Result;
use serge_core::build::Built;
use serge_core::build::Options;
use serge_core::build::ProjectCompiler;

pub fn main(options: Options) -> Result<Built> {
    let result = {
        let compiler = ProjectCompiler::new(options);
        compiler.compile()?
    };

    Ok(result)
}
