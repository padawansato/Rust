trait DuckLike {
    // トレイトを実装する型が実装するべきメソッドを定義
    fn sound(&self);

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
    fn sound(&self) {
        println!("quack");
    }
}

struct Cat;

impl DuckLike for Cat {
    fn sound(&self){
        println!("mew");
    }
}

struct Tsuchinoko;

// 別の型にも実装できます。
impl DuckLike for Tsuchinoko {
    fn sound(&self) {
        // どうやらこのツチノコの正体はネコだったようです
        println!("mew");
    }

    // デフォルトメソッドで上書きすることもできる
    fn walk(&self) {
        println!("wringgling");
    }
}

// // 既存の型にトレイトを実装することもできる
// // モンキーパッチをしているような気分    <=?
// impl DuckLike for i64 {
//     fn sound(&self) {
//         for _ in 0..*self {
//             println!("sound");
//         }
//     }
// }


fn main() {
    let duck = Duck;
    let tsuchinoko = Tsuchinoko;
    let cat = Cat;
    // let i = 3;
    println!("ある生き物Aの鳴き声");
    duck.sound(); // => sound()
    println!("ある生き物Aの歩き方");
    duck.walk();
    println!("ある生き物Bの鳴き声");
    tsuchinoko.sound(); // => mew
    println!("ある生き物Cの鳴き声");
    cat.sound();
    // i.sound(); // sound(); sound(); sound()
    println!("ある生き物Cの歩き方");
    tsuchinoko.walk();
}