/**
 * 単語の頻度を数えるツール
 * 1. コマンドラインで指定された引数を読み込む
 * 2. 指定されたファイルを読み込む
 * 3. ファイルから1行ずつ読み込む
 * 4. その行を単語に分割する
 * 5. 出現した単語の頻度を数える
*/
use std::env;
use std::fs::File;
use std::io::BufReader;
// grepと大体使っているクレートが一緒っぽい。

// lib.rsに分離したクレートを使う
use wordcount::count;

fn main() {
    // 1. コマンドラインで指定された引数を読み込む
    let filename = env::args().nth(1).expect("1 argument FILENAME required");
    // 2. 指定されたファイルを読み込む
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);

    // 3. ファイルから1行ずつ読み込む
    let freqs = count(reader, Default::default());
    println!("{:?}", freqs);
}
