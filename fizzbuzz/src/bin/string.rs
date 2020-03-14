/*
リードオンリーなら&str
書き換えたいなら&mut String
そのまま値をずっと持っておきたいならString
を使うことが多いようです。
*/

fn main(){
    // `&str`は`to_string()`メソッドで`String`にできる。
    let mut a: String = "abc".to_string();
    // 少しややこしいが、`String`に`&str`を足すと`String`ができる。
    // `&str`に`String`を足したり`String`に`String`を足したりはできない。
    println!("a:{}", a);
    a += "def";
    println!("a:{}", a);

    let x = 1.0.to_string();
    println!("x:{}", x);

    a += x.as_str();
    println!("a:{}", a);
}