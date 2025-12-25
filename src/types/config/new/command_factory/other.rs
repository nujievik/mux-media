use super::Blocks;
use crate::{Msg, undashed};
use clap::{Arg, ArgAction};

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
                    .alias("list-languages")
                    .help(Msg::HelpListLangs.as_str_localized())
                    .action(ArgAction::SetTrue),
            );

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
