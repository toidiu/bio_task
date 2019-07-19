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
    pub static ref CONFIG: AppConfig = {
        get_config()
    };

}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub paseto_timeout_min: i64,
    pub password: String,
    pub paseto_token: String,
}

fn get_config() -> AppConfig {
    let contents = fs::read_to_string("local.password.toml")
        .expect("Something went wrong reading the file");

    let config: AppConfig = toml::from_str(&contents).expect(
        "please add a password file at root of server
                project 'local.password.toml'",
    );
    config
}
