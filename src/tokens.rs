#[derive(Clone, PartialEq)]
pub enum Tokens {
    Bang,
    Equal,
    Minus,
    Add,
    Multiply,
    Subtract,
    Divide,
    Delimiter,
    SemiColon,
    Colon,
    Space,
    NewLine,
    Ident,
}

#[derive(Clone, PartialEq)]
pub struct TokensStruct {
    pub token: Tokens,
    pub char: char,
    pub line: u128,
    pub char_pos: u128,
}

impl TokensStruct {
    pub fn new(token: Tokens, char: char, line: u128, char_pos: u128) -> Self {
        Self {
            token,
            char,
            line,
            char_pos,
        }
    }
}

impl Into<Tokens> for String {
    fn into(self) -> Tokens {
        match self.as_str() {
            "!" => Tokens::Bang,
            "}" | "{" | "]" | "[" | ")" | "(" => Tokens::Delimiter,
            " " => Tokens::Space,
            "+" => Tokens::Add,
            "*" => Tokens::Multiply,
            "-" => Tokens::Subtract,
            ";" => Tokens::SemiColon,
            ":" => Tokens::Colon,
            "\n" => Tokens::NewLine,
            _ => Tokens::Ident,
        }
    }
}

pub trait SameVecType {
    fn same_type(self) -> bool;
}

impl SameVecType for Vec<TokensStruct> {
    fn same_type(self) -> bool {
        let mut last_item = self[0].clone();
        let mut result = true;
        let _ = self.iter().map(move |f| {
            let cloned = f.clone();
            if last_item != f.clone() {
                let res_ref = &mut result;
                *res_ref = false;
            }
            last_item = cloned;
        });
        return result;
    }
}
