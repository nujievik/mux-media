use super::Blocks;
use crate::{CliArg, Msg, undashed};
use clap::{Arg, ArgAction};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

impl Blocks {
    pub fn auto(mut self) -> Self {
        let mut cmd = self
            .0
            .next_help_heading(Msg::HelpAutoFlags.as_str_localized())
            .arg(
                Arg::new(undashed!(NoAuto))
                    .long(undashed!(NoAuto))
                    .help(Msg::HelpNoAuto.as_str_localized())
                    .action(ArgAction::SetTrue),
            );

        for flag in AutoFlag::iter() {
            let help_arg = flag.as_help_arg();
            let arg = flag.as_arg();
            let no_arg = flag.as_no_arg();

            cmd = cmd
                .arg(
                    Arg::new(help_arg.undashed())
                        .long(help_arg.undashed())
                        .help(flag.as_help())
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new(arg.undashed())
                        .long(arg.undashed())
                        .action(ArgAction::SetTrue)
                        .hide(true),
                )
                .arg(
                    Arg::new(no_arg.undashed())
                        .long(no_arg.undashed())
                        .action(ArgAction::SetTrue)
                        .hide(true)
                        .conflicts_with(arg.undashed()),
                );
        }

        self.0 = cmd;
        self
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
enum AutoFlag {
    AutoDefaults,
    AutoForceds,
    AutoTitles,
    AutoLangs,
    AutoEncs,
}

impl AutoFlag {
    const fn as_help_arg(self) -> CliArg {
        match self {
            Self::AutoDefaults => CliArg::HelpAutoDefaults,
            Self::AutoForceds => CliArg::HelpAutoForceds,
            Self::AutoTitles => CliArg::HelpAutoTitles,
            Self::AutoLangs => CliArg::HelpAutoLangs,
            Self::AutoEncs => CliArg::HelpAutoEncs,
        }
    }

    const fn as_arg(self) -> CliArg {
        match self {
            Self::AutoDefaults => CliArg::AutoDefaults,
            Self::AutoForceds => CliArg::AutoForceds,
            Self::AutoTitles => CliArg::AutoTitles,
            Self::AutoLangs => CliArg::AutoLangs,
            Self::AutoEncs => CliArg::AutoEncs,
        }
    }

    const fn as_no_arg(self) -> CliArg {
        match self {
            Self::AutoDefaults => CliArg::NoAutoDefaults,
            Self::AutoForceds => CliArg::NoAutoForceds,
            Self::AutoTitles => CliArg::NoAutoTitles,
            Self::AutoLangs => CliArg::NoAutoLangs,
            Self::AutoEncs => CliArg::NoAutoEncs,
        }
    }

    fn as_help(self) -> &'static str {
        match self {
            Self::AutoDefaults => Msg::HelpAutoDefaults,
            Self::AutoForceds => Msg::HelpAutoForceds,
            Self::AutoTitles => Msg::HelpAutoTitles,
            Self::AutoLangs => Msg::HelpAutoLangs,
            Self::AutoEncs => Msg::HelpAutoEncs,
        }
        .as_str_localized()
    }
}
