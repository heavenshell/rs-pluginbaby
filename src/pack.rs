use std::fs;
use std::path::PathBuf;

use failure::Error;
use git2::Repository;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use walkdir::{DirEntry, WalkDir};

#[derive(Deserialize, Serialize, Debug)]
pub struct Pack {
    path: PathBuf,
    url: String,
    remote_name: String,
}

impl Pack {
    pub fn new(path: PathBuf, url: &str, remote_name: &str) -> Pack {
        Pack {
            path,
            url: url.to_string(),
            remote_name: remote_name.to_string(),
        }
    }
    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

pub fn fetch_repo(path: &PathBuf, url: &str) -> Result<(), Error> {
    let repo = Repository::open(&path).expect("No repository found");
    let mut remote = repo.remote_anonymous(&url)?;
    remote.fetch(&["master"], None, None)?;

    Ok(())
}

pub fn reset_repo(path: &PathBuf) -> Result<(), Error> {
    let repo = Repository::open(&path).expect("No repository found");
    let reference = "HEAD";
    let oid = repo.refname_to_id(reference)?;
    let object = repo.find_object(oid, None)?;
    repo.reset(&object, git2::ResetType::Hard, None)?;

    Ok(())
}

pub fn update_repo(path: PathBuf, url: &str) -> Result<(), Error> {
    fetch_repo(&path, url)?;
    reset_repo(&path)?;
    // TODO update submodule
    Ok(())
}

pub fn clone_repo(url: &str, path: PathBuf) -> Result<(), Error> {
    let repo = Repository::clone(url, &path)?;
    debug!("{:?}", repo.path());

    Ok(())
}

pub fn get_package(path: PathBuf, remote_name: &str) -> Option<Pack> {
    let repo = Repository::open(&path).expect("No repository found");
    let remote = repo.find_remote(remote_name).ok()?;
    let url = remote.url()?;
    let pack = Pack::new(path, url, remote_name);
    Some(pack)
}

pub fn collect_git_paths(base_path: PathBuf, max_depth: usize) -> Vec<PathBuf> {
    let mut paths = vec![];
    let walker = WalkDir::new(&base_path)
        .max_depth(max_depth)
        .follow_links(true);
    for entry in walker {
        let e: DirEntry = match entry {
            Ok(e) => e,
            Err(e) => {
                error!("{:?}", e);
                continue;
            }
        };
        let path = e.into_path();
        if !path.is_dir() {
            continue;
        }
        if fs::metadata(&path.join(".git")).is_err() {
            // Not a git directory.
            continue;
        }
        paths.push(path);
    }
    paths
}

pub fn collect_repos(base_path: PathBuf, max_depth: usize) -> Vec<Pack> {
    let mut packs: Vec<Pack> = Vec::new();
    let paths = collect_git_paths(base_path, max_depth);
    for path in paths {
        let pack = match get_package(path, "origin") {
            Some(p) => p,
            None => {
                error!("Repository not found.",);
                continue;
            }
        };
        packs.push(pack);
    }
    packs
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_repository_integrations() {
        // TODO Clone from GitHub is too slow...
        // Test clone_repo()
        let tmpdir = tempdir().unwrap();
        let ret = clone_repo(
            "https://github.com/heavenshell/vim-inhibitor",
            tmpdir.path().to_path_buf(),
        );
        assert!(ret.is_ok(), "Clone git repository");

        // Test get_package()
        let pack = get_package(tmpdir.path().to_path_buf(), "origin").unwrap();
        assert_eq!(
            pack.url(),
            "https://github.com/heavenshell/vim-inhibitor",
            "Repository url"
        );
        assert_eq!(pack.remote_name, "origin", "Repository remote name");

        // Test collect_git_paths()
        let paths = collect_git_paths(tmpdir.path().to_path_buf(), 1);
        assert_eq!(paths.len(), 1, "Collect git repository path length is 1");
        assert_eq!(
            paths[0],
            tmpdir.path(),
            "Collect git repository path is same"
        );

        // Test collect_repos()
        let packs = collect_repos(tmpdir.path().to_path_buf(), 1);
        assert_eq!(packs.len(), 1, "Collect packs length is 1");
        assert_eq!(packs[0].path(), tmpdir.path(), "Collect pack path is same");
        assert_eq!(
            packs[0].url(),
            "https://github.com/heavenshell/vim-inhibitor",
            "Collect git url path is same"
        );
    }
}
