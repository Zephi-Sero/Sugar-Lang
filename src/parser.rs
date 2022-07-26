use crate::{
    ast::{ASTStruct, Symbol, AST},
    tokens::{Tokens, TokensStruct},
};

use tracing::info;

pub struct Parser {
    tokens: Vec<TokensStruct>,
    syntax_tree: ASTStruct,
}

impl Parser {
    pub fn new(tokens: Vec<TokensStruct>) -> Self {
        Self {
            tokens,
            syntax_tree: ASTStruct {
                ast: AST::Block {
                    scope: 0,
                    contents: vec![],
                },
                char_pos: 0,
                line: 0,
            },
        }
    }
    pub fn run(mut self) -> ASTStruct {
        let mut last_token: Option<TokensStruct> = None;
        for x in self.tokens.clone() {
            info!("CurTokens: {:#?} {:#?}", x, last_token);
            if match last_token {
                // Check if it's an arrow
                Some(v) => v.token == Tokens::Minus && x.token == Tokens::Greater,
                None => false,
            } {
                info!("Arrow ran!");
                let block = ASTStruct::get_block(&mut self.syntax_tree).unwrap();
                block.1.push(ASTStruct {
                    ast: AST::Symbol(Symbol::Arrow),
                    line: x.line,
                    char_pos: x.char_pos,
                });
            }
            last_token = Some(x.clone());
        }
        return self.syntax_tree.clone();
    }
}
