/** 
 * 借用とは所有権を自分に残したまま、値を他人に貸すことができること。
 * 「参照型はT型の借用とも呼ばれる」 
 * ミュータブルな借用は使っていい貸し借り、
 * イミュータブルな借用は見せるだけの貸し借り、
 * ミュータブルな借用は一度に一つ
 */

 fn ref_string(s: &String){
    println!("{}", s);
 }

 fn refmut_string(s: &mut String){
     // ここでsに対して変更を加えるなどの操作も可能。
     println!("{}", s);
 }

 fn main(){
    ref_tmp();
    refmut_tmp();
 }

 fn ref_tmp(){
     let s = "this is a recource".to_string();
     // 参照1つ目
     let t = &s;
     // 参照2つ目、同時に２つ存在できる。
     ref_string(&s);
 }
 
 fn refmut_tmp(){
     let mut s = "this is a recource".to_string();
     // ミュータブルな参照1つ目
     let t = &mut s;
     // ミュータブルな参照2つ目はエラー
     refmut_string(&s); 
 }