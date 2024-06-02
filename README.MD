# System

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/vrtl/system/build.yml?style=flat&logo=github&label=build)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/vrtl/system/clippy.yml?style=flat&logo=github&label=lints)
![Codecov](https://img.shields.io/codecov/c/github/vrtl/system?token=5WvQyfAGjE&style=flat&logo=codecov)
[![dependency status](https://deps.rs/repo/github/vrtl/system/status.svg?path=crates%2Fapp)](https://deps.rs/repo/github/vrtl/system?path=crates%2Fapp)
![GitHub License](https://img.shields.io/github/license/vrtl/system?style=flat&logo=apache)

## Development
### Setup VSCode for Leptos

- Install leptosfmt

`cargo install leptosfmt`

   - Include in VSCode settings.json

    "rust-analyzer.procMacro.ignored": {
        "leptos_macro": [
            "server"
        ]
    },
    "rust-analyzer.rustfmt.overrideCommand": [
        "leptosfmt",
        "--stdin",
        "--rustfmt"
    ],

