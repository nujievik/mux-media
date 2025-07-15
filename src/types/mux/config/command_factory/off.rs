use super::super::cli_args::MuxConfigArg;
use super::Blocks;
use crate::{CLIArg, Msg};
use clap::{Arg, ArgAction};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

impl Blocks {
    pub fn off(mut self) -> Self {
        let mut cmd = self
            .0
            .next_help_heading(Msg::HelpOffOnProOptions.to_str_localized());

        for flag in OffFlag::iter() {
            let help_arg = flag.to_help_arg();
            let arg = flag.to_arg();
            let no_arg = flag.to_no_arg();

            cmd = cmd
                .arg(
                    Arg::new(help_arg.as_long())
                        .long(help_arg.as_long())
                        .help(flag.to_help())
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new(arg.as_long())
                        .long(arg.as_long())
                        .action(ArgAction::SetTrue)
                        .hide(true),
                )
                .arg(
                    Arg::new(no_arg.as_long())
                        .long(no_arg.as_long())
                        .action(ArgAction::SetTrue)
                        .hide(true)
                        .conflicts_with(arg.as_long()),
                );
        }

        self.0 = cmd;
        self
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
enum OffFlag {
    AddDefaults,
    AddForceds,
    AddEnableds,
    AddNames,
    AddLangs,
    AddCharsets,
}

impl OffFlag {
    fn to_help_arg(self) -> MuxConfigArg {
        match self {
            Self::AddDefaults => MuxConfigArg::HelpAddDefaults,
            Self::AddForceds => MuxConfigArg::HelpAddForceds,
            Self::AddEnableds => MuxConfigArg::HelpAddEnableds,
            Self::AddNames => MuxConfigArg::HelpAddNames,
            Self::AddLangs => MuxConfigArg::HelpAddLangs,
            Self::AddCharsets => MuxConfigArg::HelpAddCharsets,
            //Self::SortFonts => MuxConfigArg::HelpSortFonts,
        }
    }

    fn to_arg(self) -> MuxConfigArg {
        match self {
            Self::AddDefaults => MuxConfigArg::AddDefaults,
            Self::AddForceds => MuxConfigArg::AddForceds,
            Self::AddEnableds => MuxConfigArg::AddEnableds,
            Self::AddNames => MuxConfigArg::AddNames,
            Self::AddLangs => MuxConfigArg::AddLangs,
            Self::AddCharsets => MuxConfigArg::AddCharsets,
            //Self::SortFonts => MuxConfigArg::SortFonts,
        }
    }

    fn to_no_arg(self) -> MuxConfigArg {
        match self {
            Self::AddDefaults => MuxConfigArg::NoAddDefaults,
            Self::AddForceds => MuxConfigArg::NoAddForceds,
            Self::AddEnableds => MuxConfigArg::NoAddEnableds,
            Self::AddNames => MuxConfigArg::NoAddNames,
            Self::AddLangs => MuxConfigArg::NoAddLangs,
            Self::AddCharsets => MuxConfigArg::NoAddCharsets,
            //Self::SortFonts => MuxConfigArg::NoSortFonts,
        }
    }

    fn to_help(self) -> &'static str {
        match self {
            Self::AddDefaults => Msg::HelpAddDefaults,
            Self::AddForceds => Msg::HelpAddForceds,
            Self::AddEnableds => Msg::HelpAddEnableds,
            Self::AddNames => Msg::HelpAddNames,
            Self::AddLangs => Msg::HelpAddLangs,
            Self::AddCharsets => Msg::HelpAddCharsets,
        }
        .to_str_localized()
    }
}
