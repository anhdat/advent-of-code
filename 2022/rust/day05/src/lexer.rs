pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Copy, Clone, Debug)]
pub enum Tok<'input> {
    Space,
    Tab,
    Linefeed,
    Num(&'input str),
    Char(&'input str),
    LitMove,
    LitFrom,
    LitTo,
    OpenSqrBrk,
    CloseSqrBrk,
    EmptyCrat,
}

#[derive(Copy, Clone, Debug)]
pub enum LexicalError {}

use std::str::CharIndices;

pub struct Lexer<'input> {
    chars: std::iter::Peekable<CharIndices<'input>>,
    input: &'input str,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices().peekable(),
            input: input,
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok<'input>, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some((_, ' ')) | Some((_, '\n')) | Some((_, '\t')) => continue,

                None => return None, // End of file
                Some((i, _)) => loop {
                    match self.chars.peek() {
                        Some((j, ')')) | Some((j, '(')) | Some((j, '+')) | Some((j, '-'))
                        | Some((j, '*')) | Some((j, '/')) | Some((j, ' ')) => {
                            return Some(Ok((i, Tok::Num(&self.input[i..*j]), *j)))
                        }
                        None => return Some(Ok((i, Tok::Num(&self.input[i..]), self.input.len()))),
                        _ => {
                            self.chars.next();
                        }
                    }
                },
            }
        }
    }
}
