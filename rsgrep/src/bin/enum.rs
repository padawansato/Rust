/**
* 列挙型はどれか1つを表す型
* 関数型言語から来た機能
* `enum 名前 {列挙子, ..}`
* 列挙師へのアクセス`列挙型名::列挙子名`
*/

// コマンドを登録して実行するプログラム
// コマンドはゲームの移動をイメージしている

enum Command {
    // 列挙子は構造体のように3種類の定義ができる
    Right(i64),
    Up(i64),
    Move { x: i64, y: i64 },
    Print,
}

fn main() {
    let mut cur = (0, 0);
    // 列挙子を指定してコマンドを登録
    let commands = &[
        Command::Move { x: 0, y: 0 },
        Command::Right(5),
        Command::Up(5),
        Command::Print,
        Command::Move { x: 10, y: 10 },
        Command::Print,
    ];
    for c in commands {
        // match式で値を取り出す
        match *c {
            // match式でのパターンマッチでも、列挙型名を明記する
            Command::Right(x) => cur.0 += x,
            Command::Up(y) => cur.1 += y,
            Command::Move { x, y } => {
                cur.0 += x;
                cur.1 += y;
            }
            Command::Print => {
                println!("{:?}", cur);
            }
        }
    } // => (5,5)
} // (10,10)