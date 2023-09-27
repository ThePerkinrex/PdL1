use thiserror::Error;

enum ErrorType {
	Lexer
}
pub struct ErrorId {
	ty: ErrorType,
	id: usize
}

#[derive(Debug, Error)]
pub enum LexerError {
	#[error(transparent)]
	Utf8(#[from] utf8_read::Error),
	#[error("Unexpected char for token start: {0}")]
	UnexpectedChar(char)
}
pub struct ErrorManager {
	line: u32,
	column: u32,
	lexer_errors: Vec<(u32, u32, LexerError)>
}

impl ErrorManager {
	pub const fn new() -> Self {
		Self {
			line: 1,
			column: 1,
			lexer_errors: Vec::new(),
		}
	}

	pub fn report_lexer_error<E>(&mut self, e: E) -> ErrorId where LexerError: From<E> {
		let id = ErrorId {ty: ErrorType::Lexer, id: self.lexer_errors.len()};
		self.lexer_errors.push((self.line, self.column, e.into()));
		id
	}

	pub fn new_line(&mut self) {
		self.line+=1;
		self.column = 1;
	}

	pub fn next_column(&mut self) {
		self.column += 1;
	}

	pub fn print_errors(self) {
		if !self.lexer_errors.is_empty() {
			println!("Lexer errors:");
		}
		for (line, column, error) in self.lexer_errors {
			println!("[{line}:{column}]: {error}")
		}
	}
}