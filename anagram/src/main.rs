// 標準ライブラリにハッシュマップがある。
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

// chars()イテレータから
fn sorted_string(s: &str) -> String {
    let mut s = s.chars().collect::<Vec<char>>();
    s.sort();
    s.into_iter().collect::<String>()
}

// ②
struct Anagram(HashMap<String, Vec<String>>); //Stringを目印にはStringを格納した配列を格納した連想配列

impl Anagram {
    // トレイト境界`AsRef<Path>`は、ざっくり意訳すると「パス名っぽいもの」を表す
    // `Self`は、`Anagram`へのエイリアス
    fn new<P: AsRef<Path>>(dictfile: P) -> Result<Self, io::Error> {
        let file = File::open(dictfile)?; // ★計算に成功したらその値を取り出し、失敗したらResultのErrで関数から抜ける
        let file = io::BufReader::new(file);
        // ハッシュマップを準備しておく
        let mut anagram = Anagram(HashMap::new());
        for line in file.lines() {
            let word = line?; // ★
            anagram.add_word(word);
        }
        Ok(anagram)
    }

    // テーブルを更新するので`&mut self`を使う
    // 登録した単語をテーブルが所有するので、`word`の所有権も奪う
    fn add_word(&mut self, word: String) {
        // 単語をアルファベット順にソートしたものを作ってキーにする
        let sorted = sorted_string(&word);
        // キーに対応する値があればそれを、なければ新たにデフォルト値（Vec::new()）を入れる
        // 返り値はキーに対応する値
        // ハッシュマップはデータの所有者なので、キーもデフォルト値も所有権を奪う
        self.0.entry(sorted).or_insert(Vec::new()).push(word);
    }

    // 検索はリードオンリーなので`&self`を使う
    // キーはリードオンリーなので`word`も参照で受け取る
    fn find(&self, word: &str) -> Option<&Vec<String>> {
        let word = sorted_string(word);
        // データの所有権はハッシュマップにあるので、返り値は参照型
        // 参照型なのでコピーは発生せず、高速
        self.0.get(&word)
    }
}

fn main() {
    // 実行時にコマンドライン引数として単語を受け取る
    let word = std::env::args().nth(1).expect("USAGE: word");
    // 辞書からAnagram構造体を作る
    // 多くのUnix環境では、このパスに辞書がある（ない場合は、手で辞書を準備してパスを変えてください）
    let table = Anagram::new("/usr/share/dict/words").expect("failed to make table");

    println!("{:?}", table.find(&word));
}
