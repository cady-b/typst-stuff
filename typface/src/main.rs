mod args;
mod cache;
mod config;
mod process;

use cache::{Cache, TypstVersion, VersionPrefix};
use clap::Parser;
use itertools::{Either, Itertools};
use process::{call, resolve_env};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Cli::parse();

    let cfg = config::get_conf()?;
    let cache = std::cell::LazyCell::new(cache::read_cache);

    if let Some(args) = args.command {
        use args::Commands;

        match args {
            Commands::Defer(remaining) => {
                let bin = cfg.default.canonicalize().unwrap();
                let env = cache::lookup::version(&bin, &cache)
                    .map(|v| resolve_env(&cfg.opt, &v))
                    .unwrap_or_default();

                call(bin, remaining.remaining, env)
            }
            Commands::Discover => cache::rediscover_binaries(cfg.discover),
            Commands::List => {
                // TODO: restrict file-completions to .typ)?

                println!("Default: {}\n", cfg.default.display());

                let (raw, versioned): (Vec<_>, Vec<_>) =
                    cache.iter().partition_map(|(b, v)| match v.prefix() {
                        VersionPrefix::Raw => Either::Left((b, v.raw.clone().unwrap_or_default())),
                        VersionPrefix::Versioned => Either::Right((b, v)),
                    });

                if !raw.is_empty() {
                    println!("Named:");
                }
                let max_len = raw.iter().map(|(_, v)| v.len()).max().unwrap_or_default();
                for (bin, version) in raw {
                    println!("  {version:max_len$} ({bin})");
                }

                if !versioned.is_empty() {
                    println!("\nVersioned:");
                }
                for (bin, version) in versioned
                    .iter()
                    .sorted_by(|(_, b), (_, a)| a.triplet().cmp(&b.triplet()))
                {
                    println!("  {} ({bin})", version.stringify());
                }
            }
            Commands::Preview(maybe_version) => {
                let mut args = maybe_version.remaining.into_iter().peekable();

                let (bin, version) = match args.peek() {
                    Some(s) if let Some((bin, version)) = match_version(s, &cache) => {
                        let _ = args.next(); // skip the version
                        (bin.into(), version)
                    }
                    _ => {
                        let bin = cfg.default.canonicalize().unwrap();
                        let version = cache::lookup::version(&bin, &cache).unwrap();

                        (bin, version)
                    }
                };

                let args = ["watch".to_owned()]
                    .into_iter()
                    .chain(args)
                    .chain(["/tmp/typface_preview.pdf".to_owned(), "--open".to_owned()]);

                call(bin, args, resolve_env(&cfg.opt, &version));
            }
        }
    } else {
        let mut args = args.remaining.remaining.into_iter().peekable();

        let (bin, env) = match args.peek() {
            Some(s) if let Some((bin, version)) = match_version(s, &cache) => {
                let _ = args.next(); // skip over the version
                (bin.into(), resolve_env(&cfg.opt, &version))
            }
            _ => {
                let bin = cfg.default.canonicalize().unwrap();
                let env = cache::lookup::version(&bin, &cache)
                    .map(|v| resolve_env(&cfg.opt, &v))
                    .unwrap_or_default();

                (bin, env)
            }
        };

        call(bin, args, env)
    }

    //let file = std::env::current_dir().map(|v| v.join("typst.toml"));

    Ok(())
}

fn match_version(s: &str, cache: &Cache) -> Option<(String, TypstVersion)> {
    cli_scan_version(s, cache).map(|version| {
                if let Some(bin) = cache::lookup::binary(&version, cache) {
                    (bin, version)
                } else {
                    eprintln!(
                        "couldn't find version {} under installed binaries. Is it in $PATH and did you run --discover?",
                        version.stringify()
                    );

                    std::process::exit(1);
                }
            })
}

fn cli_scan_version(s: &str, cache: &Cache) -> Option<TypstVersion> {
    let explicit = cache
        .iter()
        .filter_map(|(_, v)| {
            if v.prefix() == VersionPrefix::Raw {
                Some((v.stringify(), v))
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
                .filter_map(|(_, v)| match v.prefix() {
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
