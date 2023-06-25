use nom::{
    branch::alt,
    bytes::complete::take_till,
    character::complete::{char, digit1, line_ending, not_line_ending, tab},
    combinator::{consumed, map, map_res, opt},
    error::{Error, ErrorKind, ParseError},
    multi::separated_list1,
    number::complete::double as float,
    sequence::{separated_pair, tuple},
    Err, IResult,
};

/// Parser
pub struct Parser;

impl Parser {
    pub fn parse(input: &str) -> Result<Vec<Parsed>, Err<Error<String>>> {
        parse(input).map_err(|error| error.to_owned())
    }
}

pub fn parse(input: &str) -> Result<Vec<Parsed>, Err<Error<&str>>> {
    let (input, parsed) = alt((
        // Quadruple
        separated_list1(
            line_ending,
            map(
                tuple((
                    take_till(is_tab),
                    consumed(tab),
                    separated_pair(unsigned, char(':'), unsigned),
                    consumed(tab),
                    opt(float),
                    consumed(tab),
                    opt(float),
                    consumed(tab),
                    opt(float),
                )),
                |(first, _, second, _, third, _, fourth, _, fifth)| {
                    Parsed::All(
                        first.trim().to_owned(),
                        second,
                        third.unwrap_or_default(),
                        fourth.unwrap_or_default(),
                        fifth.unwrap_or_default(),
                    )
                },
            ),
        ),
        separated_list1(
            line_ending,
            map(
                tuple((
                    take_till(is_tab),
                    consumed(tab),
                    separated_pair(unsigned, char(':'), unsigned),
                    consumed(tab),
                    float,
                    consumed(tab),
                    float,
                    consumed(tab),
                    float,
                )),
                |(first, _, second, _, third, _, fourth, _, fifth)| {
                    Parsed::All(first.trim().to_owned(), second, third, fourth, fifth)
                },
            ),
        ),
        // Single
        separated_list1(
            line_ending,
            map(
                separated_pair(unsigned, char(':'), unsigned),
                Parsed::Integers,
            ),
        ),
        separated_list1(line_ending, map(float, Parsed::Float)),
        separated_list1(
            line_ending,
            map(not_line_ending, |first: &str| {
                Parsed::String(first.trim().to_owned())
            }),
        ),
    ))(input)?;
    if !input.is_empty() {
        return Err(Err::Error(Error::from_error_kind(
            input,
            ErrorKind::NonEmpty,
        )));
    }
    Ok(parsed)
}

fn is_tab(c: char) -> bool {
    c == '\t'
}

fn unsigned(input: &str) -> IResult<&str, u8> {
    map_res(digit1, str::parse)(input)
}

/// Parsed
#[derive(Clone, Debug)]
pub enum Parsed {
    All(String, (u8, u8), f64, f64, f64),
    String(String),
    Integers((u8, u8)),
    Float(f64),
}

#[cfg(test)]
mod tests {
    use super::parse;
    use anyhow::Result;

    #[test]
    fn single() -> Result<()> {
        let input = "Myr\nPam\nPol\nPvc\nSte\nOle\nVac\nLin\nAra\nLnn\nGad";
        parse(input)?;
        let input = "1862684\n73601110\n682851\n728781\n81055180\n42801155\n3965364\n234026993\n692503\n126940011\n754173";
        parse(input)?;
        let input = "1862684.0\n73601110.0\n682851.0\n728781.0\n81055180.0\n42801155.0\n3965364.0\n234026993.0\n692503.0\n126940011.0\n754173";
        parse(input)?;
        let input = "14:0\n16:0\n16:1\n16:1\n18:0\n18:1\n18:1\n18:2\n20:0\n18:3\n20:1";
        parse(input)?;
        Ok(())
    }

    #[test]
    fn quadruple() -> Result<()> {
        let input = "Myr\t14:0\t1862684\t17704879\nPam\t16:0\t73601110\t670424971\nPol\t16:1\t682851\t6525447\nPvc\t16:1\t728781\t12238542\nSte\t18:0\t81055180\t594773905\nOle\t18:1\t42801155\t394892314\nVac\t18:1\t3965364\t37695042\nLin\t18:2\t234026993\t1411132104\nAra\t20:0\t692503\t6000303\nLnn\t18:3\t126940011\t866277520\nGad\t20:1\t754173\t9072411";
        parse(input)?;
        Ok(())
    }
}
