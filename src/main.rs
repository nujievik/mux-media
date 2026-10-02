use mux_media::{MuxLogger, run};

fn main() -> Result<(), i32> {
    run().or_else(|e| {
        if e.use_stderr() {
            e.print();
            MuxLogger::print_try_help();
            Err(e.code())
        } else {
            Ok(())
        }
    })
}
