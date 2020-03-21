/**
 * 列挙型はOption型とResutl型がある
 * rustには例外がないので、その代わりにこれらを使う
 * option型は値があれば返し、なければnilというときにつかう
 * result型は計算が失敗するかもしれないときに使われる型
*/

enum Option<T> {
    // 値がないか
    None,
    // ある
    Some(T),
}

enum Result<T, E> {
    // 計算が成功したか
    Ok(T),
    // 失敗してもエラーが出たか
    Err(E),
}