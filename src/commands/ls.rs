use crate::commands::WholeStreamCommand;
use crate::errors::ShellError;
use crate::prelude::*;

pub struct LS;

impl WholeStreamCommand for LS {
    fn run(
        &self,
        args: CommandArgs,
        registry: &CommandRegistry,
    ) -> Result<OutputStream, ShellError> {
        ls(args, registry)
    }

    fn name(&self) -> &str {
        "ls"
    }

    fn signature(&self) -> Signature {
        Signature::build("ls").optional("path", SyntaxType::Path)
    }
}

fn ls(args: CommandArgs, registry: &CommandRegistry) -> Result<OutputStream, ShellError> {
    let shell_manager = args.shell_manager.clone();
    let args = args.evaluate_once(registry)?;
    shell_manager.ls(args)
}
