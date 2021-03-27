// 位置情報関連のデータ型の定義
#[derive(Debug, Clone, PartalEq, Eq, Hash)]
struct Loc(usize, usize);

// loc 便利メソッド 定義　location
impl Loc {
    fn merge(&self, other: &Loc) -> Loc {
        use std::cmp::{max, min};
        Loc(min(self.0, other.0), max(self.1, other.1))
    }
}

// アノテーション　値に様々なデータをもたせたもの
// Locをもたせる
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Annot<T> {
    value: T,
    loc: Loc,
}

impl<T> Annot<T> {
    fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}

// トークンの実装
// トークンはトークンの種類に位置情報を加えたものと定義する
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TokenKind {
    // [0-9][0-9]*
    Number(u64),
    // +
    Plus,
    // -
    Minus,
    // *
    Asterisk,
    // /
    Slash,
    // (
    LParen,
    // )
    RParen,
}

// TokenKindにアノテーションをつけたものをTokenとして定義
// 型エイリアスで宣言したToken型
type Token = Annot<TokenKind>;

// しかし1つのトークンを作るのに
// Token{value:TokenKind::Plus, loc: Loc(0, 1)}
// と非常に長い記述が必要なのでヘルパー関数を定義します。
// Tokenは型エイリアスだが、データ型と同じように固有メソッドを定義できる。
impl Token {
    fn number(n: u64, loc: Loc) -> Self {
        Self::new(TokenKind::Number(n), loc)
    }
    fn plus(loc: Loc) -> Self {
        Self::new(TokenKind::Plus, loc)
    }
    fn minus(loc: Loc) -> Self {
        Self::new(Tokenkind::Minus, loc)
    }
    fn asterisk(loc: Loc) -> Self {
        Self::new(TokenKind::Asterisk, loc)
    }
    fn slash(loc: Loc) -> Self {
        Self::new(TokenKind::Slash, loc)
    }
    fn lparen(loc: Loc) -> Self {
        Self::new(TokenKind::LParen, loc)
    }
    fn rparen(loc: Loc) -> Self {
        Self::new(TokenKind::RParen, loc)
    }
}

// 字句解析エラー
// TokenKindと同じように
#[derive(Debug, Clond, PartialEq, Eq, Hash)]
enum LexErrorKind {
    InvalidChar(char),
    Eof,
}

type LexError = Annot<LexErrorKind>;

impl LexError {
    fn invalid_char(c: char, loc: Loc) -> Self {
        Self::new(LexErrorKind::InvalidChar(c), loc)
    }
    fn eof(loc: Loc) -> Self {
        Self::new(LexErrorKind::Eof, loc)
    }
}

// 字句解析器の実装
// 入力にmatch
// トークン列にして返す
fn lex(input: &str)-> Result<Vec<Token>,LexError>{
    // 解析結果を保存するベクタ
    let mut tokens = Vec::new();
    // 入力
    let input = input.as_bytes();
    // 位置を管理する値
    let mut pos = 0;
    // サブレキサを呼び出した後posを更新するマクロ
    macro_rules! lex_a_token {
        ($lexer:expr) => {
            let (tok, p) = $lexer?;
            tokens.push(tok);
            pos = p;
        };
    }
    while pos < input.len(){
        // ここでそれぞれの関数にinputとposを渡す
        match input[pos] {
            // 遷移図通りの実装
            b'0'...b'9' => lex_a_token!(lex_number(input, pos)),
            
        }
    }
}


fn main() {
    println!("Hello, world!");
}
