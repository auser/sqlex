use crate::parser::statements::Insert;
use crate::parser::types::InsertValue;
use crate::rules::get_struct_by_name;
use crate::settings::MaskingConfig;
use pii_masker_pii::similarity;

pub struct Transform<'a> {
    pub config: &'a MaskingConfig,
}

impl<'a> Transform<'a> {
    pub fn new(config: &'a MaskingConfig) -> Self {
        Self { config }
    }

    pub fn mask_dml_stmts(&self, dmls: &mut [Insert]) {
        for stmt in dmls {
            let columns: &[String] = &stmt.column_names;
            let values: &[InsertValue] = &stmt.values[0].0;
            let values_str: Vec<String> = values.iter().map(|v| v.to_string().replace('\'', "")).collect();
            
            let masked = columns
                .iter()
                .zip(values_str.iter())
                .map(|(column, value)| {
                    if self.config.filter_column(value) || self.config.filter_column(column) {
                        let rule = get_struct_by_name(column);
                        let masked_value = rule.fake();
                        (column.clone(), masked_value.clone())
                    } else {
                        (column.clone(), value.clone())
                    }
                })
                .collect::<Vec<(String, String)>>();

            stmt.values[0].0 = masked.iter().cloned().map(|(_, value)| InsertValue::Text { value }).collect::<Vec<InsertValue>>();
        }
    }
}

mod tests {
    use super::*;
    use crate::parser::statements::Insert;
    use crate::parser::MySqlParser;
    use crate::{parser::Rule, settings::parse_masking_config};
    use pest::Parser;
    use regex::Regex;
    use std::collections::HashMap;

    #[test]
    fn test_mask_dml_stmts() {
        let config = parse_masking_config("./tests/more.yaml");
        let cfg = config.unwrap();
        let transform = Transform::new(&cfg);
        let mut dmls = vec![Insert::from(
            MySqlParser::parse(
                Rule::INSERT_STATEMENT,
                "INSERT INTO `my_table` (`contact`, `email`) VALUES ('John Doe', 'jdoe@gmail.com');",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        )];
        transform.mask_dml_stmts(dmls.as_mut_slice());
        let email_regex = Regex::new(
            &cfg.patterns
                .iter()
                .find(|p| p.name.as_ref().unwrap() == "email")
                .unwrap()
                .regex,
        )
        .unwrap();

        assert_ne!(dmls[0].values[0].0[0], InsertValue::Text { value: "John Doe".to_owned() });
        assert_ne!(dmls[0].values[0].0[1], InsertValue::Text { value: "jdoe@gmail.com".to_owned() });
        assert!(email_regex.is_match(&dmls[0].values[0].0[1].to_string().replace('\'', "")));
    }
}
