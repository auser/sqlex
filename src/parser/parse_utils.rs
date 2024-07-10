use pest::iterators::Pair;

use super::Rule;

pub fn trimmed_str<'a>(pair: Pair<'a, Rule>) -> String {
    pair.as_str().trim_matches('`').to_string()
}
