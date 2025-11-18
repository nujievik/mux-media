use super::Blocks;
use crate::{Msg, Tool, undashed};
use clap::{Arg, ArgAction};

const HIDE_TOOL_ARG: bool = !cfg!(feature = "static");

impl Blocks {
    pub fn other(mut self) -> Self {
        self.0 = self
            .0
            .next_help_heading(Msg::HelpOtherOptions.as_str_localized())
            .arg(
                Arg::new(undashed!(ListContainers))
                    .long(undashed!(ListContainers))
                    .help(Msg::HelpListContainers.as_str_localized())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new(undashed!(ListLangs))
                    .long(undashed!(ListLangs))
                    .alias(undashed!(ListLanguages))
                    .help(Msg::HelpListLangs.as_str_localized())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new(undashed!(ListLangsFull))
                    .long(undashed!(ListLangsFull))
                    .help(Msg::HelpListLangsFull.as_str_localized())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new(undashed!(Sys))
                    .long(undashed!(Sys))
                    .help(Msg::HelpSys.as_str_localized())
                    .hide(HIDE_TOOL_ARG)
                    .action(ArgAction::SetTrue),
            );

        for t in Tool::iter() {
            self = self.tool_arg(t);
        }

        self
    }

    pub fn version(mut self) -> Self {
        self.0 = self.0.arg(
            Arg::new(undashed!(Version))
                .short('V')
                .long(undashed!(Version))
                .help_heading(Msg::HelpOtherOptions.as_str_localized())
                .help(Msg::HelpVersion.as_str_localized())
                .action(ArgAction::SetTrue),
        );

        self
    }

    pub fn help(mut self) -> Self {
        self.0 = self.0.arg(
            Arg::new(undashed!(Help))
                .short('h')
                .long(undashed!(Help))
                .help_heading(Msg::HelpOtherOptions.as_str_localized())
                .help(Msg::HelpHelp.as_str_localized())
                .action(ArgAction::SetTrue),
        );

        self
    }
}

impl Blocks {
    fn tool_arg(mut self, tool: Tool) -> Blocks {
        let arg = tool.as_cli_arg();
        self.0 = self.0.arg(
            Arg::new(arg.undashed())
                .long(arg.undashed())
                .value_name("options")
                .help(Msg::RunCommand.as_str_localized())
                .hide(HIDE_TOOL_ARG)
                .trailing_var_arg(true)
                .allow_hyphen_values(true)
                .num_args(..),
        );

        self
    }
}
