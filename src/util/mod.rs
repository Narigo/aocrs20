use std::fs;

pub fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

pub fn split_into_parts(content: &String) -> Vec<String> {
    let mut parts: Vec<String> = Vec::new();
    let mut last_part = String::new();
    for line in content.lines() {
        if line == "" {
            parts.push(last_part.to_owned());
            last_part = String::new();
            continue;
        }
        if last_part != "" {
            last_part.push_str("\n");
            last_part.push_str(line);
        } else {
            last_part.push_str(line);
        }
    }
    if last_part != "" {
        parts.push(last_part.to_owned());
    }
    parts
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_into_parts() {
        let content = "a\nb\n\nc\nd\n";
        let result = split_into_parts(&content.to_owned());
        println!("{:?}", result);
        assert_eq!(2, result.len(), "Should have two elements");
        assert_eq!("a\nb", result[0], "First element should be a\nb");
        assert_eq!("c\nd", result[1], "Second element should be c\nd");
    }
}
