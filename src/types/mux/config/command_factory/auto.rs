use super::super::cli_args::MuxConfigArg;
use super::Blocks;
use crate::{CLIArg, Msg};
use clap::{Arg, ArgAction};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

impl Blocks {
    pub fn auto(mut self) -> Self {
        let mut cmd = self
            .0
            .next_help_heading(Msg::HelpAutoFlags.to_str_localized())
            .arg(
                Arg::new(MuxConfigArg::Pro.as_long())
                    .short('p')
                    .long(MuxConfigArg::Pro.as_long())
                    .alias("pro-mode")
                    .help(Msg::HelpPro.to_str_localized())
                    .action(ArgAction::SetTrue),
            );

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
    AutoDefaults,
    AutoForceds,
    AutoEnableds,
    AutoNames,
    AutoLangs,
    AutoCharsets,
}

impl OffFlag {
    fn to_help_arg(self) -> MuxConfigArg {
        match self {
            Self::AutoDefaults => MuxConfigArg::HelpAutoDefaults,
            Self::AutoForceds => MuxConfigArg::HelpAutoForceds,
            Self::AutoEnableds => MuxConfigArg::HelpAutoEnableds,
            Self::AutoNames => MuxConfigArg::HelpAutoNames,
            Self::AutoLangs => MuxConfigArg::HelpAutoLangs,
            Self::AutoCharsets => MuxConfigArg::HelpAutoCharsets,
            //Self::SortFonts => MuxConfigArg::HelpSortFonts,
        }
    }

    fn to_arg(self) -> MuxConfigArg {
        match self {
            Self::AutoDefaults => MuxConfigArg::AutoDefaults,
            Self::AutoForceds => MuxConfigArg::AutoForceds,
            Self::AutoEnableds => MuxConfigArg::AutoEnableds,
            Self::AutoNames => MuxConfigArg::AutoNames,
            Self::AutoLangs => MuxConfigArg::AutoLangs,
            Self::AutoCharsets => MuxConfigArg::AutoCharsets,
            //Self::SortFonts => MuxConfigArg::SortFonts,
        }
    }

    fn to_no_arg(self) -> MuxConfigArg {
        match self {
            Self::AutoDefaults => MuxConfigArg::NoAutoDefaults,
            Self::AutoForceds => MuxConfigArg::NoAutoForceds,
            Self::AutoEnableds => MuxConfigArg::NoAutoEnableds,
            Self::AutoNames => MuxConfigArg::NoAutoNames,
            Self::AutoLangs => MuxConfigArg::NoAutoLangs,
            Self::AutoCharsets => MuxConfigArg::NoAutoCharsets,
            //Self::SortFonts => MuxConfigArg::NoSortFonts,
        }
    }

    fn to_help(self) -> &'static str {
        match self {
            Self::AutoDefaults => Msg::HelpAutoDefaults,
            Self::AutoForceds => Msg::HelpAutoForceds,
            Self::AutoEnableds => Msg::HelpAutoEnableds,
            Self::AutoNames => Msg::HelpAutoNames,
            Self::AutoLangs => Msg::HelpAutoLangs,
            Self::AutoCharsets => Msg::HelpAutoCharsets,
        }
        .to_str_localized()
    }
}
