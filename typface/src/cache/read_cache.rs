use std::io::{BufRead, BufReader};

use crate::{cache::TypstVersion, config};

pub type Cache = Vec<(String, TypstVersion)>;

pub fn read_cache() -> Cache {
    let cache_file = config::cache_file();
    let mut f = BufReader::new(
        std::fs::OpenOptions::new()
            .read(true)
            .open(&cache_file)
            .expect(&format!(
                "Unable to read cache from {cache_file}. Have you --discover your Typst binaries?\n\n"
            )),
    );

    let mut buf = Vec::new();
    let mut typsts = Vec::new();
    loop {
        if f.read_until(0, &mut buf).unwrap() == 0 {
            break;
        };
        buf.pop();
        let version = scan_cache_version(&buf);
        buf.clear();

        if f.read_until(0, &mut buf).unwrap() == 0 {
            break;
        };
        buf.pop();
        let path = String::from_utf8(buf.to_owned()).ok();
        buf.clear();

        if let (Some(version), Some(path)) = (version, path) {
            typsts.push((path, version))
        }
    }

    typsts
}

fn scan_cache_version(buf: &[u8]) -> Option<TypstVersion> {
    let mut s = unscanny::Scanner::new(str::from_utf8(buf).ok()?);

    match s.eat() {
        Some('r') => Some(TypstVersion::raw(s.after().to_string())),
        Some('v') => {
            let (major, minor, patch) = TypstVersion::parse_triplet(&mut s).ok()?;
            Some(TypstVersion::new(major, minor, patch))
        }
        _ => None,
    }
}
