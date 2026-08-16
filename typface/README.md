A minimal wrapper for working with multiple versions of Typst on the same machine. Made for my personal use; it's far from production-ready and only works on Linux currently.

```help
Usage: typface [REMAINING]... [COMMAND]

Commands:
  --        Forward all following arguments directly to Typst
  preview   Preview a specified file. Runs `watch`, writing output to `/tmp/` and opening it in the default viewer
  discover  (Re-) Discover installed Typst binaries
  list      Information on previously discovered binaries
  help      Print this message or the help of the given subcommand(s)

Arguments:
  [REMAINING]...  Tries to parse the first as a version string (i.e. `0.15`, or `0.13.1`), forwarding others to Typst

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Typst will be run according a configuration like this, from `~/.config/typface/config.toml`:
```toml
default = "/usr/bin/typst"

[opt.debug]
env = { TYPST_FEATURES = "html,bundle,a11y-extras" }
[opt.release]
env = { TYPST_FEATURES = "html,bundle,a11y-extras" }
[opt."0.15"]
env = { TYPST_FEATURES = "html,bundle,a11y-extras" }
[opt."0.14"]
env = { TYPST_FEATURES = "html,a11y-extras" }

[discover.named]
"/home/cady/.local/bin/typst-debug" = "debug"
"/home/cady/.local/bin/typst-release" = "release"
```

A note on the fish (i don't use anything else soo 😅) completions: Because typface allows directly specifying a file to run, the initial suggestion will be quite littered. To surpress this, append `complete -c typface -n "__fish_typface_needs_command" -f` to the rules output by the build script.
