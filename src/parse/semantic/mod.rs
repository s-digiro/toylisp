#[cfg(test)]
mod test;

use super::data::Syntax;
use crate::expression::{ Atom, ExRef, List };

pub fn parse(tree: Syntax) -> Result<ExRef, String> {
    match tree {
        Syntax::Atom(a) => parse_symbol(a),
        Syntax::List(l) => parse_list(l),
        Syntax::Number(n) => parse_number(n),
        Syntax::String(s) => parse_string(s),
    }
}

fn parse_symbol(symbol: String) -> Result<ExRef, String> {
    Ok(Atom::symbol(&symbol))
}

fn parse_list(list: Vec<Syntax>) -> Result<ExRef, String> {
    if list.len() == 0 {
        return Ok(List::nil())
    }

    let children = list.into_iter()
        .map(|syn| parse(syn))
        .collect::<Result<Vec<ExRef>, String>>()?;

    Ok(List::from(children))
}

fn parse_string(s: String) -> Result<ExRef, String> {
    Ok(Atom::string(&s))
}

fn parse_number(n: isize) -> Result<ExRef, String> {
    Ok(Atom::number(n))
}