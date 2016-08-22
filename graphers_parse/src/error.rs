use std::fmt;
use std::error::Error as StdError;
use tok;

type ParseError<'input> = ::lalrpop_util::ParseError<usize, tok::Tok<'input>, tok::Error>;

#[derive(Debug)]
pub struct Error<'input>(ParseError<'input>);

impl<'input> Error<'input> {
    pub fn span(&self) -> Option<(usize, usize)> {
        use ::lalrpop_util::ParseError::*;

        match &self.0 {
			&InvalidToken { location } => {
                Some((location, location + 1))
            }

			&UnrecognizedToken { token: Some((from, _, to)), expected: _ } => {
                Some((from, to))
            }

			&UnrecognizedToken { token: None, expected: _ } => {
                None
            }

			&ExtraToken { token: (from, _, to) } => {
                Some((from, to))
            }

			&User { error: tok::Error { location: from, code: _ } } => {
                Some((from, from+1))
            }
		}
    }
}

impl<'input> fmt::Display for Error<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ::lalrpop_util::ParseError::*;

        match &self.0 {
			&InvalidToken { location: _ } => {
                write!(f, "found invalid token")
            }

			&UnrecognizedToken { token: Some((_, ref tok, _)), ref expected } => {
                write!(f, "found unexpected token {:?}, expected one of: {}", tok, expected.join(", "))
            }

			&UnrecognizedToken { token: None, ref expected } => {
                write!(f, "found end of input, expected one of: {}", expected.join(", "))
            }

			&ExtraToken { token: (_, ref tok, _) } => {
                write!(f, "found unexpected token {:?}, expected end of input", tok)
            }

			&User { error: tok::Error { location: _, code: tok::ErrorCode::UnrecognizedToken } } => {
                write!(f, "found invalid token")
            }

			&User { error: tok::Error { location: _, code: tok::ErrorCode::UnterminatedStringLiteral } } => {
                write!(f, "found unterminated string literal")
            }
		}
    }
}

impl<'input> From<ParseError<'input>> for Error<'input> {
    fn from(value: ParseError<'input>) -> Error<'input> {
        Error(value)
    }
}

impl<'input> StdError for Error<'input> {
    fn description(&self) -> &'static str {
        "failed to parse as GraphQL"
    }
}
