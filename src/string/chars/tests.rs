// textos::string::chars::tests

use super::*;

#[test]
fn char_conversions() {
    let a32 = 'a';
    let a8 = Char8::try_from(a32).unwrap();
    let a16 = Char16::try_from(a32).unwrap();
    let a24 = Char24::try_from(a32).unwrap();

    assert_eq![a16, a8.into()];
    assert_eq![a24, a8.into()];
}
