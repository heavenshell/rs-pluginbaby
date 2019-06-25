use std::env;
use std::fs::{canonicalize, create_dir_all, metadata, File};
use std::io::BufReader;
use std::path::PathBuf;

use clap::ArgMatches;
use log::{debug, error, info};
use serde_yaml;

use crate::pack::{clone_repo, Pack};

#[derive(Debug)]
struct Args {
    path: PathBuf,
    dist: Option<PathBuf>,
}

impl Args {
    fn new(path: PathBuf, dist: Option<PathBuf>) -> Args {
        Args { path, dist }
    }
}

fn parse(matches: &ArgMatches) -> Args {
    let conf_path = match value_t!(matches, "conf", String) {
        Ok(v) => PathBuf::from(v),
        Err(_) => {
            let current = env::current_dir().unwrap();
            info!("path dose not found, use {:?}", &current);
            current
        }
    };
    let dist_path = match value_t!(matches, "dist", String) {
        Ok(v) => Some(PathBuf::from(v)),
        Err(_) => None,
    };

    let path = canonicalize(conf_path).unwrap();
    Args::new(path, dist_path)
}

pub fn execute(matches: &ArgMatches) {
    let args = parse(&matches);

    let force = args.dist.is_some();
    let dist = if force {
        let d = &args.dist.unwrap();
        if !d.exists() {
            if let Ok(v) = create_dir_all(d) {
                v
            };
        }
        canonicalize(d).unwrap()
    } else {
        env::current_dir().unwrap()
    };

    let repofile = if args.path.ends_with("Repofile") {
        args.path
    } else {
        args.path.join("Repofile")
    };

    let file = File::open(repofile).unwrap();
    let reader = BufReader::new(file);
    let repos: Vec<Pack> = serde_yaml::from_reader(reader).unwrap();
    info!("Start Restoring");
    for repo in &repos {
        let url = repo.url();
        let path = if force {
            dist.join(repo.path().to_path_buf().iter().last().unwrap())
        } else {
            repo.path().to_path_buf()
        };

        let dirname = match path.parent() {
            Some(d) => d,
            None => {
                error!("Parent path does not found");
                continue;
            }
        };

        if metadata(&path).is_ok() {
            error!("{:?} already exists at {:?}", url, path);
            continue;
        }

        // Check restore path is exists.
        if metadata(dirname).is_err() {
            match create_dir_all(dirname) {
                Ok(v) => v,
                Err(e) => {
                    error!("{:?}", e);
                    continue;
                }
            };
        }
        debug!("Restore {:?}", path);
        match clone_repo(url, path.to_path_buf()) {
            Ok(v) => v,
            Err(e) => {
                error!("{:?}", e);
                continue;
            }
        };
    }
}
