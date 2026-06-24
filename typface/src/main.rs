mod cache;
mod config;
mod process;

use cache::{Cache, TypstVersion, VersionPrefix};
use process::{call, resolve_env};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1).peekable();

    let cfg = config::get_conf()?;
    let cache = std::cell::LazyCell::new(|| cache::read_cache());

    if let Some(cmd) = args.peek() {
        match cmd.as_str() {
            "--version" => version(),
            "--list" => {
                // TODO: completions based on this
                // Possibly fix the input file completions thing as well (only suggest .typ)?

                // TODO: print a sorted list
                for (_, version) in cache.iter() {
                    println!("{}", version.get_string());
                }
            }
            "--discover" => cache::rediscover_binaries(cfg.discover),
            "--" => {
                let _ = args.next(); // skip over `--`

                let bin = cfg.default.canonicalize().unwrap();
                let env = cache::lookup::version(&bin, &cache)
                    .map(|v| resolve_env(&cfg.opt, &v))
                    .unwrap_or_default();

                call(bin, args, env)
            }
            s if let Some((bin, version)) = cli_scan_version(s, &cache).map(|version| {
                if let Some(bin) = cache::lookup::binary(&version, &cache) {
                    (bin, version)
                } else {
                    eprintln!(
                        "couldn't find version {} under installed binaries. Is it in $PATH and did you run --discover?",
                        version.get_string()
                    );

                    std::process::exit(1);
                }
            }) => {
                let _ = args.next(); // skop over the version
                call(bin, args, resolve_env(&cfg.opt, &version))
            },
            _ => {
                let bin = cfg.default.canonicalize().unwrap();
                let env = cache::lookup::version(&bin, &cache)
                    .map(|v| resolve_env(&cfg.opt, &v))
                    .unwrap_or_default();

                call(bin, args, env)
            }
        }
    }

    Ok(())
}

fn version() -> ! {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    std::process::exit(0);
}

fn cli_scan_version(s: &str, cache: &Cache) -> Option<TypstVersion> {
    let explicit = cache
        .iter()
        .filter_map(|(_, v)| {
            if v.get_prefix() == VersionPrefix::Raw {
                Some((v.get_string(), v))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if let Some((_, v)) = explicit.iter().find(|(v, _)| s == v) {
        return Some((*v).clone());
    }

    let mut s = unscanny::Scanner::new(s);
    s.eat_whitespace();

    let major = s.eat_while(char::is_ascii_digit).parse::<u32>();
    if !s.eat_if(".") {
        None?;
    }

    let minor = s.eat_while(char::is_ascii_digit).parse::<u32>();
    let _ = s.eat_if(".");

    let patch = s.eat_while(char::is_ascii_digit).parse::<u32>();

    if let (Ok(major), Ok(minor)) = (major, minor) {
        // if patch is specified, use it; else use the highest patch of the specified major / minor
        let patch = patch.ok().or_else(|| {
            cache
                .iter()
                .filter_map(|(_, v)| match v.get_prefix() {
                    VersionPrefix::Versioned if v.major == major && v.minor == minor => {
                        Some(v.patch)
                    }
                    _ => None,
                })
                .max()
        })?;

        return Some(TypstVersion::new(major, minor, patch));
    }

    None
}
