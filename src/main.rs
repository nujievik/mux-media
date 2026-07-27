use mux_media::run;

fn main() -> Result<(), i32> {
    run().or_else(|e| {
        if e.use_stderr() {
            e.print();
            Err(e.code())
        } else {
            Ok(())
        }
    })
}
