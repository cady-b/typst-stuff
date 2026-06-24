use std::{
    ffi::{CString, OsStr},
    os::unix::ffi::OsStrExt,
};

use crate::{
    cache::{TypstVersion, VersionPrefix},
    config,
};

pub fn call(
    binary: impl AsRef<OsStr>,
    args: impl IntoIterator<Item = impl AsRef<str>>,
    extra_env: impl IntoIterator<Item = impl AsRef<str>>,
) -> ! {
    // https://docs.rs/nix/latest/nix/unistd/fn.execve.html
    // https://man.archlinux.org/man/core/man-pages/execve.2.en

    let argv0 = CString::new(binary.as_ref().as_bytes()).unwrap();
    let args = [argv0.clone()]
        .into_iter()
        .chain(args.into_iter().flat_map(|v| CString::new(v.as_ref())))
        .collect::<Vec<_>>();

    let mut env = std::env::vars()
        .flat_map(|(k, v)| CString::new(format!("{k}={v}")))
        .collect::<Vec<_>>();

    env.extend(extra_env.into_iter().flat_map(|v| CString::new(v.as_ref())));

    //dbg!("calling", &argv0, &args);

    let _ = nix::unistd::execve(&argv0, &args, &env);
    std::process::exit(1);
}

pub fn resolve_env(config: &config::Opt, version: &TypstVersion) -> Vec<String> {
    // we don't care about the patch version
    let check_ver = match version.get_prefix() {
        VersionPrefix::Raw => version.to_owned(),
        VersionPrefix::Versioned => TypstVersion::new(version.major, version.minor, 0),
    };

    config
        .iter()
        .map(|(v, o)| {
            let mut split = v.split(".");
            let v = match (
                split.next().and_then(|v| v.parse::<u32>().ok()),
                split.next().and_then(|v| v.parse::<u32>().ok()),
            ) {
                (Some(major), Some(minor)) => TypstVersion::new(major, minor, 0),
                _ => TypstVersion::raw(v.to_owned()),
            };

            (v, o)
        })
        .find(|(v, _)| v == &check_ver)
        .map(|(_, o)| {
            o.env
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}
