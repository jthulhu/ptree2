use crate::style::Style;

use std::io;
use std::borrow::Cow;

///
/// Main trait for exposing a tree structure to `ptree`
///
pub trait TreeItem: Clone {
    ///
    /// The type of this item's child items
    ///
    /// This is usually Self, but may be any type that itself implements TreeItem.
    ///
    type Child: TreeItem;

    ///
    /// Write the item's own contents (without children) to `f`
    ///
    /// The function returns an [`io::Result<()>`][io::Result], so calls to [`f.write`][write_fn] and
    /// [`write!`][write_macro] can be chained with `?`.
    ///
    /// The provided `style` may be used for formatting hints.
    /// Usually, everything printed should be run through [`style.paint`].
    /// However, this is not enforced, and custom implementations may choose to format
    /// only parts of the output, apply its own formatting in combination with the provided
    /// config, or ignore formatting altogether.
    ///
    /// [io::Result]: https://doc.rust-lang.org/std/io/type.Result.html
    /// [write_fn]: https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.write
    /// [write_macro]: https://doc.rust-lang.org/std/macro.write.html
    /// [`style.paint`]: ../style/struct.Style.html#typemethod.paint
    fn write_self<W: io::Write>(&self, f: &mut W, style: &Style) -> io::Result<()>;

    ///
    /// Retrieve a list of this item's children
    ///
    /// If the items contains no children (it is a leaf item), this method returns an empty list.
    ///
    fn children(&self) -> Cow<[Self::Child]>;
}

///
/// A simple concrete implementation of [`TreeItem`] using [`String`]s
///
/// While a tree of `StringItem`s can be constructed directly,
/// it is usually easier to use a [`TreeBuilder`].
///
/// [`TreeItem`]: ../item/trait.TreeItem.html
/// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
/// [`TreeBuilder`]: ../builder/struct.TreeBuilder.html
#[derive(Clone, Debug)]
pub struct StringItem {
    /// The item's own text, to be returned by [`write_self`]
    ///
    /// [`write_self`]: trait.TreeItem.html#tymethod.write_self
    pub text: String,
    /// The list of item's children
    pub children: Vec<StringItem>,
}

impl TreeItem for StringItem {
    type Child = Self;

    fn write_self<W: io::Write>(&self, f: &mut W, style: &Style) -> io::Result<()> {
        write!(f, "{}", style.paint(&self.text))
    }

    fn children(&self) -> Cow<[Self::Child]> {
        Cow::from(&self.children[..])
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::str::from_utf8;
    use super::*;

    use crate::output::write_tree_with;
    use crate::print_config::PrintConfig;

    #[test]
    fn small_item_output() {
        let deps = StringItem {
            text: "petgraph".to_string(),
            children: vec![
                StringItem {
                    text: "quickcheck".to_string(),
                    children: vec![
                        StringItem {
                            text: "libc".to_string(),
                            children: vec![],
                        },
                        StringItem {
                            text: "rand".to_string(),
                            children: vec![
                                StringItem {
                                    text: "libc".to_string(),
                                    children: vec![],
                                },
                            ],
                        },
                    ],
                },
                StringItem {
                    text: "fixedbitset".to_string(),
                    children: vec![],
                },
            ],
        };

        let config = PrintConfig {
            indent: 4,
            leaf: Style::default(),
            branch: Style::default(),
            ..PrintConfig::default()
        };

        let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

        write_tree_with(&deps, &mut cursor, &config).unwrap();

        let data = cursor.into_inner();
        let expected = "\
                        petgraph\n\
                        ├── quickcheck\n\
                        │   ├── libc\n\
                        │   └── rand\n\
                        │       └── libc\n\
                        └── fixedbitset\n\
                        ";
        assert_eq!(from_utf8(&data).unwrap(), expected);
    }
}
