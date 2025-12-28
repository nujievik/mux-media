use crate::{Config, Container, IsDefault, Msg, MuxLogger, Output, Result, TryFinalizeInit};

impl TryFinalizeInit for Config {
    fn try_finalize_init(&mut self) -> Result<()> {
        input(self)?;
        output(self)?;
        container(self);

        return Ok(());

        fn input(cfg: &mut Config) -> Result<()> {
            cfg.input.upd_out_need_num(cfg.output.need_num());
            cfg.input.try_finalize_init()
        }

        fn output(cfg: &mut Config) -> Result<()> {
            if cfg.is_output_constructed_from_input
                && Some(cfg.input.dir.as_path()) != cfg.output.dir.parent()
            {
                cfg.output = Output::try_from(&cfg.input)?;
            }

            cfg.output.try_finalize_init()
        }

        fn container(cfg: &mut Config) {
            let mut c = Container::new(&cfg.output);

            if !c.is_default() && !cfg.retiming_options.is_default() {
                eprintln!(
                    "{}{}. {} Matroska (.mkv)",
                    MuxLogger::color_prefix(log::Level::Warn),
                    Msg::UnsupRetimingExt,
                    Msg::Using,
                );
                c = Container::Matroska;
            }

            let ext = c.as_ext();
            if ext != &cfg.output.ext {
                cfg.output.ext = ext.into();
            }

            cfg.container = c;
        }
    }
}
