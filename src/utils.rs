use nom::IResult;

pub fn str_error_to_string<T>(value: IResult<&str, T>) -> IResult<&str, T, String> {
    value.map_err(|err| err.map(|v| v.input.to_string()))
}
