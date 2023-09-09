// textos::fmt::indent
//!
//

#[cfg(feature = "alloc")]
use alloc::{format, string::String, vec::Vec};

/// *`i`ndented `format`*.
///
/// # Examples
/// ```
/// use textos::fmt::iformat;
///
/// assert_eq!["  foo\n  bar", &iformat!(2, "{}\n{}", "foo", "bar")];
/// ```
#[macro_export]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
macro_rules! iformat {
    ($indent:expr, $($args:tt)*) => {
        $crate::fmt::indent($indent, &format![$($args)*])
    };
}
#[cfg(feature = "alloc")]
pub use iformat;

/// Indents a multi-line `string` slice with the given number of `spaces`.
///
/// # Examples
/// ```
/// use textos::fmt::indent;
///
/// assert_eq!["  foo\n  bar", &indent(2, "foo\nbar")];
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub fn indent(spaces: usize, string: &str) -> String {
    let indentation = " ".repeat(spaces);

    let lines: Vec<&str> = string.lines().collect();

    let mut indented_lines: Vec<String> = Vec::new();
    for line in lines {
        indented_lines.push(format!("{}{}", indentation, line));
    }

    indented_lines.join("\n")
}
