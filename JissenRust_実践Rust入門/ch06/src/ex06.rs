struct Circle {
    radius: u32,
}

impl Circle {
    // Circle::diameterメソッド
    fn diameter(&self) -> u32 {
        self.radius * 2
    }

    // small circle関連関数
    fn small_circle() -> Circle {
        Circle { radius: 1 }
    }
}
fn main() {
    // Circleの関連関数small_circleの呼び出し
    let circle1 = Circle::small_circle();
    println!("Circle1's diameter: {}", circle1.diameter());
}
