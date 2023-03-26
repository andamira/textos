// textos::ascii
//
//! ASCII related functionality.
//

/// Builds an if-else tree that returns a result depending on the uncased
/// comparison with the given ASCII.
///
/// It expects a comparison value
///
/// It uses the `eq_ignore_ascii_case` function, so the comparsion value could
/// be a [`&str`], [`char`], [`&[u8]`][slice] or [`u8`].
///
/// # Example
/// ```
/// use textos::ascii_eq_uncased;
///
/// fn ascii_eq(s: &str) -> Result<usize, &'static str> {
///     ascii_eq_uncased![s, "No matches.", "Zero", 0, "One", 1, "Two", 2]
/// }
///
/// assert_eq![Ok(0), ascii_eq("zErO")];
/// assert_eq![Ok(1), ascii_eq("onE")];
/// assert![ascii_eq("Three").is_err()];
/// ```
///
/// [`&str`]: <https://doc.rust-lang.org/std/primitive.str.html#method.eq_ignore_ascii_case>
/// [`char`]: <https://doc.rust-lang.org/std/primitive.char.html#method.eq_ignore_ascii_case>
/// [slice]: <https://doc.rust-lang.org/std/primitive.slice.html#method.eq_ignore_ascii_case>
/// [`u8`]: <https://doc.rust-lang.org/std/primitive.u8.html#method.eq_ignore_ascii_case>
///
#[macro_export]
macro_rules! ascii_eq_uncased {
    (
        // The ASCII value to match against.
        $value:ident,

        // The error returned when there are no matches.
        $error:expr,

        // The first string and result matching pair.
        $str:literal, $res:expr,

        // Optional additional string and result matching pairs.
        $($ostr:literal, $ores:expr),*

    ) => {
        if $value.eq_ignore_ascii_case(&$str) {
            Ok($res)
        }
        $(
            else if $value.eq_ignore_ascii_case(&$ostr) { Ok($ores) }
        )+
        else {
            Err($error)
        }
    }
}
