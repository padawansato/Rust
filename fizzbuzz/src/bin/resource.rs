/** 
 * rustの値は資源
 * 所有権とは「値を使うとなくなる」ということ
 * 関数を呼び出したり、変数に格納したりすると、所有権が別の所有者に移る
 * 所有者がいなくなったリソースは自動で解放される。
 * GIL的な？
 * 所有権の移動はムーブと呼ぶ
*/

fn print_string(s: String){
    println!("{}", s);
    // sはこの関数の終わりで消滅する。
    // このタイミングで、sのメモリも自動で解放される。
}

fn main(){
    let s = "this is a recource".to_string();
    // 以下の行で、`s`が束縛されている文字列の所有権が`t`に移る。以後`s`は使えない。
    let t = s;
    // 以下の行で、文字列の所有権が`t`から`print_string`に移る。以後`s`は使えない。  
    print_string(t);
    // もう一度tを使おうとしてもエラー
    // print_string(t); // error[E0382]: use of moved value: `t`
    // 同じくsを使おうとしてもエラー。
    // print_string(s); // error[E0382]: use of moved value: `s`
}