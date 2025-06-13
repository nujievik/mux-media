use mux_media::run;

fn main() -> Result<(), i32> {
    run().or_else(|e| {
        e.print();
        if e.use_stderr() { Err(e.code) } else { Ok(()) }
    })
}
