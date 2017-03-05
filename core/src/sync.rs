//! Sync module to sync file between server and client
use APP_INFO;
use app_dirs::{AppDataType, app_dir};
use error::*;
use std::collections::hash_map::DefaultHasher;
use std::fs::{File, read_dir};
use std::io::{Read, Write};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// A sync cache
pub type SyncCache = (String, u64);

/// Sync cacher to cache all files
pub struct SyncCacher {
    cacher_path: PathBuf,
}

impl SyncCacher {
    /// Default constructor
    pub fn new<S: AsRef<str>>(cacher_name: S) -> Result<SyncCacher> {
        let cacher_name = cacher_name.as_ref();
        let cacher_path = app_dir(AppDataType::UserCache, &APP_INFO, cacher_name)?;
        Ok(SyncCacher { cacher_path: cacher_path.to_path_buf() })
    }

    /// Cache path
    pub fn build_cache<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let hash = hash_path(&path);
        let cache_path = self.cacher_path.join(hash);
        let mut cache_file = File::create(&cache_path)?;
        build_cache(&mut cache_file, &path)
    }

    /// Query cache for path
    pub fn query_cache<P: AsRef<Path>>(&self, path: P) -> Result<Vec<SyncCache>> {
        let hash = hash_path(&path);
        let cache_path = self.cacher_path.join(hash);
        let mut cache_file = File::open(cache_path)?;
        read_cache(&mut cache_file)
    }
}

/// Hash a path
fn hash_path<P: AsRef<Path>>(path: P) -> String {
    let path_ref = path.as_ref();
    let path_str = ::common::canonicalize(&path_ref);
    let mut hasher = DefaultHasher::new();
    path_str.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// Cache path
fn build_cache<P: AsRef<Path>, W: Write>(cache_stream: &mut W, path: P) -> Result<()> {
    let path = path.as_ref();
    if path.is_dir() {
        for mut s in cache_dir(&path)? {
            s.push('\n');
            cache_stream.write(s.as_bytes())?;
        }
    } else {
        cache_stream.write(cache_file(&path)?.as_bytes())?;
    }
    Ok(())
}

/// Read cache
fn read_cache<R: Read>(cache_stream: &mut R) -> Result<Vec<SyncCache>> {
    let mut buf = String::new();
    cache_stream.read_to_string(&mut buf)?;
    let mut vec = Vec::new();
    for line in buf.split('\n') {
        if line.is_empty() {
            continue;
        }
        let splitted = line.split_whitespace().map(|s| s.to_owned()).collect::<Vec<String>>();
        if splitted.len() == 2 {
            let path = splitted[0].clone();
            let time = splitted[1].parse::<u64>()?;
            vec.push((path, time));
        }
    }
    Ok(vec)
}

/// Cache dir
fn cache_dir<P: AsRef<Path>>(dir_path: P) -> Result<Vec<String>> {
    let path = dir_path.as_ref();
    let mut vec = Vec::new();
    vec.push(cache_file(&path)?);
    for entry in read_dir(&path)? {
        let entry = entry?;
        if entry.path().is_file() {
            vec.push(cache_file(entry.path())?);
        } else {
            vec.append(&mut cache_dir(entry.path())?);
        }
    }
    Ok(vec)
}

/// Cache file
fn cache_file<P: AsRef<Path>>(file_path: P) -> Result<String> {
    let path = file_path.as_ref();
    let name = path.to_str().unwrap().to_owned();
    let name = ::common::canonicalize(name);
    let metadata = path.metadata()?;
    let modified = metadata.modified()?;
    let now = SystemTime::now();
    let elapsed = now.duration_since(modified)?;
    Ok(format!("{} {}", name, elapsed.as_secs()))
}
