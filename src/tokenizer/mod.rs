pub mod token;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, char},
    multi::many1,
    number::complete::float,
    sequence::delimited,
};

use token::Token;

fn parse_ident(input: &str) -> IResult<&str, Token> {
    let (remaining, parsed) = alpha1(input)?;
    Ok((remaining, Token::Ident(parsed.to_string())))
}

fn parse_number(input: &str) -> IResult<&str, Token> {
    let (remaining, parsed) = float(input)?;
    Ok((remaining, Token::Number(parsed)))
}

fn parse_string_literal(input: &str) -> IResult<&str, Token> {
    let (remaining, parsed) = delimited(char('"'), take_until("\""), char('"')).parse(input)?;
    Ok((remaining, Token::Literal(parsed.to_string())))
}

fn parse_eq(input: &str) -> IResult<&str, Token> {
    let (remaining, _) = tag("=")(input)?;
    Ok((remaining, Token::Eq))
}

fn parse_space(input: &str) -> IResult<&str, Token> {
    let (remaining, _) = tag(" ")(input)?;
    Ok((remaining, Token::Space))
}

fn parse_operators(input: &str) -> IResult<&str, Token> {
    let parsers = (
        char('+'),
        char('-'),
        char('*'),
        char('/'),
        char('.'),
        char(','),
    );
    let (remaining, parsed) = alt(parsers).parse(input)?;

    let token = match parsed {
        '+' => Token::Add,
        '-' => Token::Subtract,
        '*' => Token::Multiply,
        '/' => Token::Divide,
        '.' => Token::Dot,
        ',' => Token::Comma,
        _ => unreachable!(),
    };

    Ok((remaining, token))
}

fn parse_keywords(input: &str) -> IResult<&str, Token> {
    let parsers = (tag("func"), tag("class"));
    let (remaining, parsed) = alt(parsers).parse(input)?;

    let token = match parsed {
        "func" => Token::Func,
        "class" => Token::Class,
        _ => unreachable!(),
    };

    Ok((remaining, token))
}

fn parse_brackets(input: &str) -> IResult<&str, Token> {
    let parsers = (char('('), char(')'), char('{'), char('}'));
    let (remaining, parsed) = alt(parsers).parse(input)?;

    let token = match parsed {
        '(' => Token::LeftParen,
        ')' => Token::RightParen,
        '{' => Token::LeftCurlyParen,
        '}' => Token::RightCurlyParen,
        _ => unreachable!(),
    };

    Ok((remaining, token))
}

fn parse_eol(input: &str) -> IResult<&str, Token> {
    let (remaining, _) = char(';')(input)?;
    Ok((remaining, Token::EoL))
}

pub fn parse_line(input: &str) -> Vec<Token> {
    let parsers = (
        parse_keywords,
        parse_operators,
        parse_ident,
        parse_eq,
        parse_number,
        parse_space,
        parse_eol,
        parse_string_literal,
        parse_brackets,
    );
    let mut parser = many1(alt(parsers));
    let (_, tokens) = parser.parse(input).unwrap();

    tokens
}
