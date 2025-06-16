use super::Blocks;
use crate::types::off_on_pro::{OffOnPro, OffOnProArg};
use crate::{CLIArg, CLIArgs};
use clap::{Arg, ArgAction};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

impl Blocks {
    pub fn off(mut self) -> Self {
        let mut cmd = self.0.next_help_heading("Off on Pro options");

        for flag in OffFlag::iter() {
            let help_arg = flag.to_help_arg();
            let arg = flag.to_arg();
            let no_arg = flag.to_no_arg();

            cmd = cmd
                .arg(
                    Arg::new(help_arg.as_long())
                        .long(help_arg.as_long())
                        .help(help_arg.help())
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

#[derive(EnumIter)]
enum OffFlag {
    AddDefaults,
    AddForceds,
    AddEnableds,
    AddNames,
    AddLangs,
    SortFonts,
}

impl OffFlag {
    fn to_help_arg(&self) -> <OffOnPro as CLIArgs>::Arg {
        match self {
            Self::AddDefaults => OffOnProArg::HelpAddDefaults,
            Self::AddForceds => OffOnProArg::HelpAddForceds,
            Self::AddEnableds => OffOnProArg::HelpAddEnableds,
            Self::AddNames => OffOnProArg::HelpAddNames,
            Self::AddLangs => OffOnProArg::HelpAddLangs,
            Self::SortFonts => OffOnProArg::HelpSortFonts,
        }
    }

    fn to_arg(&self) -> <OffOnPro as CLIArgs>::Arg {
        match self {
            Self::AddDefaults => OffOnProArg::AddDefaults,
            Self::AddForceds => OffOnProArg::AddForceds,
            Self::AddEnableds => OffOnProArg::AddEnableds,
            Self::AddNames => OffOnProArg::AddNames,
            Self::AddLangs => OffOnProArg::AddLangs,
            Self::SortFonts => OffOnProArg::SortFonts,
        }
    }

    fn to_no_arg(&self) -> <OffOnPro as CLIArgs>::Arg {
        match self {
            Self::AddDefaults => OffOnProArg::NoAddDefaults,
            Self::AddForceds => OffOnProArg::NoAddForceds,
            Self::AddEnableds => OffOnProArg::NoAddEnableds,
            Self::AddNames => OffOnProArg::NoAddNames,
            Self::AddLangs => OffOnProArg::NoAddLangs,
            Self::SortFonts => OffOnProArg::NoSortFonts,
        }
    }
}

impl OffOnProArg {
    fn help(&self) -> &'static str {
        match self {
            Self::HelpAddDefaults => "On/Off auto set default-track-flags",
            Self::HelpAddForceds => "On/Off auto set forced-display-flags",
            Self::HelpAddEnableds => "On/Off auto set track-enabled-flags",
            Self::HelpAddNames => "On/Off auto set track-names",
            Self::HelpAddLangs => "On/Off auto set track-languages",
            Self::HelpSortFonts => "On/Off sort in-files fonts",
            _ => panic!("Received unsupported fn help() arg"),
        }
    }
}
