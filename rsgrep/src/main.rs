use std::env;
/**
 * todo1 :引数で受け取ったファイルの中身を1行ずつプリントできる<=cat?
*/
// stdクレートのfsモジュールにあるFile型をインポート。以後は`File`として参照できる
use std::fs::File;
use std::io::{BufRead, BufReader};

// std以外のクレートを使う際
extern crate regex;
// regexからRegex型をインポート
use regex::Regex;

fn usage() {
    println!("rsgrep PATTERN FILENAME");
}

fn main() {
    // 引数からパターンを取り出す
    let pattern = match env::args().nth(1) {
        //これで引数一番目を取る
        Some(pattern) => pattern,
        None => {
            usage();
            return;
        }
    };
    // 取り出したパターンからRegexを改めて作る
    // 無効な正規表現だった場合などにはエラーが返る
    let reg = match Regex::new(&pattern) {
        Ok(reg) => reg,
        Err(e) => {
            println!("Invalid regexp {}: {}", pattern, e);
            return;
        }
    };
    // envモジュールのargs関数で、引数を取得
    // そのうち2番目を`nth`で取得(0番目はプログラムの名前)
    // 引数があるかわからないので、Opthionで返される。<= 使い時
    let filename = match env::args().nth(2) {
        // あれば取り出す
        Some(filename) => filename,
        None => {
            usage();
            return; //<= いらないんじゃなかったの？
        }
    };
    // File構造体のopen関連関数でファイルを開ける
    // 失敗する可能性があるため、Resultで返ってくる
    // 下の方でもう一度filenameを使うためにここでは&filenameと参照渡しの形
    let file = match File::open(&filename) {
        Ok(file) => file,
        Err(e) => {
            println!("An error occurred while opening file {}:{}", filename, e);
            return;
        }
    };
    // Fileをそのまま使うと遅い、加えてlineメソッドを使うためにBufReaderに含む
    // このnewもただの関連関数
    let input = BufReader::new(file);
    // BufReaderが実装するトレイトのBufReadにあるlinesメソッドを呼び出す
    // 返り値はイテレータなのでfor式で繰り返しができる
    for line in input.lines() {
        // 入力がUTF-8ではないなどの理由で、行のパースに失敗することがあるので、
        // lineもResultに含まれている
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                println!("An error occurred while reading a line {}", e);
                return;
            }
        };
        if reg.is_match(&line) {
            // パターンにマッチしたらプリントする
            // is_matchはリードオンリーなので参照型を受け取る
            println!("{}", line);
        }
    }
}
