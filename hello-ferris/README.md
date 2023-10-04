new project `cargo new hello-ferris`


```
$ cargo add ferris_says
    Updating crates.io index
warning: translating `ferris_says` to `ferris-says`
      Adding ferris-says v0.3.1 to dependencies.
             Features:
             - clippy
    Updating crates.io index

```




Search `cargo search ferris-says`

Adding dependencies `cargo add ferris-says` now we can run `cargo build` and Cargo will install our dependency for us.

[crates/ferris-says](https://crates.io/crates/ferris-says)


We can **run this program** by moving into the new directory that we made and running this in our terminal: `cargo run`

{

Project:
- build your project or compile the current package with `cargo build`
- run your project with `cargo run`
- test your project with `cargo test`
- build documentation for your project with `cargo doc`
- publish a library to crates.io with `cargo publish`
- To test that you have Rust and Cargo installed, you can run this in your terminal of choice: `cargo --version`
- help `cargo help`
- remove the target directory `cargo clean`
- analyze the current package and report errors, but don't build object files `cargo check`
- create a new cargo package `cargo new`
- create a new cargo package in an existing directory `cargo init`
- add dependencies to a manifest file `cargo add`
- remove dependencies from a manifest file
`cargo remove`
- run a binary or example of the local package `cargo run`
- run the tests `cargo test` 
- run the benchmarks `cargo bench`
- update dependencies listed in Cargo.lock `cargo update`
- search registry for crates `cargo search`
- package and upload this package to the registry `cargo publish`
- install a Rust binary. Default location is $HOME/.cargo/bin `cargo install`
- uninstall a Rust binary `cargo uninstall`


}

Get the latest version of Rust by running `rustup update` or `rustup update stable`.