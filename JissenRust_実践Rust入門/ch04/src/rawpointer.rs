/* 
 * *const T型 不変の生ポインタ
 * *mut T型 可変の生ポインタ
 */
fn main(){
    let c1 = 'A';
    // &で参照を作り、型強制で生ポインタに変換する
    let c1_ptr: *const char = &c1;

        println!("c1: {:?}", c1);
        println!("*c1_ptr: {:?}", unsafe{*c1_ptr});

    let c2 = 'B'; // ""じゃない
    let c2_ptr: *const char = &c2;

        println!("c2: {:?}", c2);
        println!("*c2_ptr: {:?}", unsafe{*c2_ptr});

    let mut n1 = 0; //i32型
    let n1_ptr: *mut i32 = &mut n1;

        println!("n1:{:?}", n1);
        println!("n1_str:{:?}", unsafe{*n1_ptr});
    
    unsafe{
        *n1_ptr = 1_000;
        println!("n1_str:{:?}", unsafe{*n1_ptr});
    }
}