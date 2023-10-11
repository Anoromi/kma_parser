use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    error::{ErrorKind, ParseError},
    multi::{many1, separated_list0},
    AsChar, IResult, InputTakeAtPosition,
};

pub fn alphabetic<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position1_complete(|item| !item.as_char().is_alphabetic(), ErrorKind::IsA)
}

pub fn parse_title_specialties(value: &str) -> IResult<&str, Vec<String>> {
    let value = value.trim();

    let (input, result) = separated_list0(
        many1(alt((tag(" "), tag(","), tag("+")))),
        many1(alt((alphabetic, is_a(".")))),
    )(value)?;

    Ok((
        input,
        result
            .into_iter()
            .map(|v| {
                v.iter().fold(String::new(), |mut pr, ne| {
                    pr.push_str(ne);
                    pr
                })
            })
            .collect(),
    ))

    //todo!()
}

#[cfg(test)]
mod tests {
    use super::parse_title_specialties;

    #[test]
    fn test_parse_title_specialties() {
        {
            let value = parse_title_specialties("a+b+c");
            assert_eq!(
                value,
                Ok(("", vec!["a".to_string(), "b".to_string(), "c".to_string()]))
            );
        }
        {
            let value = parse_title_specialties("a + bbc ,c ");
            assert_eq!(
                value,
                Ok((
                    "",
                    vec!["a".to_string(), "bbc".to_string(), "c".to_string()]
                ))
            );
        }
        {
            let value = parse_title_specialties("a, bbc., c");
            assert_eq!(
                value,
                Ok((
                    "",
                    vec!["a".to_string(), "bbc.".to_string(), "c".to_string()]
                ))
            );
        }
        {
            let value = parse_title_specialties("екон.+мен.");
            assert_eq!(
                value,
                Ok(("", vec!["екон.".to_string(), "мен.".to_string()]))
            );
        }

        {
            let value = parse_title_specialties("макр.");
            assert_eq!(
                value,
                Ok(("", vec!["макр.".to_string()]))
            );
        }
    }
}
