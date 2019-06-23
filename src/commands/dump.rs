use std::env;
use std::fs::{canonicalize, File};
use std::io::Write;
use std::path::PathBuf;

use clap::ArgMatches;
use log::info;
use serde_yaml;

use crate::pack::collect_repos;

struct Args {
    path: PathBuf,
    depth: usize,
    dist: PathBuf,
}

impl Args {
    fn new(path: PathBuf, depth: usize, dist: PathBuf) -> Args {
        Args { path, depth, dist }
    }
}

fn parse(matches: &ArgMatches) -> Args {
    let depth = match value_t!(matches, "depth", usize) {
        Ok(v) => v,
        Err(_) => 3,
    };
    let base_path = match value_t!(matches, "path", String) {
        Ok(v) => PathBuf::from(v),
        Err(_) => {
            let current = env::current_dir().unwrap();
            info!("path dose not found, use {:?}", &current);
            current
        }
    };
    let dist_path = match value_t!(matches, "dist", String) {
        Ok(v) => PathBuf::from(v),
        Err(_) => {
            let current = env::current_dir().unwrap();
            info!(
                "dist dose not found, write to {:?}",
                &current.join("Repofile")
            );
            current
        }
    };
    let path = canonicalize(base_path).unwrap();
    let dist = canonicalize(dist_path).unwrap();
    Args::new(path, depth, dist.join("Repofile"))
}

pub fn execute(matches: &ArgMatches) {
    let args = parse(&matches);
    let packs = collect_repos(args.path, args.depth);

    let mut file = File::create(args.dist).unwrap();
    let data = serde_yaml::to_string(&packs).unwrap();

    writeln!(file, "{}", data).unwrap();
    file.flush().unwrap();
}
