/**
 * 単語の頻度を数えるツール
 * 1. コマンドラインで指定された引数を読み込む
 * 2. 指定されたファイルを読み込む
 * 3. ファイルから1行ずつ読み込む
 * 4. その行を単語に分割する
 * 5. 出現した単語の頻度を数える
*/
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
// grepと大体使っているクレートが一緒っぽい。

pub fn count(input: impl BufRead) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new(); // HashMap<キーString,バリューusize>型

    for line in input.lines() {
        let line = line.unwrap();
        // 4. その行を単語に分割する
        for m in re.find_iter(&line) {
            let word = m.as_str().to_string();
            // 5. 出現した単語の頻度を数える
            *freqs.entry(word).or_insert(0) += 1;
        }
    }
    freqs
}

fn main() {
    // 1. コマンドラインで指定された引数を読み込む
    let filename = env::args().nth(1).expect("1 argument FILENAME required");
    // 2. 指定されたファイルを読み込む
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // 3. ファイルから1行ずつ読み込む
    let freqs = count(reader);
    println!("{:?}", freqs);
}
