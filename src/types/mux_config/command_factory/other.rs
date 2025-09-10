use super::Blocks;
use crate::{Msg, MuxConfigArg, ParseableArg};
use clap::{Arg, ArgAction};

impl Blocks {
    pub fn other(mut self) -> Self {
        self.0 = self
            .0
            .next_help_heading(Msg::HelpOtherOptions.as_str_localized())
            .arg(
                Arg::new(MuxConfigArg::ListContainers.undashed())
                    .long(MuxConfigArg::ListContainers.undashed())
                    .help(Msg::HelpListContainers.as_str_localized())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new(MuxConfigArg::ListLangs.undashed())
                    .long(MuxConfigArg::ListLangs.undashed())
                    .alias("list-languages")
                    .help(Msg::HelpListLangs.as_str_localized())
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new(MuxConfigArg::UserTools.undashed())
                    .long(MuxConfigArg::UserTools.undashed())
                    .help(Msg::HelpUserTools.as_str_localized())
                    .hide(hide_tool_arg())
                    .action(ArgAction::SetTrue),
            );

        self.tool_arg(
            MuxConfigArg::Ffmpeg,
            MuxConfigArg::FfmpegHelp,
            Msg::HelpFfmpegHelp,
        )
        .tool_arg(
            MuxConfigArg::Ffprobe,
            MuxConfigArg::FfprobeHelp,
            Msg::HelpFfprobeHelp,
        )
        .tool_arg(
            MuxConfigArg::Mkvmerge,
            MuxConfigArg::MkvmergeHelp,
            Msg::HelpMkvmergeHelp,
        )
    }

    pub fn version(mut self) -> Self {
        self.0 = self.0.arg(
            Arg::new(MuxConfigArg::Version.undashed())
                .short('V')
                .long(MuxConfigArg::Version.undashed())
                .help_heading(Msg::HelpOtherOptions.as_str_localized())
                .help(Msg::HelpVersion.as_str_localized())
                .action(ArgAction::SetTrue),
        );

        self
    }

    pub fn help(mut self) -> Self {
        self.0 = self.0.arg(
            Arg::new(MuxConfigArg::Help.undashed())
                .short('h')
                .long(MuxConfigArg::Help.undashed())
                .help_heading(Msg::HelpOtherOptions.as_str_localized())
                .help(Msg::HelpHelp.as_str_localized())
                .action(ArgAction::SetTrue),
        );

        self
    }
}

impl Blocks {
    fn tool_arg(mut self, long: MuxConfigArg, help_long: MuxConfigArg, help: Msg) -> Blocks {
        self.0 = self
            .0
            .arg(
                Arg::new(long.undashed())
                    .long(long.undashed())
                    .hide(true)
                    .trailing_var_arg(true)
                    .allow_hyphen_values(true)
                    .num_args(..),
            )
            .arg(
                Arg::new(help_long.undashed())
                    .long(help_long.undashed())
                    .help(help.as_str_localized())
                    .hide(hide_tool_arg())
                    .action(ArgAction::SetTrue),
            );

        self
    }
}

const fn hide_tool_arg() -> bool {
    if cfg!(all(
        feature = "with_embedded_bins",
        windows,
        target_arch = "x86_64"
    )) {
        false
    } else {
        true
    }
}
