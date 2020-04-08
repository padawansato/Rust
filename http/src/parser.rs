/**
 *  HTTPリクエストパーサー
 * バイト列を受け取って、パースし、その結果を返すpaerse関数。
 */
use std::str::from_utf8;

/**①では、ParseResultという列挙型をジェネリクスとして定義しています。
 * このParseResultは、パース結果を包むのに使います。
 * パーサでは、成功と失敗のほかに「パースの途中で入力が終わってしまった」もあり、
 * Resultが使えないので、このように自前で定義しています。 */

pub enum ParseResult<T> {
    Complete(T),
    Partial,
    Error,
}

/**
 * ParseResult型の実装では、is_completeとis_partialというメソッドを定義しておきます（②）。
 * これらのメソッドにはpubがついていないため、上位のモジュールからは見えません。
 * メソッド定義の中でuse self::ParseResult::*;としているのは
 * この列挙型ParseResultの列挙子をインポートするという意味です（このselfはparserモジュールを指しています）。
 * こうすることで、以降ではParseResult::CompleteなどとせずにCompleteのように書けます。
 */

impl<T> ParseResult<T> {
    fn is_complete(&self) -> bool {
        use self::ParseResult::*;
        match *self {
            Complete(_) => true,
            _ => false,
        }
    }

    fn is_partial(&self) -> bool {
        use self::ParseResult::*;
        match *self {
            Partial => true,
            _ => false,
        }
    }
}

/**
 * 標準ライブラリのFromトレイトをParseResultに実装しておきます（③）。
 * 普通のResultをParseResultに変換できるようになります。
 */

impl<T, E> From<Result<T, E>> for ParseResult<T> {
    fn from(r: Result<T, E>) -> Self {
        use self::ParseResult::*;
        match r {
            Ok(t) => Complete(t),
            Err(_) => Error,
        }
    }
}

/**④のRequestは、パース結果を表す構造体です。
 * ここではタプル構造体として定義しました。
 * 構造体のフィールドも上位のモジュールから利用するので、
 * フィールドにもpubを付けています。
 * 構造体の定義で'aというパラメータが使われているのは、
 * &strのライフタイムを越えてRequestが参照されないように、
 * 参照のライフタイムを明示するためです。
 * このような'で前置されたパラメータをライフタイムパラメータといいます。
 * ライフタイムも、型と同じようにして、プログラム上で扱えるというわけです。
*/

pub struct Request<'a>(pub &'a str);

/**
 * 最後に、parse関数を定義しています。
 */

pub fn parse(mut buf: &[u8]) -> ParseResult<Request> {
    use self::ParseResult::*; // ★

    // b".." は、バイト列リテラル
    let get = b"GET ";
    let end = b"\r\n";
    // GET がこなければエラー
    if !buf.starts_with(get) {
        return Error;
    }

    // GET をパースした残りは、パスネームと\r\n
    buf = &buf[get.len()..];
    if buf.ends_with(end) {
        buf = &buf[0..buf.len() - end.len()]
    } else {
        // 末尾が\r\nでなければ、入力が完了していないとみなす
        // 本当は途中に\r\nがある可能性もあるが、簡単のためスルー
        return Partial;
    }

    // from_utf8で、&[u8]から&strが作れる。データのコピーはしない
    // ただし失敗するかもしれないので、返り値はResultに包まれる
    from_utf8(buf)
        // タプル構造体は関数としても扱える
        // Result<&str, Utf8Error> -> Result<Request, Utf8Error>
        .map(Request)
        // Fromを実装したのでIntoのintoメソッドが自動で実装されている
        // Result<Request, Utf8Error> -> ParseResult<Request>
        .into()
}

#[test]
fn http09_get_success_foo_bar() {
    let req = b"GET /foo/bar\r\n";
    let res = parse(req);
    assert!(res.is_complete());
}

#[test]
fn http09_get_partial_root() {
    let req = b"GET /\r";
    let res = parse(req);
    assert!(res.is_partial());
}

// テストに`should_panic`アトリビュートをつけることでパニックしたらok、しなかったらfailとなる
#[test]
#[should_panic]
fn http09_post_failure() {
    let req = b"POST /\r\n";
    let res = parse(req);
    assert!(res.is_complete());
}
