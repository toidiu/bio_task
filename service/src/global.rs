use slog::*;
use std::fs::{self, OpenOptions};

lazy_static! {
    pub static ref ROOT: slog::Logger = {
        // terminal output in development
        let formatter = slog_term::FullFormat::new(
                slog_term::PlainDecorator::new(std::io::stdout())
            ).build();

        let fuse = slog_async::Async::new(formatter.fuse()).build().fuse();
        slog::Logger::root(fuse, o!("crate" => "fin", "version" => env!("CARGO_PKG_VERSION")))


    };

}
