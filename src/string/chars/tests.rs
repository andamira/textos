// textos::string::chars::tests

use super::*;

#[test]
fn char_encodings() {
    let c1 = '\u{000061}'; // a
    let c2 = '\u{0000B1}'; // ±
    let c3 = '\u{0020AC}'; // €
    let c4 = '\u{01D160}'; // 𝅘𝅥𝅮

    // eprintln!("fn char_encodings():");
    // eprintln!("{0:?} x{0:05X} utf8:{1} utf16:{2}", Char32(c1), c1.len_utf8(), c1.len_utf16());
    // eprintln!("{0:?} x{0:05X} utf8:{1} utf16:{2}", Char32(c2), c2.len_utf8(), c2.len_utf16());
    // eprintln!("{0:?} x{0:05X} utf8:{1} utf16:{2}", Char32(c3), c3.len_utf8(), c3.len_utf16());
    // eprintln!("{0:?} x{0:05X} utf8:{1} utf16:{2}", Char32(c4), c4.len_utf8(), c4.len_utf16());

    assert![c1.byte_len() == 1 && c1.len_utf8() == 1 && c1.len_utf16() == 1];
    assert![c2.byte_len() == 1 && c2.len_utf8() == 2 && c2.len_utf16() == 1];
    assert![c3.byte_len() == 2 && c3.len_utf8() == 3 && c3.len_utf16() == 1];
    assert![c4.byte_len() == 3 && c4.len_utf8() == 4 && c4.len_utf16() == 2];

    assert![Char8::try_from(c1).is_ok()];
    assert![Char8::try_from(c2).is_ok()];
    assert![Char8::try_from(c3).is_err()];
    assert![Char8::try_from(c4).is_err()];

    assert![Char16::try_from(c1).is_ok()];
    assert![Char16::try_from(c2).is_ok()];
    assert![Char16::try_from(c3).is_ok()];
    assert![Char16::try_from(c4).is_err()];

    assert![Char24::try_from(c1).is_ok()];
    assert![Char24::try_from(c2).is_ok()];
    assert![Char24::try_from(c3).is_ok()];
    assert![Char24::try_from(c4).is_ok()];

    assert![Char32::try_from(c1).is_ok()];
    assert![Char32::try_from(c2).is_ok()];
    assert![Char32::try_from(c3).is_ok()];
    assert![Char32::try_from(c4).is_ok()];

    //

    let c8_1 = Char8::try_from(c1).unwrap();
    let c16_1 = Char16::try_from(c1).unwrap();
    let c24_1 = Char24::from(c1);
    let c32_1 = Char32::from(c1);

    assert![c8_1.to_char() == c1];
    assert![c16_1.to_char() == c1];
    assert![c24_1.to_char() == c1];
    assert![c32_1.to_char() == c1];

    //

    let c8_2 = Char8::try_from(c2).unwrap();
    let c16_2 = Char16::try_from(c2).unwrap();
    let c24_2 = Char24::from(c2);
    let c32_2 = Char32::from(c2);

    assert![c8_2.to_char() == c2];
    assert![c16_2.to_char() == c2];
    assert![c24_2.to_char() == c2];
    assert![c32_2.to_char() == c2];

    //

    let c16_3 = Char16::try_from(c3).unwrap();
    let c24_3 = Char24::from(c3);
    let c32_3 = Char32::from(c3);

    assert![c16_3.to_char() == c3];
    assert![c24_3.to_char() == c3];
    assert![c32_3.to_char() == c3];

    //

    let c24_4 = Char24::from(c4);
    let c32_4 = Char32::from(c4);

    assert![c24_4.to_char() == c4];
    assert![c32_4.to_char() == c4];

    //

    assert_eq![Char16::from(Char8::try_from(c1).unwrap()).to_char(), c1];
    assert_eq![Char24::from(Char8::try_from(c2).unwrap()).to_char(), c2];
    assert_eq![Char24::from(Char16::try_from(c3).unwrap()).to_char(), c3];
}
