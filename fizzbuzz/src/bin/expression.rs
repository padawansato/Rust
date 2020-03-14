/** 
 * if文とif式があり、Rustはif式 
 * 式は答えがあるように戻り値がある。
 * 命令文は戻り値がない。
 * つまり、if式だけを書いても、戻り値があるから、結果を関数に渡せる。
 * */ 

 // 階乗を計算する。
fn factorial(n: usize) -> usize{
    // ifは式なので関数の最後に置くと値を返せる。
    if n == 0 {
        println!("1");
        1
    } else {
        println!("{}", n);
        n * factorial(n - 1)    
    }
}

fn main(){
    // `;`をつけると文になり、コンパイルが通る。
    println!("10の階乗は {}", factorial(10));
}