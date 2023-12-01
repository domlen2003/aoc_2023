# Advent of Code 2023

## Setup
Cargo generation tools and just are used to make the workflow easier.

```shell
cargo install just
cargo install cargo-generate
```
## Prepare for a new day

```shell
just create <day>
```

## Working on a day
The `just work` command is passed a particular day and part and is the equivalent workflow of running all of these in a row and stopping if one fails, then re-starting the flow after changes.

```
cargo check
cargo test 
clippy-tracing --action check
cargo clippy
cargo bench
```