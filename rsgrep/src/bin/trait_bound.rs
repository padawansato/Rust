/**
 * トレイト境界
 * 「あるトレイトを実装する型」をジェネリクスで受け取ることもできる。これをトレイト境界という。
 * 型の集合を定義するもの
 * 関数をまとめた関数みたいなもの
 **/

trait DuckLike {
    // トレイトを実装する型が実装するべきメソッドを定義
    fn quack(&self);

    // デフォルトメソッドを定義することもできる
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

// struct Tsuchinoko;

// // 別の型にも実装できます。
// impl DuckLike for Tsuchinoko {
//     fn quack(&self) {
//         // どうやらこのツチノコの正体はネコだったようです
//         println!("mew");
//     }

//     // デフォルトメソッドで上書きすることもできる
//     fn walk(&self) {
//         println!("wringgling");
//     }
// }

// 既存の型にトレイトを実装することもできる
// モンキーパッチをしているような気分    <=?
impl DuckLike for i64 {
    fn quack(&self) {
        for _ in 0..*self {
            println!("quack");
        }
    }
}

/**
 * ここまでtrait.rsと一緒
*/


// ジェネリクスの型パラメータに`型パラメータ名：　トレイト名`で境界をつけることができる
fn duck_go<D: DuckLike>(duck: D) {
    // 境界をつけることで関数本体でトレイトのメソッドが使える
    duck.quack();
    duck.walk();
}

fn main() {
    let duck = Duck;
    duck_go(duck);
//     let f = 0.0;
//     duck_go(f); // Ducklikeとれいとはfloatを実装していないため、コンパイルエラー
}