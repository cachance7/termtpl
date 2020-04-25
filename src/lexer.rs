use std::convert::From;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Copy, Clone, Debug)]
pub enum Tok<'input> {
    Symbol(&'input str),
    CurlyOpen,
    CurlyClose,
    Space,
    Linefeed,
}

impl<'input> From<Tok<'input>> for String {
    fn from(t: Tok<'input>) -> String {
        match t {
            Tok::Linefeed => String::from("\n"),
            Tok::CurlyClose => String::from("}"),
            Tok::CurlyOpen => String::from("{"),
            Tok::Space => String::from(" "),
            Tok::Symbol(s) => String::from(s),
        }
    }
}

use std::str::CharIndices;

pub struct Lexer<'input> {
    chars: std::iter::Peekable<CharIndices<'input>>,
    input: &'input str,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices().peekable(),
            input,
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok<'input>, usize, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some((_, '\t')) => continue, // Tabs not supported
                Some((i, ' ')) => return Some(Ok((i, Tok::Space, i + 1))),
                Some((i, '\n')) => return Some(Ok((i, Tok::Linefeed, i + 1))),
                Some((i, '}')) => return Some(Ok((i, Tok::CurlyClose, i + 1))),
                Some((i, '{')) => return Some(Ok((i, Tok::CurlyOpen, i + 1))),

                None => return None, // End of file
                Some((i, _)) => loop {
                    match self.chars.peek() {
                        Some((j, '}')) | Some((j, '{')) | Some((j, '\n')) | Some((j, ' ')) => {
                            return Some(Ok((i, Tok::Symbol(&self.input[i..*j]), *j)))
                        }
                        None => {
                            return Some(Ok((
                                i,
                                Tok::Symbol(&self.input[i..]),
                                self.input.len(),
                            )))
                        }
                        _ => {}
                    }
                    self.chars.next();
                },
            }
        }
    }
}
