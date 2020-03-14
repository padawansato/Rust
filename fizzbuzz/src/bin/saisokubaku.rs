fn main(){
    // 1つ目の"x"を束縛する。
    let x: i32 = 1;
    println!("{}", x);
    // 2つ目の"x"を束縛する。これは先のｘとは別物。
    let x: &str = "abc";
    println!("{}", x);
}