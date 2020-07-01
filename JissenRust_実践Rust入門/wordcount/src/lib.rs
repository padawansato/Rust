//! wordcountは単語を数える機能を提供します。オプションで文字、行もカウントできます。
//! 詳しくは[`count`]のドキュメント(fm.count.html)を参照してください。
#![warn(missing_docs)]

use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

/// inputから1行ずつUTF8文字列を読み込み、頻度を数える
///
/// 頻度を数える対象はオプションにより制御される
/// * [`CountOption::Char`](enum.CountOption.html#variant.Char): Unicodeの1文字ごと
/// * [`CountOption::Word`](enum.CountOption.html#variant.Word): 正規表現`\w+`にマッチする単語ごと
/// * [`CountOption::Line`](enum.CountOption.html#variant.Line): `\n`または`\r\n`で区切られた行ごと
///
/// # Panics
///
/// 入力がUTF8でない場合パニックする。

pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new(); // HashMap<キーString,バリューusize>型

    for line in input.lines() {
        let line = line.unwrap();
        use crate::CountOption::*;
        match option {
            Char => {
                for c in line.chars() {
                    *freqs.entry(c.to_string()).or_insert(0) += 1;
                }
            }
            Word => {
                // 4. その行を単語に分割する
                for m in re.find_iter(&line) {
                    let word = m.as_str().to_string();
                    // 5. 出現した単語の頻度を数える
                    *freqs.entry(word).or_insert(0) += 1;
                }
            }
            Line => *freqs.entry(line.to_string()).or_insert(0) += 1,
        }
    }
    freqs
}

// 行や文字もカウントできるようにoptionを追加する
/// [`count`](fn.count.html)で使うオプション
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountOption {
    /// 文字ごとの頻度を数える
    Char,
    /// 単語ごとの頻度を数える
    Word,
    /// 行ごとの頻度を数える
    Line,
}
/// オプションのデフォルトは[`Word`](enum.CountOption.html#valiant.Word)
impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}

#[test]
fn  word_count_works() {
    /// Cursorは内部にバイト列を保持してインメモリバッファを作るデータ型
    use std::io::Cursor;
    let mut exp = HashMap::new();
    exp.insert("aa".to_string(), 1);
    exp.insert("bb".to_string(), 2);
    exp.insert("cc".to_string(), 1);
    assert_eq!(count(Cursor::new("aa bb cc bb"), CountOption::Word), exp);
}

#[test]
#[should_panic]
fn word_count_do_not_contain_unknown_words() {
    use std::io::Cursor;

    count(
        Cursor::new([
            b'a', // a
            0xf0, 0x90, 0x80, // でたらめなバイト列
            0xe3, 0x81, 0x82, // あ
        ]),
        CountOption::Word,
    );
}

#[cfg(tet)]
mod test {
    use super::*;
    use sdt::io::Cursor;
    // Mapの複数のkey,valueに対してassertするマクロ
    macro_rules! assert_map {
        ($expr: expr ,{$($key:expr=>$value:expr),*}) => {
            $(assert_eq!($expr[$key],$value));*
        };
    }
    fn word_count_works_with_macro() {
        let freqs = count(Cursor::new("aa bb dd"), CountOption::Word);

        assert_eq!(freqs.len(), 3);
        assert_map!(freqs,{"aa" => 1,"bb" => 1,"dd"=> 1});
    }
}
