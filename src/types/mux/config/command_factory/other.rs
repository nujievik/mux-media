use super::super::cli_args::MuxConfigArg;
use super::Blocks;
use crate::{CLIArg, Msg};
use clap::{Arg, ArgAction};

impl Blocks {
    pub fn other(mut self) -> Self {
        // Help Only. This args processing in raw
        self.0 = self
            .0
            .next_help_heading(Msg::HelpOtherOptions.to_str_localized())
            .arg(
                Arg::new(MuxConfigArg::ListLangs.as_long())
                    .long(MuxConfigArg::ListLangs.as_long())
                    .help(Msg::HelpListLangs.to_str_localized())
                    .action(ArgAction::SetTrue),
            );

        #[cfg(not(all(windows, any(target_arch = "x86", target_arch = "x86_64"))))]
        {
            self.0 = self.0.arg(
                Arg::new(MuxConfigArg::UserTools.as_long())
                    .long(MuxConfigArg::UserTools.as_long())
                    .action(ArgAction::SetTrue)
                    .hide(true),
            );
        }

        #[cfg(all(windows, any(target_arch = "x86", target_arch = "x86_64")))]
        {
            self.0 = self.0.arg(
                Arg::new(MuxConfigArg::UserTools.as_long())
                    .long(MuxConfigArg::UserTools.as_long())
                    .help(Msg::HelpUserTools.to_str_localized())
                    .action(ArgAction::SetTrue),
            );
        }

        self.0 = self
            .0
            .next_help_heading(Msg::HelpOtherOptions.to_str_localized())
            .arg(
                Arg::new(MuxConfigArg::MkvmergeHelp.as_long())
                    .long(MuxConfigArg::MkvmergeHelp.as_long())
                    .help(Msg::HelpMkvmergeHelp.to_str_localized())
                    .action(ArgAction::SetTrue),
            );

        self
    }

    pub fn version(mut self) -> Self {
        self.0 = self.0.arg(
            Arg::new(MuxConfigArg::Version.as_long())
                .short('V')
                .long(MuxConfigArg::Version.as_long())
                .help_heading(Msg::HelpOtherOptions.to_str_localized())
                .help(Msg::HelpVersion.to_str_localized())
                .action(ArgAction::Version),
        );

        self
    }

    pub fn help(mut self) -> Self {
        self.0 = self.0.arg(
            Arg::new(MuxConfigArg::Help.as_long())
                .short('h')
                .long(MuxConfigArg::Help.as_long())
                .help_heading(Msg::HelpOtherOptions.to_str_localized())
                .help(Msg::HelpHelp.to_str_localized())
                .action(ArgAction::Help),
        );

        self
    }
}
