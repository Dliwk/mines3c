//! Tokenizer

use std::str::CharIndices;
use unicode_xid::UnicodeXID;

use self::ErrorCode::*;
use self::Tok::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    pub location: usize,
    pub code: ErrorCode,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ErrorCode {
    UnrecognizedToken,
    UnterminatedStringLiteral,
    InvalidVariable,
    IntegerTooBig,
}

fn error<T>(c: ErrorCode, l: usize) -> Result<T, Error> {
    Err(Error {
        location: l,
        code: c,
    })
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Tok<'input> {
    // Keywords
    Fn,
    Inline,
    Return,
    Repeat,
    Loop,
    If,
    Else,
    While,
    Arg,
    Label,

    // Identifiers
    Id(&'input str),
    Var(&'input str),
    StringLiteral(&'input str),
    Integer(u32),

    // Symbols
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semi,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
    LessOrEqual,
    GreaterOrEqual,
    Bang,
    RightArrow,
}

pub struct Tokenizer<'input> {
    text: &'input str,
    chars: CharIndices<'input>,
    lookahead: Option<(usize, char)>,
    shift: usize,
}

pub type Spanned<T> = (usize, T, usize);

const KEYWORDS: &[(&str, Tok<'static>)] = &[
    ("fn", Fn),
    ("inline", Inline),
    ("return", Return),
    ("repeat", Repeat),
    ("loop", Loop),
    ("if", If),
    ("else", Else),
    ("while", While),
    ("arg", Arg),
    ("label", Label),
];

impl<'input> Tokenizer<'input> {
    pub fn new(text: &'input str, shift: usize) -> Tokenizer<'input> {
        let mut t = Tokenizer {
            text,
            chars: text.char_indices(),
            lookahead: None,
            shift,
        };
        t.bump();
        t
    }

    fn bump(&mut self) -> Option<(usize, char)> {
        self.lookahead = self.chars.next();
        self.lookahead
    }

    fn word(&mut self, idx: usize) -> Spanned<&'input str> {
        match self.take_while(is_identifier_continue) {
            Some(end) => (idx, &self.text[idx..end], end),
            None => (idx, &self.text[idx..], self.text.len()),
        }
    }

    fn take_while<F>(&mut self, mut keep_going: F) -> Option<usize>
    where
        F: FnMut(char) -> bool,
    {
        self.take_until(|c| !keep_going(c))
    }

    fn take_until<F>(&mut self, mut terminate: F) -> Option<usize>
    where
        F: FnMut(char) -> bool,
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

    // noinspection DuplicatedCode
    fn next_unshifted(&mut self) -> Option<Result<Spanned<Tok<'input>>, Error>> {
        loop {
            return match self.lookahead {
                Some((idx, '(')) => {
                    self.bump();
                    Some(Ok((idx, LeftParen, idx + 1)))
                }
                Some((idx, ')')) => {
                    self.bump();
                    Some(Ok((idx, RightParen, idx + 1)))
                }
                Some((idx, '{')) => {
                    self.bump();
                    Some(Ok((idx, LeftBrace, idx + 1)))
                }
                Some((idx, '}')) => {
                    self.bump();
                    Some(Ok((idx, RightBrace, idx + 1)))
                }
                Some((idx, ';')) => {
                    self.bump();
                    Some(Ok((idx, Semi, idx + 1)))
                }
                Some((idx, '<')) => match self.bump() {
                    Some((idx1, '=')) => {
                        self.bump();
                        Some(Ok((idx, LessOrEqual, idx1 + 1)))
                    }
                    _ => Some(Ok((idx, LessThan, idx + 1))),
                },
                Some((idx, '>')) => match self.bump() {
                    Some((idx1, '=')) => {
                        self.bump();
                        Some(Ok((idx, GreaterOrEqual, idx1 + 1)))
                    }
                    _ => Some(Ok((idx, GreaterThan, idx + 1))),
                },
                Some((idx, '!')) => match self.bump() {
                    Some((idx1, '=')) => {
                        self.bump();
                        Some(Ok((idx, NotEqual, idx1 + 1)))
                    }
                    _ => Some(Ok((idx, Bang, idx + 1))),
                },
                Some((idx, '=')) => match self.bump() {
                    Some((idx1, '=')) => {
                        self.bump();
                        Some(Ok((idx, Equal, idx1 + 1)))
                    }
                    _ => Some(error(UnrecognizedToken, idx)),
                },
                Some((idx, '-')) => match self.bump() {
                    Some((idx1, '>')) => {
                        self.bump();
                        Some(Ok((idx, RightArrow, idx1 + 1)))
                    }
                    _ => Some(error(UnrecognizedToken, idx)),
                },
                Some((idx, '$')) => {
                    self.bump();
                    Some(self.variable(idx))
                }
                Some((idx, '/')) => match self.bump() {
                    Some((_, '/')) => {
                        self.take_until(|c| c == '\n');
                        continue;
                    }
                    _ => Some(error(UnrecognizedToken, idx)),
                },
                Some((idx, '"')) => {
                    self.bump();
                    Some(self.string_literal(idx))
                }
                Some((idx, c)) if c.is_ascii_digit() => {
                    self.bump();
                    let (text, idx1) = match self.take_while(|c| c.is_ascii_digit()) {
                        Some(idx1) => (&self.text[idx..idx1], idx1),
                        None => (&self.text[idx..], self.text.len()),
                    };
                    match text.parse() {
                        Ok(val) => Some(Ok((idx, Integer(val), idx1))),
                        _ => Some(error(IntegerTooBig, idx)),
                    }
                }
                Some((idx, c)) if is_identifier_start(c) => Some(self.identifierish(idx)),
                Some((_, c)) if c.is_whitespace() => {
                    self.bump();
                    continue;
                }
                Some((idx, _)) => Some(error(UnrecognizedToken, idx)),
                None => None,
            };
        }
    }

    fn string_literal(&mut self, idx: usize) -> Result<Spanned<Tok<'input>>, Error> {
        match self.take_until(|c| c == '"') {
            // TODO escape?
            Some(idx1) => {
                self.bump();
                let text = &self.text[idx + 1..idx1];
                Ok((idx, StringLiteral(text), idx1 + 1))
            }
            None => error(UnterminatedStringLiteral, idx),
        }
    }

    fn variable(&mut self, idx: usize) -> Result<Spanned<Tok<'input>>, Error> {
        match self.lookahead {
            Some((_, c)) if is_identifier_start(c) => {
                match self.take_while(is_identifier_continue) {
                    Some(idx1) => {
                        let text = &self.text[idx + 1..idx1];
                        Ok((idx, Var(text), idx1))
                    }
                    None => Ok((idx, Var(&self.text[idx + 1..]), self.text.len())),
                }
            }
            _ => error(InvalidVariable, idx),
        }
    }

    fn identifierish(&mut self, idx: usize) -> Result<Spanned<Tok<'input>>, Error> {
        let (start, word, end) = self.word(idx);

        // If it's not a keyword then it's an Identifier
        let tok = KEYWORDS
            .iter()
            .filter(|&&(w, _)| w == word)
            .map(|(_, t)| t.clone())
            .next()
            .unwrap_or(Id(word));

        Ok((start, tok, end))
    }
}

impl<'input> Iterator for Tokenizer<'input> {
    type Item = Result<Spanned<Tok<'input>>, Error>;

    fn next(&mut self) -> Option<Result<Spanned<Tok<'input>>, Error>> {
        match self.next_unshifted() {
            None => None,
            Some(Ok((l, t, r))) => Some(Ok((l + self.shift, t, r + self.shift))),
            Some(Err(Error { location, code })) => Some(Err(Error {
                location: location + self.shift,
                code,
            })),
        }
    }
}


fn is_identifier_start(c: char) -> bool {
    c.is_xid_start() || c == '_'
}

fn is_identifier_continue(c: char) -> bool {
    c.is_xid_continue() || c == '_'
}
