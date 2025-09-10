use crate::{MuxConfig, Muxer, Output, Result, Tool, TryFinalizeInit};

impl TryFinalizeInit for MuxConfig {
    fn try_finalize_init(&mut self) -> Result<()> {
        input(self)?;
        output(self)?;

        let muxer = Muxer::from(&self.output);
        tool_paths(self, muxer)?;
        self.muxer = muxer;

        return Ok(());

        fn input(cfg: &mut MuxConfig) -> Result<()> {
            cfg.input.upd_out_need_num(cfg.output.need_num());
            cfg.input.try_finalize_init()
        }

        fn output(cfg: &mut MuxConfig) -> Result<()> {
            if cfg.is_output_constructed_from_input
                && Some(cfg.input.dir.as_path()) != cfg.output.dir.parent()
            {
                cfg.output = Output::try_from(&cfg.input)?;
            }

            cfg.output.try_finalize_init()
        }

        fn tool_paths(cfg: &mut MuxConfig, muxer: Muxer) -> Result<()> {
            let mut tools: Vec<Tool> = Vec::with_capacity(Tool::LENGTH);
            tools.push(Tool::Mkvmerge);
            if !matches!(muxer, Muxer::Matroska) {
                tools.push(Tool::Ffmpeg);
            }
            cfg.tool_paths.try_resolve_many(tools, &cfg.output.temp_dir)
        }
    }
}
