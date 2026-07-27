use super::MuxError;

impl From<MuxError> for clap::Error {
    fn from(err: MuxError) -> clap::Error {
        match err {
            MuxError::Clap(e) => e,
            e => {
                if !e.use_stderr() {
                    return clap::Error::new(clap::error::ErrorKind::DisplayVersion);
                }

                let mut msg = e.to_string();
                if !msg.ends_with('\n') {
                    msg.push('\n');
                }
                clap::Error::raw(clap::error::ErrorKind::InvalidValue, msg)
            }
        }
    }
}
