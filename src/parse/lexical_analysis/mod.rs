mod error;
pub use error::*;

mod token;
pub use token::*;

#[cfg(test)]
mod test;

use std::mem;

pub fn parse(text: &str) -> Result<Vec<Token>, LexError> {
    enum State {
        InList,
        InAtom,
        InString,
        InNumber,
        SlashQuote,
    }

    let mut state = State::InList;
    let mut buf = String::new();
    let mut ret = Vec::new();

    for c in text.chars() {
        match (&state, c) {
            (_, '\'') => ret.push(Token::Quote),

            (State::InList, ' ')
            | (State::InList, '\t')
            | (State::InList, '\n')
            | (State::InList, '\r') => (),

            (State::InList, '(') => ret.push(Token::OpenList),

            (State::InList, ')') => ret.push(Token::CloseList),

            (State::InList, '"') => state = State::InString,

            (State::InList, n) if could_be_number(n) => {
                state = State::InNumber;
                buf.push(n);
            },

            (State::InList, c) => {
                state = State::InAtom;
                buf.push(c);
            },

            (State::InAtom, ' ')
            | (State::InAtom, '\t')
            | (State::InAtom, '\n')
            | (State::InAtom, '\r') => {
                ret.push(Token::Symbol(mem::replace(&mut buf, String::new())));
                state = State::InList;
            },

            (State::InAtom, '(') => {
                ret.push(Token::Symbol(mem::replace(&mut buf, String::new())));
                ret.push(Token::OpenList);
                state = State::InList;
            }

            (State::InAtom, ')') => {
                ret.push(Token::Symbol(mem::replace(&mut buf, String::new())));
                ret.push(Token::CloseList);
                state = State::InList;
            }

            (State::InAtom, c) => buf.push(c),

            (State::SlashQuote, c) => {
                buf.push(c);
                state = State::InString;
            }

            (State::InString, '"') => {
                ret.push(Token::String(mem::replace(&mut buf, String::new())));
                state = State::InList;
            },

            (State::InString, '\\') => state = State::SlashQuote,

            (State::InString, c) => buf.push(c),

            (State::InNumber, ' ')
            | (State::InNumber, '\t')
            | (State::InNumber, '\n')
            | (State::InNumber, '\r') => {
                let num = mem::replace(&mut buf, String::new())
                    .parse::<isize>().unwrap();
                ret.push(Token::Number(num));
                state = State::InList;
            },

            (State::InNumber, '(') => {
                let num = mem::replace(&mut buf, String::new())
                    .parse::<isize>().unwrap();
                ret.push(Token::Number(num));
                ret.push(Token::OpenList);
                state = State::InList;
            }

            (State::InNumber, ')') => {
                let num = mem::replace(&mut buf, String::new())
                    .parse::<isize>().unwrap();
                ret.push(Token::Number(num));
                ret.push(Token::CloseList);
                state = State::InList;
            }

            (State::InNumber, n) if n.is_numeric() => buf.push(c),

            (State::InNumber, c) => {
                buf.push(c);
                state = State::InAtom;
            }
        }
    }

    match state {
        State::InAtom => ret.push(Token::Symbol(mem::take(&mut buf))),
        State::InNumber => ret.push(
            Token::Number(mem::take(&mut buf).parse::<isize>().unwrap())
        ),
        State::InString => {
            let bad_string = format!("\"{}", buf);
            let index = text.find(&bad_string).unwrap();
            let pre_bad = &text[0..index];
            let line = pre_bad.matches("\n").count() + 1;
            let column = index - pre_bad.rfind("\n").unwrap();

            return Err(
                LexError::unterminated_string_error(bad_string, line, column)
            )
        }
        _ => (),
    }

    if ret.first() != Some(&Token::OpenList) {
        return Err(LexError::no_root_list_error())
    }

    Ok(ret)
}

pub fn could_be_number(c: char) -> bool {
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-'].contains(&c)
}
