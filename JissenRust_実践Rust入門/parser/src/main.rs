/// 位置情報を持つ関数を作成する.0.1の区間

#[drive(Debug, Clone, PartialEq, Eq, Hash)]
struct Loc(usize, usize);

// locに便利メソッドを実装
impl Loc {
    fn merge(&self, other: &Loc) -> Loc {
        use std::cmp::{max, min};
        Loc(min(self.0, other.0), max(self.1, other.1))
    }
}

/// アノテーション。値に様々な値に様々なデータをもたせたもの
#[drive(Debug, Clone, PartialEq, Eq, Hash)]
struct Annot<T> {
    value: T,
    log: Loc,
}

impl<T> Annot<T>{
    fn new(value: T, log:Loc)-> Self{
        Self (value,loc)
    }
}

/// トークンの作成
/// トークンはトークンの種類と位置情報のペア

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum TokenKind {
    /// [0-9][0-9]*
    Number(u64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
}
/// トークンカインドにアノテーションを付けたものをTokenとしえｔ定義する。
type Token = Annot<TokenKind>;

impl Token {
    fn number(n:u32,loc:Loc) -> Self {
        Self::okfn new() -> Self {
            
        }
        
    }
}


fn main() {
    println!("Hello, world!");
}
