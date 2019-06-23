use std::env;
use std::fs::canonicalize;
use std::io::{BufWriter, stdout, Write};
use std::path::PathBuf;

use clap::ArgMatches;
use log::{error, info};

use crate::pack::collect_repos;

#[derive(Debug)]
struct Args {
    path: PathBuf,
    depth: usize,
}

impl Args {
    fn new(path: PathBuf, depth: usize) -> Args {
        Args {
            path,
            depth,
        }
    }
}

fn parse(matches: &ArgMatches) -> Args {
    let depth = match value_t!(matches, "depth", usize) {
       Ok(v) => v,
       Err(_) => 3
    };
    let base_path = match value_t!(matches, "path", String) {
        Ok(v) => PathBuf::from(v),
        Err(_) => {
            let current = env::current_dir().unwrap();
            info!("path does not found. retrieve {:?}", &current);
            current
        },

    };
    let path = canonicalize(base_path).unwrap();
    Args::new(path, depth)
}

pub fn execute(matches: &ArgMatches) {
    let out = stdout();
    let mut br = BufWriter::new(out.lock());
    let args = parse(&matches);
    let packs = collect_repos(args.path, args.depth);
    for pack in &packs {
        let url = pack.url();
        let path = match pack.path().to_str() {
            Some(p) => p,
            None => continue,
        };
        match writeln!(br, "{} \n   => {}", &url, path) {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                continue
            },
        };
    }
    info!("{} repositories found.", &packs.len());
}
