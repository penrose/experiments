use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    character::complete::{
        alpha1, alphanumeric1, char, line_ending, multispace1, not_line_ending, space1,
    },
    combinator::{eof, map, recognize, value},
    error::ParseError,
    multi::{many0_count, many1_count},
    sequence::{delimited, pair},
    IResult,
};

macro_rules! rules {
    ($($(#[$m:meta])* $v:vis fn $i:ident() -> $t:ty $b:block)+) => {
        $(
            $(#[$m])*
            $v fn $i<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, $t, E> {
                let mut parser = $b;
                parser(input)
            }
        )+
    }
}

pub(crate) use rules;

rules!(
    fn endline() -> () {
        value((), alt((line_ending, eof)))
    }

    fn comment() -> () {
        value((), delimited(tag("--"), not_line_ending, endline))
    }

    fn multiline_comment() -> () {
        value((), delimited(tag("/*"), take_until("*/"), tag("*/")))
    }

    pub fn string() -> String {
        map(delimited(char('"'), is_not("\""), char('"')), String::from)
    }

    pub fn identifier() -> &str {
        // https://github.com/Geal/nom/blob/7.1.1/doc/nom_recipes.md#rust-style-identifiers
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0_count(alt((alphanumeric1, tag("_")))),
        ))
    }

    pub fn _c_() -> () {
        value(
            (),
            many0_count(alt((value((), multispace1), comment, multiline_comment))),
        )
    }

    pub fn nl() -> () {
        value((), pair(_0, alt((endline, comment))))
    }

    pub fn _1() -> () {
        value((), many1_count(alt((value((), space1), multiline_comment))))
    }

    pub fn _0() -> () {
        value((), many0_count(alt((value((), space1), multiline_comment))))
    }
);
