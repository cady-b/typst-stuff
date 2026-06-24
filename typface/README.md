A minimal interface for working with multiple versions of Typst on Linux. Made for my personal use, it's far from production-ready and only works on Linux currently cuz that's what I have.

Reads a config like this and runs Typst (optionally a specific version specified as the first argument) accordingly basically:
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

`typface --discover` discoveres installed Typst binaries, `typface --list` lists what's in the cache.
