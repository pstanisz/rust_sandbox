# Hello, Cargo!

A project created with Cargo, which is Rust's build system and package manager.

## Create a project

To create a new project with Cargo:

```shell
cargo new <project_name>
```

## TOML

Te Cargo configuration file format (*Tom's Obvious, Minimal Language*).

Section `[package]` indicates that the statemets are configuring a package.

Section `[dependencies]` contains a list of all of the project dependencies.

## Structure

Cargo expects that sources are within `src` subdirectory, while top level directory only contains readme, licence info, configuration files or anything else, not related to the code.

## Compilation

To compile a Cargo project:

```shell
cargo build
```

By default, *debug* mode is used and executable lands in `target/debug/<project_name>`

To compile and run a Cargo project:

```shell
cargo run
```

### Cargo.lock

The file keeps track of exact versions of dependencies.

### cargo check

The `cargo check` command checks whether the project compiles without producing an executable (it is faster than usual build).

### Release mode

To build the project in release mode:

```shell
cargo build --release
```

Executable lands in `target/release/<project_name>`.

### Cleanup

To cleanup the project:

```shell
cargo clean
```
