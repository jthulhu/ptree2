# Pretty-print tree-like structures

**Warning.** (28 july 2024) This is a fork of the now seemingly unmaintained crate
[`ptree`](https://gitlab.com/Noughmad/ptree/).  This fork has been created out of the need of
bumping the version of the dependencies of the `ptree` crate.  This means that there are a few
points to keep in mind:
- The owner of this repository ("me") is *not* the author of the crate.
- This fork currently offers exactly the same features and API as its original counterpart.
- I do *not* plan to add any new features to this repository, or to maintain it any further
  myself.  Just keep it afloat and accept whatever sane PR anyone offers.
- If you would like to maintain this crate, add features, or anything similar, I am willing to
  give you ownership and/or write access to the repository and name on crates.io (if that is
  possible).

**Warning.** (1 august 2024) The author of the original crate has resumed maintaining it, and has
updated the deprecated dependencies, so this fork doesn't have a reason to live anymore.  You should
directly use the crate `ptree` in place of this one.

---

The `ptree` crate supports output formatting due to a user-provided configuration file and/or
environment variables.  ![Different output styles](https://i.imgur.com/KqPUFHq.png)

## Usage

```toml
[dependencies]
ptree = { version = "1", package = "ptree2" }
```

## Constructing a tree

There are two main ways of using `ptree` to print a tree-like data structure.
The first is to implement `TreeItem` for your structure.
The second is to create a new tree, either using `TreeBuilder` or by manually constructing `StringItem`s.

The `ptree` crate includes implementations of `TreeItem` for some common types, including a custom `StringItem` and `petgraph::Graph`.

## Printing the tree

A tree can be printed to standard output using `print_tree`, or to an arbitrary writer using `write_tree`.
Both functions have variants which take a `PrintConfig` that controls the output.
Text is formatted using `ansi-term`, which allows changing colors and styles of the printed text.

## User configuration

By default, `ptree` loads configuration from a user configuration file.
This allows end users to globally configure the output format for all applications using `ptree`.
Applications can use this configuration directly, modify it, or ignore it altogether.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
