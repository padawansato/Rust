fn main() {
    let x = 1 + 2;
    let mut y = x;
    println!("y={}", y);
    y = 5;
    let z = y;
    println!("結果＝{}", z);
}


// x => 3 => 
// y => x => 3 
// z => y => x => 3 