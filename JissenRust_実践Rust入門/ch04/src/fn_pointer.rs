/**
 * 型ポインタ 型のエイリアス的な？
 * 変数は値に値に付く別名だから、関数に変数をつける。これが関数ポインタ
 */
fn double(n:i32) -> i32 {
    n + n
}
fn abs(n:i32) -> i32 {
    if n > 0 {n} else {-n}
}

fn main() {
    let mut f:
        fn(i32) -> i32 = double;
    assert_eq!(f(-42), -84);

    f = abs;
    assert_eq!(f(-42), 42);

    // fのサイズ8byte
    println!("std: {:?}", std::mem::size_of_val(&f));
    println!("std: {:?}", std::mem::size_of::<usize>());
    assert_eq!(std::mem::size_of_val(&f), std::mem::size_of::<usize>());

}