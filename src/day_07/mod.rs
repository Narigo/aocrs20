use regex::Regex;
use std::collections::HashMap;

type ColoredBag = String;
type RuleSet = HashMap<ColoredBag, Vec<(u32, ColoredBag)>>;

fn file_to_rule_set(input: &String) -> Result<RuleSet, String> {
    let mut rules = RuleSet::new();
    let start_re = Regex::new(r"^(.*?) bags contain (.*)\.$").unwrap();
    let bag_re = Regex::new(r"^(\d+) (.*)\s*bags?$").unwrap();
    for line in input.lines() {
        match start_re.captures(line) {
            Some(groups) => {
                let key = groups
                    .get(1)
                    .expect("Should have a group 1")
                    .as_str()
                    .to_owned();
                let values = groups
                    .get(2)
                    .expect("Should have a group 2")
                    .as_str()
                    .split(",")
                    .map(|s| s.trim())
                    .map(|s| (1, s.to_owned()));
                rules.insert(key, values.collect());
            }
            None => {}
        }
    }
    Ok(rules)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::*;

    #[test]
    fn check_example() {
        let file = read_file("./src/day_07/example.txt");
        let rules = file_to_rule_set(&file).expect("Should be possible to read as rules");
        println!("{:?}", rules);
        assert_eq!(9, rules.len());
    }
}
