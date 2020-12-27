use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

type ColoredBag = String;
type RuleSet = HashMap<ColoredBag, Vec<(u32, ColoredBag)>>;
type ContainedBySet = HashSet<ColoredBag>;

fn split_amount_and_color(input: &str) -> Result<(u32, String), String> {
    let bag_re = Regex::new(r"^(\d+) (.*?)\s+bags?$").unwrap();
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

fn contained_by_from_rules(rules: &RuleSet, color: &ColoredBag) -> ContainedBySet {
    let mut values: ContainedBySet = HashSet::new();
    for entry in rules.iter() {
        let entry_values = entry.1;
        println!("entry_values: {:?}", entry_values);
        for colored_bag in entry_values {
            println!("colored_bag: {:?}", colored_bag);
            if colored_bag.1.eq(color) {
                values.insert(entry.0.clone());
                let contained_by = contained_by_from_rules(rules, entry.0);
                values = values.union(&contained_by).map(|s| s.to_owned()).collect();
            }
        }
    }
    values
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::*;

    #[test]
    fn check_rules_set() {
        let file = read_file("./src/day_07/example.txt");
        let rules = file_to_rule_set(&file).expect("Should be possible to read as rules");
        println!("{:?}", rules);
        assert_eq!(9, rules.len());
    }

    #[test]
    fn check_example() {
        let file = read_file("./src/day_07/example.txt");
        let rules = file_to_rule_set(&file).expect("Should be possible to read as rules");
        let contained_by = contained_by_from_rules(&rules, &"shiny gold".to_owned());
        println!("{:?}", contained_by);
        assert_eq!(4, contained_by.len());
    }
}
