/**
 * copy型の参照とは湯水の如く使える制限なしの参照
 */
fn main(){
    let x = 1;
    //所有権がムーブしそうだが、
    let y = x;
    println!("{:?}", y);
    // 数値はcopyな値なので、一度使った後も使える。
    println!("{:?}", x);


    // &strもstrへの参照でcopyな値。
    let a = "abc";
    let b = a;
    println!("{}", a);
    
}
