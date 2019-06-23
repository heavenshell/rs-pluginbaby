use std::env;
use std::fs::canonicalize;
use std::path::PathBuf;

use log::{error, info};
use clap::ArgMatches;

use crate::pack::{collect_repos, update_repo};

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
        Err(_) => env::current_dir().unwrap(),

    };
    let path = canonicalize(base_path).unwrap();
    Args::new(path, depth)
}

pub fn execute(matches: &ArgMatches) {
    let args = parse(&matches);
    let packs = collect_repos(args.path, args.depth);
    for pack in packs {
        match update_repo(pack.path().to_owned(), &*pack.url()) {
            Ok(_) => info!("{:?} update sucess.", pack.url()),
            Err(e) => {
                error!("{:?}", e);
                continue
            },
        };
    }
}
