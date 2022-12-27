use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::char,
    combinator::{eof, map, value, verify},
    error::ParseError,
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

use crate::util::{_c_, identifier, nl, rules, string, _0, _1};

fn is_keyword(s: &str) -> bool {
    match s {
        "All" | "Label" | "NoLabel" | "AutoLabel" => true,
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
pub enum Label {
    Math(String),
    Text(String),
}

#[derive(Clone, Debug)]
pub enum LabelOption<'a> {
    Default,
    IDs(Vec<&'a str>),
}

#[derive(Debug)]
pub enum Stmt<'a> {
    Decl(&'a str, Vec<&'a str>),
    Bind(Option<&'a str>, &'a str, &'a str, Vec<&'a str>),
    ApplyPredicate(&'a str, Vec<&'a str>),
    LabelDecl(&'a str, Label),
    AutoLabel(LabelOption<'a>),
    NoLabel(&'a str),
}

rules!(
    fn id() -> &str {
        verify(identifier, |name| !is_keyword(name))
    }

    fn ids() -> Vec<&str> {
        separated_list1(delimited(_0, char(','), _0), id)
    }

    fn pids() -> Vec<&str> {
        delimited(
            char('('),
            separated_list0(char(','), delimited(_0, id, _0)),
            char(')'),
        )
    }

    fn tex_literal() -> String {
        map(delimited(char('$'), is_not("$"), char('$')), String::from)
    }

    fn label_option() -> LabelOption {
        alt((
            value(LabelOption::Default, kw("All")),
            map(ids, LabelOption::IDs),
        ))
    }

    fn decl() -> Stmt {
        map(tuple((id, _1, ids, nl)), |(typ, _, names, _)| {
            Stmt::Decl(typ, names)
        })
    }

    fn bind() -> Stmt {
        map(
            tuple((id, _0, tag(":="), _0, id, _0, pids, nl)),
            |(variable, _, _, _, func, _, args, _)| Stmt::Bind(None, variable, func, args),
        )
    }

    fn decl_bind() -> Stmt {
        map(
            tuple((id, _1, id, _0, tag(":="), _0, id, _0, pids, nl)),
            |(typ, _, variable, _, _, _, func, _, args, _)| {
                Stmt::Bind(Some(typ), variable, func, args)
            },
        )
    }

    fn apply_predicate() -> Stmt {
        map(tuple((id, _0, pids, nl)), |(name, _, args, _)| {
            Stmt::ApplyPredicate(name, args)
        })
    }

    fn label_decl() -> Stmt {
        let l = alt((map(string, Label::Text), map(tex_literal, Label::Math)));
        map(
            tuple((kw("Label"), _1, id, _0, l, nl)),
            |(_, _, variable, _, label, _)| Stmt::LabelDecl(variable, label),
        )
    }

    fn no_label() -> Stmt {
        map(tuple((kw("NoLabel"), _1, id, nl)), |(_, _, arg, _)| {
            Stmt::NoLabel(arg)
        })
    }

    fn auto_label() -> Stmt {
        map(
            tuple((kw("AutoLabel"), _1, label_option, nl)),
            |(_, _, option, _)| Stmt::AutoLabel(option),
        )
    }

    fn label_stmt() -> Stmt {
        alt((label_decl, no_label, auto_label))
    }

    fn stmt() -> Stmt {
        alt((decl, bind, decl_bind, apply_predicate, label_stmt))
    }

    pub fn parse() -> Vec<Stmt> {
        terminated(many0(preceded(_c_, stmt)), pair(_c_, eof))
    }
);
