use clap::{App, AppSettings, Arg, SubCommand};
use std::path::Path;

fn validate_path(path: String) -> Result<(), String> {
    let p = Path::new(&path);
    if p.exists() {
        Ok(())
    } else {
        Err(String::from("Given path not found."))
    }
}

pub fn build_cli() -> App<'static, 'static> {
    let path = Arg::with_name("path")
        .long("path")
        .short("p")
        .takes_value(true)
        .validator(validate_path)
        .help("Path to repositories");

    let depth = Arg::with_name("depth")
        .long("depth")
        .help("Maximum depth of retrive directories")
        .default_value("3");

    App::new("pluginbaby")
        .about("Package manager")
        .author(crate_authors!())
        .version(crate_version!())
        .subcommand(
            SubCommand::with_name("list")
                .about("List installed repositories")
                .arg(&path)
                .arg(&depth),
        )
        .subcommand(
            SubCommand::with_name("restore")
                .about("Clone repository from Repofile")
                .arg(
                    Arg::with_name("conf")
                        .long("conf")
                        .short("c")
                        .takes_value(true)
                        .validator(validate_path)
                        .help("Path to Repofile for restore repositories"),
                )
                .arg(
                    Arg::with_name("dist")
                        .long("dist")
                        .short("d")
                        .takes_value(true)
                        .help("Path to clone repository"),
                ),
        )
        .subcommand(
            SubCommand::with_name("update")
                .about("Update repository")
                .arg(&path)
                .arg(&depth),
        )
        .subcommand(
            SubCommand::with_name("dump")
                .about("Dump repository info to file")
                .arg(&path)
                .arg(&depth)
                .arg(
                    Arg::with_name("dist")
                        .long("dist")
                        .short("d")
                        .takes_value(true)
                        .validator(validate_path)
                        .help("Generate Repofile to dist"),
                ),
        )
        .subcommand(
            SubCommand::with_name("completions")
                .about("Generates completion scripts for your shell")
                .setting(AppSettings::Hidden)
                .arg(
                    Arg::with_name("SHELL")
                        .required(true)
                        .possible_values(&["bash", "fish", "zsh"])
                        .help("The shell to generate the script for"),
                ),
        )
}
