#![deny(warnings)]
#![recursion_limit = "128"]

#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;

pub mod error;
pub mod options;

pub use oom_api as api;
pub use oom_db as db;
pub use oom_model as model;

use error::Result;
use std::env;
use std::net::SocketAddr;

fn main() -> Result<()> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "backtrace=info");
    }

    pretty_env_logger::init();

    let matches = clap::App::new("oom")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(options::port())
        .arg(options::host())
        .arg(options::database_url())
        .arg(options::config_path())
        .get_matches();

    let addr: SocketAddr = format!(
        "{}:{}",
        matches.value_of("host").unwrap(),
        matches.value_of("port").unwrap()
    )
    .parse()
    .unwrap();

    info!("oom backend server will be listening on {}", addr);

    let database_url = matches.value_of("database_url").unwrap();
    let config_path = matches.value_of("config_path").unwrap();

    api::listen(addr, database_url, config_path);

    Ok(())
}
