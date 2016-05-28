use std::str::CharIndices;

use self::ErrorCode::*;
use self::Tok::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub location: usize,
    pub code: ErrorCode
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorCode {
    UnrecognizedToken,
    UnterminatedStringLiteral,
}

fn error<T>(c: ErrorCode, l: usize) -> Result<T,Error> {
    Err(Error { location: l, code: c })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tok<'input> {
    // Symbols
    Bang,
    Dollar,
    LParen,
    RParen,
    Ellipsis,
    Colon,
    Equals,
    At,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Pipe,

    // Keywords
    Schema,
    Query,
    Mutation,
    Type,
    Interface,
    Implements,
    On,

    // Literals
    Identifier(&'input str),
    IntValue(i32),
    // FloatValue,
    StringValue(&'input str),
}

const KEYWORDS: &'static [(&'static str, Tok<'static>)] = &[
    ("schema", Schema),
    ("query", Query),
    ("mutation", Mutation),
    ("type", Type),
    ("interface", Interface),
    ("implements", Implements),
    ("on", On),
    ];


pub struct Tokenizer<'input> {
    text: &'input str,
    chars: CharIndices<'input>,
    lookahead: Option<(usize, char)>,
    shift: usize,
}

macro_rules! eof {
    ($x:expr) => {
        match $x { Some(v) => v, None => { return None; } }
    }
}

pub type Spanned<T> = (usize, T, usize);

impl<'input> Tokenizer<'input> {
    pub fn new(text: &'input str, shift: usize) -> Tokenizer<'input> {
        let mut t = Tokenizer {
            text: text,
            chars: text.char_indices(),
            lookahead: None,
            shift: shift,
        };
        t.bump();
        t
    }


    fn literal(&mut self, idx0: usize, result: Tok<'input>) -> Result<Spanned<Tok<'input>>, Error> {
        self.bump();
        Ok((idx0, result, idx0+1))
    }

    fn next_unshifted(&mut self) -> Option<Result<Spanned<Tok<'input>>, Error>> {
        loop {
            return match self.lookahead {
                Some((idx0, '!')) => Some(self.literal(idx0, Dollar)),
                Some((idx0, '$')) => Some(self.literal(idx0, Bang)),
                Some((idx0, '(')) => Some(self.literal(idx0, LParen)),
                Some((idx0, ')')) => Some(self.literal(idx0, RParen)),
                Some((idx0, ':')) => Some(self.literal(idx0, Colon)),
                Some((idx0, '=')) => Some(self.literal(idx0, Equals)),
                Some((idx0, '@')) => Some(self.literal(idx0, At)),
                Some((idx0, '[')) => Some(self.literal(idx0, LBracket)),
                Some((idx0, ']')) => Some(self.literal(idx0, RBracket)),
                Some((idx0, '{')) => Some(self.literal(idx0, LBrace)),
                Some((idx0, '|')) => Some(self.literal(idx0, Pipe)),
                Some((idx0, '}')) => Some(self.literal(idx0, RBrace)),

                Some((idx0, '.')) => {
                    match self.bump() {
                        Some((_idx1, '.')) => {
                            match self.bump() {
                                Some((idx2, '.')) => {
                                    self.bump();
                                    Some(Ok((idx0, Ellipsis, idx2+1)))
                                }
                                _ => {
                                    Some(error(UnrecognizedToken, idx0))
                                }
                            }
                        }
                        _ => {
                            Some(error(UnrecognizedToken, idx0))
                        }
                    }
                }

                Some((idx0, '0')) => {
                    self.bump();
                    Some(Ok((idx0, IntValue(0), idx0+1)))
                }

                Some((idx0, '-')) => {
                    match self.bump() {
                        Some((idx1, '0')) => {
                            self.bump();
                            Some(Ok((idx0, IntValue(0), idx1+1)))
                        }
                        Some((idx1, c)) if is_number_start(c) => {
                            match self.number(idx1) {
                                Ok((idx1, IntValue(value), idx2)) => {
                                    Some(Ok((idx1, IntValue(-value), idx2)))
                                },
                                result => Some(result)
                            }
                        }
                        _ => {
                            println!("here {:?}", idx0);
                            Some(error(UnrecognizedToken, idx0))
                        }
                    }
                }
                Some((idx0, c)) if is_number_start(c) => {
                    Some(self.number(idx0))
                }
                Some((idx0, '"')) => {
                    self.bump();
                    Some(self.string_literal(idx0))
                }
                // byte order mark
                Some((_, '\u{FEFF}')) => {
                    self.bump();
                    continue;
                }
                Some((_, '\t')) => {
                    self.bump();
                    continue;
                }
                Some((_, ' ')) => {
                    self.bump();
                    continue;
                }
                Some((_, '\n')) => {
                    self.bump();
                    continue;
                }
                Some((_, '\r')) => {
                    self.bump();
                    continue;
                }
                Some((_, '#')) => {
                    self.bump();
                    self.take_until(|c| c == '\n');
                    continue;
                }
                // Ignore commas completely, as the GraphQL spec specifies
                Some((_, ',')) => {
                    self.bump();
                    continue;
                }
                Some((idx0, c)) if is_identifier_start(c) => {
                    Some(self.identifier(idx0))
                }
                Some((idx, _)) => {
                    Some(error(UnrecognizedToken, idx))
                }
                None => {
                    None
                }
            };
        }
    }

    fn bump(&mut self) -> Option<(usize, char)> {
        self.lookahead = self.chars.next();
        self.lookahead
    }

    fn number(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {
        let (idx0, number_str, end) = match self.take_while(is_number_continue) {
            Some(end) => (idx0, &self.text[idx0..end], end),
            None => (idx0, &self.text[idx0..], self.text.len()),
        };

        let number = number_str.parse().expect("unable to parse number");

        Ok((idx0, Tok::IntValue(number), end))
    }

    fn string_literal(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {
        // TODO: string literal value needs to be translated!
        let mut escape = false;
        let terminate = |c: char| {
            if escape {
                escape = false;
                false
            } else if c == '\\' {
                escape = true;
                false
            } else if c == '"' {
                true
            } else {
                false
            }
        };

        match self.take_until(terminate) {
            Some(idx1) => {
                self.bump(); // consume the '"'
                let text = &self.text[idx0+1..idx1]; // do not include the "" in the str
                Ok((idx0, StringValue(text), idx1+1))
            }
            None => {
                error(UnterminatedStringLiteral, idx0)
            }
        }
    }

    fn identifier(&mut self, idx0: usize) -> Result<Spanned<Tok<'input>>, Error> {
        let (start, word, end) = self.word(idx0);

        let tok =
            KEYWORDS.iter()
                    .filter(|&&(w, _)| w == word)
                    .map(|&(_, ref t)| t.clone())
                    .next()
                    .unwrap_or_else(|| Identifier(word));

        Ok((start, tok, end))
    }

    fn word(&mut self, idx0: usize) -> Spanned<&'input str> {
        match self.take_while(is_identifier_continue) {
            Some(end) => (idx0, &self.text[idx0..end], end),
            None => (idx0, &self.text[idx0..], self.text.len()),
        }
    }

    fn take_while<F>(&mut self, mut keep_going: F) -> Option<usize>
        where F: FnMut(char) -> bool
    {
        self.take_until(|c| !keep_going(c))
    }

    fn take_until<F>(&mut self, mut terminate: F) -> Option<usize>
        where F: FnMut(char) -> bool
    {
        loop {
            match self.lookahead {
                None => {
                    return None;
                }
                Some((idx1, c)) => {
                    if terminate(c) {
                        return Some(idx1);
                    } else {
                        self.bump();
                    }
                }
            }
        }
    }
}

impl<'input> Iterator for Tokenizer<'input> {
    type Item = Result<Spanned<Tok<'input>>, Error>;

    fn next(&mut self) -> Option<Result<Spanned<Tok<'input>>, Error>> {
        match self.next_unshifted() {
            None =>
                None,
            Some(Ok((l, t, r))) =>
                Some(Ok((l+self.shift, t, r+self.shift))),
            Some(Err(Error { location, code })) =>
                Some(Err(Error { location: location+self.shift, code: code })),
        }
    }
}

fn is_number_start(c: char) -> bool {
    match c {
        '1'...'9' => true, // numbers cannot start with a zero
        _ => false
    }
}

fn is_number_continue(c: char) -> bool {
    match c {
        '0'...'9' => true, // numbers cannot start with a zero
        '.' => true,
        _ => false
    }
}

fn is_identifier_start(c: char) -> bool {
    match c {
        'a'...'z' => true,
        'A'...'Z' => true,
        '_' => true, // identifier may start with underscore (see 2.1.9)
        _ => false
    }
}

fn is_identifier_continue(c: char) -> bool {
    match c {
        'a'...'z' => true,
        'A'...'Z' => true,
        '0'...'9' => true,
        '_' => true,
        _ => false
    }
}

