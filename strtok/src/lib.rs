
fn strtok<'a>(s: &'_ mut &'a str, delimiter: char) -> &'a str
{
    if let Some(i) = s.find(delimiter)  {
        let prefix = &s[..i];
        let suffix = &s[i+delimiter.len_utf8()..];
        *s = suffix;
        prefix 
    }
    else {
        let prefix = *s;
        *s = ""; 
        prefix
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strtok_works() {
        let mut s = "hello world"; 
        let result = strtok(&mut s, ' ');
        assert_eq!(result, "hello");
        let result = strtok(&mut s, ' ');
        assert_eq!(result, "world");
        let result = strtok(&mut s, ' ');
        assert_eq!(result, "");
    }
}
