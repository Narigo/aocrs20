#[cfg(test)]
mod test {
    use super::*;
    use crate::util::*;

    #[test]
    fn check_example() {
        let file = read_file("./src/day_06/example.txt");
        let result = split_into_parts(&file);
        let expected = vec!["abc", "a\nb\nc", "ab\nac", "a\na\na\na", "b"];
        assert_eq!(expected, result);
        unimplemented!()
    }
}
