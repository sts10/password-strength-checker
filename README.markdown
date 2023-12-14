# Password Strength Checker

A Rust command-line tool that uses [zxcvbn](https://github.com/shssoichiro/zxcvbn-rs) to check the "strength" of an inputted password. Inputted password is not displayed thanks to [Rustastic Password](https://github.com/conradkdotcom/rpassword). 

## Installation

Check this project's GitHub Releases page for binaries to download.

## For developers

### How to create a release

This project uses [cargo-dist](https://opensource.axo.dev/cargo-dist/) to create releases.

You're welcome to consult [my personal notes on using cargo-dist](https://sts10.github.io/docs/cargo-dist-tips.html); but basically: First, install cargo-dist with`cargo install cargo-dist`.

When you're ready to cut a new release, test the current state of the project with `cargo dist build` and `cargo dist plan`. If that went well, create a new git tag that matches the current project version in `Cargo.toml` with `git tag vX.X.X`. Finally, run `git push --tags` to kick off the release process. GitHub will handle it from here -- check your project's GitHub Releases page in about 5 to 10 minutes.

## Other links

Here's [an online demo of zxcvbn, with some other links](https://lowe.github.io/tryzxcvbn/).
