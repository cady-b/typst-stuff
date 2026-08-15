use std::{
    collections::BTreeMap,
    env, fs,
    io::{BufWriter, Write},
    path::Path,
    process::Command,
};

use crate::{cache::TypstVersion, config};

pub fn rediscover_binaries(options: config::Discover) {
    let path_var = env::var("PATH").expect("unable to read $PATH");
    let paths = env::split_paths(&path_var);

    let overwrites = options
        .named
        .iter()
        .filter_map(|(buf, v)| buf.canonicalize().ok().map(|buf| (buf, v)))
        .collect::<BTreeMap<_, _>>();

    // Find executables that "look like Typst"
    let executables = paths.flat_map(fs::read_dir).flat_map(|entries| {
        entries
            .flat_map(std::convert::identity) // TODO: check if we can use .flatten() instaed
            .filter_map(|entry| {
                let path = entry.path();
                may_be_typst(&path).then_some(path)
            })
    });

    // Verify output actually looks like typst and extract version information
    let typsts = executables.flat_map(|v| v.canonicalize()).flat_map(|exec| {
        let version = match overwrites.get(&exec) {
            Some(&v) => Some(TypstVersion::new_raw(v.to_owned())),
            None => Command::new(&exec)
                .arg("--version")
                .output()
                .ok()
                .and_then(|v| match v.status.code() {
                    Some(0) => String::from_utf8(v.stdout)
                        .ok()
                        .and_then(|v| typst_cli_version(&v).ok()),
                    _ => None,
                }),
        };

        version.map(|v| (exec, v))
    });

    let cache_file = config::cache_file();
    let mut f = BufWriter::new(
        std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&cache_file)
            .unwrap_or_else(|e| panic!("Unable to write cache to {cache_file}: {e}\n\n")),
    );

    // Write cache
    for (path, version) in typsts {
        write!(f, "{}{}", version.prefix(), version.stringify()).unwrap();
        f.write_all(&[0]).unwrap();
        write!(f, "{}", path.to_string_lossy()).unwrap();
        f.write_all(&[0]).unwrap();
    }

    f.flush().unwrap();
}

fn may_be_typst(candidate: &Path) -> bool {
    if !candidate.is_file() {
        return false;
    }

    candidate
        .file_stem()
        .map(|v| v.to_string_lossy().starts_with("typst"))
        .is_some_and(std::convert::identity)
}

fn typst_cli_version(value: &str) -> Result<TypstVersion, Box<dyn std::error::Error>> {
    let mut s = unscanny::Scanner::new(value);

    if !s.eat_if("typst") {
        Err("Not Typst")?;
    }

    let _ = s.eat_whitespace();

    let (major, minor, patch) = TypstVersion::parse_triplet(&mut s)?;

    let _ = s.eat_whitespace();

    let mut commit = None;
    if s.eat_if("(") {
        let hash = s.eat_while(char::is_ascii_hexdigit);
        if s.eat_if(")") {
            commit = Some(hash.to_owned());
        }
    }
    if commit.is_none() {
        Err("unable to parse commit")?;
    }

    Ok(TypstVersion {
        major,
        minor,
        patch,
        raw: None,
        commit,
    })
}
