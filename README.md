# Rust Github Template

A template for [cargo generate](https://github.com/cargo-generate/cargo-generate) that aims to be a starting point suitable for
the vast majority of rust projects that will be hosted on GitHub.

See the project [website](https://rust-github.github.io).

# Differences

We have added a few extra features to this template:

 - Barebones `clap` setup
 - Logging via `log` and `env_logger`
 - `anyhow` error handling
 - `color_eyre` for pretty panic messages
 - Todo and Unfinished sections added to the changelog