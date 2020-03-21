/**
 * トレイトはポリモーフィズムを実現する手段の一つ
 * haskellの型クラスに近い
 * ダックタイピングがしやすい
 */

trait DuckLike {
    // トレイトを実装する型が実装するべきメソッドを定義
    fn quack(&self);

    // デフォルトメソッドぉ定義することもできる
    fn walk(&self) {
        println!("walking");
    }
}

// トレイトを実装するためだけのデータ型なら、Unit構造体が便利
struct Duck;

// `impl　トレイト名　for 型名　{..}`で定義可能
impl DuckLike for Duck {
    // トレイトで実装されていないメソッドを実装側で定義する
    fn quack(&self) {
        println!("quick");
    }
}

struct Tsuchinoko;

// 別の型にも実装できます。
impl DuckLike for Tsuchinoko {
    fn quack(&self) {
        // どうやらこのツチノコの正体はネコだったようです
        println!("mew");
    }

    // デフォルトメソッドで上書きすることもできる
    fn walk(&self) {
        println!("wringgling");
    }
}

// 既存の型にトレイトを実装することもできる
// モンキーパッチをしているような気分    <=?
impl DuckLike for i64 {
    fn quack(&self) {
        for _ in 0..*self {
            println!("quack");
        }
    }
}

fn main() {
    let duck = Duck;
    let tsuchinoko = Tsuchinoko;
    let i = 3;
    duck.quack(); // => quack
    tsuchinoko.quack(); // => mew
    i.quack(); // quack; quack; quack
    tsuchinoko.walk();
}