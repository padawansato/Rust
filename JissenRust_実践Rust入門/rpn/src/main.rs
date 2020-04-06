fn main() {
    // exp変数をRPN形式の文字列に束縛する
    // このRPNは数式6.1 + 5.2 x 4.3 - 3.4 / 2.5 x 1.6と等しい
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -"; // let 束縛は関数型言語でよくある

    // rpn関数を呼び出して計算する。返された値にans変数を束縛する
    let ans = rpn(exp);

    // デバッグビルド時のみ、答えが正しいかチェックする
    // 浮動小数点の計算誤差を考慮し、ここでは小数点以下4桁までの値を文字列に変換している
    debug_assert_eq!("26.2840", format!("{:.4}", ans));

    // expとansの値を表示する。ansは小数点以下4桁まで表示する
    println!("{} = {:.4}", exp, ans)
}

// RPN形式の文字列expを受け取り、f64型の計算結果を返す
fn rpn(exp: &str) -> f64 {
    // 変数stackを空のスタックに束縛する
    // stackはミュータブルな変数で、値の変更を許す
    let mut stack = Vec::new();

    // expの要素をスペースで分割し、tokenをそれらに順に束縛する
    // 要素がなくなるまで繰り返す
    for token in exp.split_whitespace() {
        // tokenがf64の数値ならスタックに積む
        if let Ok(num) = token.parse::<f64>(){ // if let Okでで条件分岐を書いている = で代入しているのではない。//ここにbreakpointを打つ
            stack.push(num);
        } else {
            // tokenが数値でないなら、演算子なのか調べる
            match token {
                // tokenが演算子ならapply2関数で計算する
                // |x,y| x+ y はクロージャ
                // 引数ｘ,ｙを取り、x＋yを計算して答えを返す
                "+" => apply2(&mut stack, |x,y| x + y),// 関数内で関数内で変更するのを許すためにmut
                "-" => apply2(&mut stack, |x,y| x - y),// stackが束縛されたベクタへの参照をする
                "*" => apply2(&mut stack, |x,y| x * y),
                "/" => apply2(&mut stack, |x,y| x / y),

                // tokenが演算子ではないなら、エラーを起こして終了する
                _=> panic!("Unknown operator: {}", token),
            }
        }
    }
// スタックから数値を1つ取り出す。失敗したらエラーを起こして終了する
stack.pop().expect("Stack underflow")
}

// スタックから数値を2つ取り出し、F型のクロージャfunで計算し、結果をスタックに積む
fn apply2<F>(stack: &mut Vec<f64>, fun: F)
// F型のトレイト境界
where
    F: Fn(f64, f64) -> f64,
{
    // 変数ｙとｘをスタックの最後の2要素に束縛する
    if let(Some(y), Some(x)) = (stack.pop(), stack.pop()){
        // クロージャfunで計算し、その結果に変数ｘｗを束縛する
        let z = fun(x,y);
        // 変数zの値をスタックに積む
        stack.push(z);
    } else {
        // スタックから要素が取り出せなかったときにはエラーを起こして終了する
        panic!("Stack underflow");
    }
}