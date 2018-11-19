use reqwest::header::HeaderValue;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum Link {
    Next(String),
    Previous(String),
    Other(String, String),
}

impl Link {
    pub fn from_header(input: &HeaderValue) -> Result<Vec<Link>, Box<dyn Error>> {
        let value = input.to_str()?;
        let list = value
            .split(",")
            .map(Link::parse)
            .collect::<Result<_, _>>()?;

        Ok(list)
    }

    pub fn parse(input: &str) -> Result<Link, ParseError> {
        let tokens: Vec<&str> = input.split(";").map(|x| x.trim()).collect();
        let url_token = tokens.get(0).ok_or(ParseError(input.into()))?;
        let url = parse_url_token(url_token)?;
        let params = parse_param_tokens(&tokens[1..])?;

        match params.get("rel") {
            None => Err(ParseError(format!(
                "Unexpected link without rel parameter: {}",
                input
            ))),
            Some(&"next") => Ok(Link::Next(url.into())),
            Some(&rel) => Ok(Link::Other(url, rel.into())),
        }
    }

    pub fn is_next(&self) -> bool {
        match self {
            Link::Next(_) => true,
            _ => false,
        }
    }

    pub fn is_previous(&self) -> bool {
        match self {
            Link::Previous(_) => true,
            _ => false,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Link::Next(url) => &url,
            Link::Previous(url) => &url,
            Link::Other(url, _) => &url,
        }
    }
}

fn parse_param_tokens<'a>(tokens: &'a [&str]) -> Result<HashMap<&'a str, &'a str>, ParseError> {
    tokens
        .into_iter()
        .map(|&token| parse_param_token(token))
        .collect()
}

fn parse_param_token(token: &str) -> Result<(&str, &str), ParseError> {
    let pairs: Vec<&str> = token.split("=").collect();
    if pairs.len() != 2 {
        return Err(ParseError(
            "Unexpected param token. Expected \"<name>\"=<value>".into(),
        ));
    }
    let name = pairs[0];
    let value_token = pairs[1];
    if !(value_token.starts_with('"') && value_token.ends_with('"')) {
        return Err(ParseError(
            "Unexpected param token. Expected \"<name>\"=<value>".into(),
        ));
    }
    let value = value_token.get(1..(&value_token.len() - 1)).unwrap();

    Ok((name, value))
}

fn parse_url_token(token: &str) -> Result<String, ParseError> {
    if !(token.starts_with('<') && token.ends_with('>')) {
        return Err(ParseError(format!("Unexpected url token {}", token)));
    }
    let url = token.get(1..(&token.len() - 1)).unwrap();

    Ok(url.into())
}

#[derive(Debug)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_url_token() {
        assert_eq!(
            parse_url_token("</foo/bar>").unwrap(),
            "/foo/bar".to_string()
        );
    }

    #[test]
    fn invalid_url_token() {
        assert!(parse_url_token("/foo/bar").is_err());
    }

}
