fn square_sum(n: isize) -> isize {
    (0..n)
        // 高階関数filterとクロージャリテラル
        .filter(|i| i % 2 == 0)
        // 各要素に対して
        .map(|i| i * i)
        // イテレータ全てに対して
        .sum()
}

fn main() {
    println!("偶数の2条の合計は{}", square_sum(10))
}