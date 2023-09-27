use std::io::Read;

use utf8_read::{Reader, Char};

use crate::error::{ErrorManager, ErrorId, LexerError};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Initial,
    DivOrComment,
    LineComment,
    InlineComment,
    MaybeInlineCommentEnd,
    Num,
    Plus,
    Minus,
    Mul,
    S9,
	Div
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    Mas,
    Menos,
    Por,
    Div,
    Num,
}

pub struct Lexer<R: Read> {
    reader: Reader<R>,
    next_char: Char,
	num: u32
}

impl<R: Read> Lexer<R> {
    pub fn new(reader: R) -> Result<Self, utf8_read::Error> {
		let mut reader = Reader::new(reader);
        let c = reader.next_char()?;
        Ok(Self {
            reader,
            next_char: c,
			num: 0
        })
    }

    fn read(&mut self, error_mgr: &mut ErrorManager) -> Result<(), ErrorId> {
		error_mgr.next_column();
		if matches!(self.next_char, Char::Char('\n')) {
			error_mgr.new_line();
		}
		
        self.next_char = self.reader.next_char().map_err(|e| error_mgr.report_lexer_error(e))?;
        Ok(())
    }

    pub fn next_token(&mut self, error_mgr: &mut ErrorManager) -> Result<Option<(TokenKind, u32)>, ErrorId> {
        let mut state = State::Initial;
        loop {
			// println!("{state:?} {:?}", self.next_char);
            match state {
                State::Initial => match self.next_char {
                    Char::Char('+') => {
                        state = State::Plus;
                        self.read(error_mgr)?;
                    }
                    Char::Char('-') => {
                        state = State::Minus;
                        self.read(error_mgr)?;
                    }
                    Char::Char('*') => {
                        state = State::Mul;
                        self.read(error_mgr)?;
                    }
                    Char::Char('/') => {
                        state = State::DivOrComment;
                        self.read(error_mgr)?;
                    }
                    Char::Char(x) if x.is_ascii_digit() => {
                        state = State::Num;
						self.num = x.to_digit(10).unwrap();
                        self.read(error_mgr)?;
                    }
                    Char::Char(x) if x.is_whitespace() => {
                        self.read(error_mgr)?;
                    }
					Char::Eof | Char::NoData=> return Ok(None),
                    Char::Char(x) => {
						let id = error_mgr.report_lexer_error(LexerError::UnexpectedChar(x));
						self.read(error_mgr)?;
						return Err(id);
					},
                },
                State::DivOrComment => match self.next_char {
					Char::Char('/') => {
						state = State::LineComment;
						self.read(error_mgr)?;
					}
					Char::Char('*') => {
						state = State::InlineComment;
						self.read(error_mgr)?;
					}
					_ => {
						state = State::Div
					}
				},
                State::LineComment => match self.next_char {
					Char::Char('\n') => {
						state = State::Initial;
						self.read(error_mgr)?;
					}
					_ => {
						self.read(error_mgr)?;
					}
				},
                State::InlineComment => match self.next_char {
					Char::Char('*') => {
						state = State::MaybeInlineCommentEnd;
						self.read(error_mgr)?;
					}
					_ => {
						self.read(error_mgr)?;
					}
				},
                State::MaybeInlineCommentEnd => match self.next_char {
					Char::Char('/') => {
						state = State::Initial;
						self.read(error_mgr)?;
					}
					Char::Char('*') => {
						self.read(error_mgr)?;
					}
					_ => {
						state = State::InlineComment;
						self.read(error_mgr)?;
					}
				},
                State::Num => match self.next_char {
					Char::Char(x) if x.is_ascii_digit() => {
						self.num = self.num * 10 + x.to_digit(10).unwrap();
						self.read(error_mgr)?;
					}
					_ => state=State::S9
				},
                State::Plus => return Ok(Some((TokenKind::Mas, 0))),
                State::Minus => return Ok(Some((TokenKind::Menos, 0))),
                State::Mul => return Ok(Some((TokenKind::Por, 0))),
                State::S9 => return Ok(Some((TokenKind::Num, self.num))),
                State::Div => return Ok(Some((TokenKind::Div, 0))),
            }
        }
    }
}
