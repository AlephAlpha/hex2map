use base64::{encode_config, CharacterSet, Config};
use ca_rules::ParseNtHex;
use std::env::args;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct HexRule {
    rule_number: [u8; 16],
}

impl ParseNtHex for HexRule {
    fn from_bs(b: Vec<u8>, s: Vec<u8>) -> Self {
        let mut rule_number = [0; 16];
        for n in b {
            let i = (n >> 2) & !1;
            let j = 7 - (n & 7);
            rule_number[i as usize] |= 1 << j;
        }
        for n in s {
            let i = (n >> 2) | 1;
            let j = 7 - (n & 7);
            rule_number[i as usize] |= 1 << j;
        }
        HexRule { rule_number }
    }
}

impl HexRule {
    fn to_map_rule(&self) -> String {
        let config = Config::new(CharacterSet::Standard, false);
        format!("MAP{}", encode_config(self.rule_number, config))
    }
}

fn main() {
    for arg in args().skip(1) {
        if let Ok(rule) = HexRule::parse_rule(&arg) {
            println!("{}", rule.to_map_rule());
        } else {
            println!("Invalid rule: {}", arg);
        }
    }
}
