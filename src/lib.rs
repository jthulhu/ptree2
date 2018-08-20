#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/ptree/0.1.0")]

//! # ptree
//!
//! Pretty-print tree-like structures
//!
//! ## Basic usage
//!
//! ```
//! # use std::collections::HashMap;
//! # use std::io;
//! # use ptree::{print_tree, TreeBuilder};
//! # fn main() -> Result<(), io::Error> {
//! // Build a tree using a TreeBuilder
//! let tree = TreeBuilder::new("tree".to_string())
//!     .add_empty_child("empty branch".to_string())
//!     .build();
//!
//! // Print out the tree using default formatting
//! print_tree(&tree)?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! ## Implementing the `TreeItem` trait
//!
//! Rather than construct a new tree, one can implement the
//! `TreeItem` trait for a custom data structure.
//!
//! ```
//! # use std::collections::HashMap;
//! # use std::{io, borrow::Cow};
//! # use ptree::{print_tree, TreeItem, PrintConfig};
//! #[derive(Clone)]
//! struct MyCustomTree {}
//!
//! impl TreeItem for MyCustomTree {
//!     type Child = Self;
//!     fn write_self<W: io::Write>(&self, f: &mut W, config: &PrintConfig) -> io::Result<()> {
//!         write!(f, "{}", config.paint_leaf("My custom tree"))
//!     }
//!     fn children(&self) -> Cow<[Self::Child]> {
//!         Cow::from(vec![])
//!     }
//! }
//!
//! # fn main() -> Result<(), io::Error> {
//! // Build my custom tree structure
//! let tree = MyCustomTree {};
//!
//! // Print out the tree using default formatting
//! print_tree(&tree)?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! ## Output formatting
//!
//! ```
//! # extern crate ptree;
//! # extern crate ansi_term;
//! #
//! # use std::collections::HashMap;
//! # use std::io;
//! # use ptree::{print_tree_with, TreeBuilder, PrintConfig};
//! # use ptree::config::UTF_CHARS_BOLD;
//! # use ansi_term::{Color, Style};
//! # fn main() -> Result<(), io::Error> {
//! // Build a tree using a TreeBuilder
//! let tree = TreeBuilder::new("tree".to_string())
//!     .add_empty_child("empty branch".to_string())
//!     .build();
//!
//! // Set up the print configuration
//! let config = {
//!     let mut config = PrintConfig::for_stdout();
//!     config.branch_style = Style::new().fg(Color::Red).on(Color::Yellow).dimmed();
//!     config.leaf_style = Style::new().bold();
//!     config.chars = UTF_CHARS_BOLD;
//!     config.indent_size = 4;
//!     config
//! };
//!
//! // Print out the tree using custom formatting
//! print_tree_with(&tree, &config)?;
//!
//! # Ok(())
//! # }
//! ```

#[cfg(feature = "petgraph")]
extern crate petgraph;

#[cfg(feature = "ansi")]
extern crate ansi_term;
#[cfg(feature = "ansi")]
extern crate isatty;

#[cfg(feature = "serde")]
extern crate serde_value;

pub mod print_tree;
pub mod builder;
pub mod item;
pub mod config;

#[cfg(feature = "path")]
pub mod path;

#[cfg(feature = "petgraph")]
pub mod graph;

#[cfg(feature = "serde")]
pub mod value;

pub use print_tree::{print_tree, print_tree_with, write_tree, write_tree_with};
pub use builder::TreeBuilder;
pub use item::TreeItem;
pub use config::{IndentChars, PrintConfig};
