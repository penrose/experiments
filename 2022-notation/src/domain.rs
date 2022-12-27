use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{eof, map, opt, value, verify},
    error::ParseError,
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

use crate::util::{_c_, identifier, nl, rules, string, _0, _1};

fn is_keyword(s: &str) -> bool {
    match s {
        "type" | "constructor" | "function" | "predicate" | "notation" | "symmetric" => true,
        _ => false,
    }
}

fn kw<'a, E: ParseError<&'a str>>(
    keyword: &'static str,
) -> impl FnMut(&'a str) -> IResult<&str, (), E> {
    assert!(is_keyword(keyword));
    value((), tag(keyword))
}

#[derive(Debug)]
pub enum Stmt<'a> {
    Type(&'a str, Option<&'a str>),
    Subtype(&'a str, &'a str),
    Function(&'a str, Vec<&'a str>, &'a str),
    Predicate(bool, &'a str, Vec<&'a str>),
    Notation(String, String),
}

rules!(
    fn id() -> &str {
        verify(identifier, |name| !is_keyword(name))
    }

    fn arg() -> &str {
        terminated(id, opt(pair(_1, id)))
    }

    fn args_list() -> Vec<&str> {
        delimited(
            char('('),
            separated_list0(char(','), delimited(_0, arg, _0)),
            char(')'),
        )
    }

    fn type_decl() -> Stmt {
        let sup = opt(preceded(delimited(_0, tag("<:"), _0), id));
        map(
            tuple((kw("type"), _1, id, sup, nl)),
            |(_, _, name, supertype, _)| Stmt::Type(name, supertype),
        )
    }

    fn predicate() -> Stmt {
        let sym = opt(pair(kw("symmetric"), _1));
        map(
            tuple((sym, kw("predicate"), _1, id, _0, args_list, nl)),
            |(symmetric, _, _, name, _, args, _)| Stmt::Predicate(symmetric.is_some(), name, args),
        )
    }

    fn function() -> Stmt {
        let keyword = alt((kw("function"), kw("constructor")));
        map(
            tuple((keyword, _1, id, _0, args_list, _0, tag("->"), _0, arg, nl)),
            |(_, _, name, _, args, _, _, _, output, _)| Stmt::Function(name, args, output),
        )
    }

    fn notation() -> Stmt {
        map(
            tuple((kw("notation"), _0, string, _0, char('~'), _0, string, nl)),
            |(_, _, from, _, _, _, to, _)| Stmt::Notation(from, to),
        )
    }

    fn subtype() -> Stmt {
        map(
            tuple((id, _0, tag("<:"), _0, id, nl)),
            |(subtype, _, _, _, supertype, _)| Stmt::Subtype(subtype, supertype),
        )
    }

    fn stmt() -> Stmt {
        alt((type_decl, predicate, function, notation, subtype))
    }

    pub fn parse() -> Vec<Stmt> {
        terminated(many0(preceded(_c_, stmt)), pair(_c_, eof))
    }
);
