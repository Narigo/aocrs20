use regex::Regex;
use std::collections::HashMap;

type ColoredBag = String;
type RuleSet = HashMap<ColoredBag, Vec<(u32, ColoredBag)>>;

fn split_amount_and_color(input: &str) -> Result<(u32, String), String> {
    let bag_re = Regex::new(r"^(\d+) (.*)\s*bags?$").unwrap();
    bag_re
        .captures(input)
        .map(|groups| {
            (
                groups
                    .get(1)
                    .expect("Should have an integer amount of bags")
                    .as_str()
                    .parse::<u32>()
                    .expect("Should be able to parse an unsigned integer"),
                groups
                    .get(2)
                    .expect("Should have a bag color")
                    .as_str()
                    .to_owned(),
            )
        })
        .ok_or(format!("Could not capture a group on '{}'", input))
}

fn file_to_rule_set(input: &String) -> Result<RuleSet, String> {
    let mut rules = RuleSet::new();
    let start_re = Regex::new(r"^(.*?) bags contain (.*)\.$").unwrap();
    for line in input.lines() {
        for groups in start_re.captures(line) {
            let key = groups
                .get(1)
                .expect("Should have a group 1")
                .as_str()
                .to_owned();
            let second_part = groups.get(2).expect("Should have a group 2").as_str();
            if second_part == "no other bags" {
                rules.insert(key, vec![]);
                continue;
            }
            let values = second_part
                .split(",")
                .map(|s| s.trim())
                .map(|s| split_amount_and_color(s).expect("Could not split into amount and color"))
                .collect();
            rules.insert(key, values);
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
