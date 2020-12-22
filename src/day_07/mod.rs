use std::collections::HashMap;

type ColoredBag = String;
type RuleSet = HashMap<ColoredBag, Vec<(u32, ColoredBag)>>;

fn file_to_rule_set(input: &String) -> RuleSet {
    let mut rules = RuleSet::new();
    for line in input.lines() {
        let re = "^(.*?) bags contain (.*).$";
        rules.insert(format!("key-{}", line), vec![(1, "???".to_owned())]);
    }
    rules
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::*;

    #[test]
    fn check_example() {
        let file = read_file("./src/day_07/example.txt");
        let rules = file_to_rule_set(&file);
        println!("{}", file);
        assert_eq!(9, rules.len());
    }
}
