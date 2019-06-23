use std::io;

#[cfg(debug_assertions)]
use better_panic;
use clap::ArgMatches;
use env_logger::Env;

#[macro_use]
extern crate clap;

mod cli;
mod commands;
mod pack;

fn main() {
    #[cfg(debug_assertions)]
    better_panic::Settings::debug()
        .most_recent_first(false)
        .install();

    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let app_matches = cli::build_cli().get_matches();

    match app_matches.subcommand() {
        ("dump", Some(m)) => commands::dump::execute(m),
        ("list", Some(m)) => commands::list::execute(m),
        ("restore", Some(m)) => commands::restore::execute(m),
        ("update", Some(m)) => commands::update::execute(m),
        ("completions", Some(m)) => {
            let shell = m.value_of("SHELL").unwrap();
            cli::build_cli().gen_completions_to("pack", shell.parse().unwrap(), &mut io::stdout());
        }
        _ => commands::list::execute(&ArgMatches::default()),
    }
}
