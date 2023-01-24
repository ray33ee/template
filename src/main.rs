use clap::{Command, crate_version, crate_authors, crate_name, crate_description};
use log::{trace, debug, info, warn, error};
use anyhow::{Context, Result};

fn main() -> Result<()> {

    //Setup the env logger
    let env = env_logger::Env::default().filter_or("MY_LOG_LEVEL", "trace").write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    //Pretty panic messages
    color_eyre::install().unwrap();

    //Setup clap
    let m = Command::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .get_matches();

    println!("Hello, world!");

    Ok(())
}
