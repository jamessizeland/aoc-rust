# Advent of Code

## Getting started

This project uses the [aoc-cli](https://github.com/scarvalhojr/aoc-cli).  Follow the section on [adding your session cookie](https://github.com/scarvalhojr/aoc-cli?tab=readme-ov-file#session-cookie-) to a .session file at the root of this project.

```bash
# install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# install the advent of code CLI to automate downloading puzzles
cargo install aoc-cli
```

Set up and run the puzzles with `cargo run {day} {year}`.  It will download the input, create a template starting point, then run the test and finally run the main if that passes.  You can run your day through this command every time if you like.

```text
.
├── Cargo.toml                  # Workspace Root (Virtual Manifest)
├── Cargo.lock
├── .session                    # gitignored file containing your Advent of Code Session Cookie String
├── aoc/                        # Crate 1: Tooling & Shared Logic
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs              # The "prelude" (exports helpers so puzzles can see them)
│       ├── helpers/            # Shared logic (grid parsers, math utils)
│       │   ├── file.rs
│       │   ├── grid.rs
│       │   └── parsing.rs
│       └── bin/
│           └── cli.rs          # The entry point (generates and runs the day's tests & main)
│
└── puzzles/                    # Crate 2: The actual daily code
    ├── Cargo.toml
    ├── input/                  # Data files (fetched by the cli)
    │   └── 2025/
    │       └── day01/
    │           ├── input.aoc
    │           └── puzzle.md
    └── src/
        └── bin/                # Runnable binaries for specific days
            └── 2025_day01.rs   # cargo run -p puzzles --bin 2025_day01

```
