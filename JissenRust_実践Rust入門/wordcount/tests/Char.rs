use std::io::Cursor;
use wordcount::{count, CountOption};

#[macro_use]
mod utils;

/// b"..." バイト文字列リテラル、文字列の代わりに[u8]を構築します

#[test]
fn char_count_works() {
    let input = Cursor::new(b"aaaaaabbcddrr");
    let freqs = count(input, CountOption::Char);

    assert_map!(
        freqs,{
            "a"=> 6,
            "b"=> 2,
            "c"=> 1,
            "d"=> 2,
            "r"=> 2
        }
    );
}

#[test]
fn char_count_utf8() {
    let input = Cursor::new(
        r#"
天地玄.
宇宙洪荒
盈昃日月
辰宿列張
"#,
    );
    let freqs = count(input, CountOption::Char);
    assert_eq!(freqs.len(), 16);
    for (_, count) in freqs {
        assert_eq!(count, 1);
    }
}
