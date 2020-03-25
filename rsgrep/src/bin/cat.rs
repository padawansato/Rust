/**
 * 自作catコマンド
 * rsgrepを参考に作成する。
 * rsgrepはfileを開き、バッファに入れ、それをパターンマッチした結果を返していた。
 * そのため、パターンマッチをせず、そのまま標準出力する。
*/
use std::env;
use std::fs::File;
use std::io::BufReader;

// 使い方
fn usage() {
    println!("cat FILENAME");
}

fn main() {
    // envモジュールのargs関数で、引数を取得
    let filename = match env::args().nth(1) {
        Some(filename) => filename, // Opthionであるためokにあらず
        None => {
            // Errにあらず
            usage();
            return;
        }
    };

    // File構造体のopen関連関数でファイルを開ける
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => {
            println!(
                "ファイルを開く際にエラーが起きました。{}",
                e
            );
            return;
        }
    };

    // Fileを開いたまま使うと遅く、扱いづらいため、バッファにいれる
    let input = BufReader::new(file);
    // inputを標準出力する
    println!("{:?}", input);
}
