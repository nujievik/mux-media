use super::Blocks;
use crate::{Msg, MuxConfigArg, ParseableArg};
use clap::{Arg, ArgAction};

impl Blocks {
    pub fn other(mut self) -> Self {
        // Help Only. This args processing in raw
        self.0 = self
            .0
            .next_help_heading(Msg::HelpOtherOptions.to_str_localized())
            .arg(
                Arg::new(MuxConfigArg::ListLangs.undashed())
                    .long(MuxConfigArg::ListLangs.undashed())
                    .help(Msg::HelpListLangs.to_str_localized())
                    .action(ArgAction::SetTrue),
            );

        #[cfg(not(all(
            feature = "with_embedded_bins",
            windows,
            any(target_arch = "x86", target_arch = "x86_64")
        )))]
        {
            self.0 = self.0.arg(
                Arg::new(MuxConfigArg::UserTools.undashed())
                    .long(MuxConfigArg::UserTools.undashed())
                    .action(ArgAction::SetTrue)
                    .hide(true),
            );
        }

        #[cfg(all(
            feature = "with_embedded_bins",
            windows,
            any(target_arch = "x86", target_arch = "x86_64")
        ))]
        {
            self.0 = self.0.arg(
                Arg::new(MuxConfigArg::UserTools.undashed())
                    .long(MuxConfigArg::UserTools.undashed())
                    .help(Msg::HelpUserTools.to_str_localized())
                    .action(ArgAction::SetTrue),
            );
        }

        self.0 = self
            .0
            .next_help_heading(Msg::HelpOtherOptions.to_str_localized())
            .arg(
                Arg::new(MuxConfigArg::MkvmergeHelp.undashed())
                    .long(MuxConfigArg::MkvmergeHelp.undashed())
                    .help(Msg::HelpMkvmergeHelp.to_str_localized())
                    .action(ArgAction::SetTrue),
            );

        self
    }

    pub fn version(mut self) -> Self {
        self.0 = self.0.arg(
            Arg::new(MuxConfigArg::Version.undashed())
                .short('V')
                .long(MuxConfigArg::Version.undashed())
                .help_heading(Msg::HelpOtherOptions.to_str_localized())
                .help(Msg::HelpVersion.to_str_localized())
                .action(ArgAction::Version),
        );

        self
    }

    pub fn help(mut self) -> Self {
        self.0 = self.0.arg(
            Arg::new(MuxConfigArg::Help.undashed())
                .short('h')
                .long(MuxConfigArg::Help.undashed())
                .help_heading(Msg::HelpOtherOptions.to_str_localized())
                .help(Msg::HelpHelp.to_str_localized())
                .action(ArgAction::Help),
        );

        self
    }
}
