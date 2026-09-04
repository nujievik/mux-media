use crate::config::ConfigOutput;
use crate::{Config, Result, TryFinalizeInit};

impl TryFinalizeInit for Config {
    fn try_finalize_init(&mut self) -> Result<()> {
        self.input.try_finalize_init()?;
        self.finalize_output()
    }
}

impl Config {
    fn finalize_output(&mut self) -> Result<()> {
        if self.is_output_constructed_from_input
            && Some(self.input.dir()) != self.output.dir().parent()
        {
            self.output = ConfigOutput::try_from(&self.input)?;
        }

        self.output.try_finalize_init()
    }
}
