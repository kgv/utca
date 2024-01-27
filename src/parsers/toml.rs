use crate::fatty_acid::FattyAcid;
use serde::{Deserialize, Serialize};
use std::{mem::take, str::FromStr};
use toml_edit::{
    de::{from_str, Error},
    ser::{self, to_document},
    value,
    visit_mut::{visit_inline_table_mut, visit_item_mut, visit_value_mut, VisitMut},
    InlineTable, Item, Key, Value,
};

pub fn to_string<T: Serialize + ?Sized>(value: &T) -> Result<String, ser::Error> {
    let mut document = to_document(value)?;
    Visitor.visit_document_mut(&mut document);
    Ok(document.to_string())
}

/// Parsed
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Parsed {
    pub name: String,
    pub fatty_acids: Vec<FattyAcid>,
}

impl FromStr for Parsed {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        from_str(value)
    }
}

/// Visitor
struct Visitor;

impl VisitMut for Visitor {
    fn visit_item_mut(&mut self, node: &mut Item) {
        let mut item = take(node);
        item = match item.into_table() {
            Ok(mut table) => {
                table.sort_values_by(|left, _, right, _| {
                    const KEYS: [&str; 3] = ["mag2", "dag1223", "tag123"];

                    let key = |target: &Key| KEYS.iter().position(|&source| source == target.get());
                    key(left).cmp(&key(right))
                });
                value(table.into_inline_table())
            }
            Err(item) => item,
        };
        item = match item.into_array_of_tables() {
            Ok(array_of_tables) => Item::ArrayOfTables(array_of_tables),
            Err(item) => item,
        };
        *node = item;
        visit_item_mut(self, node);
    }

    fn visit_inline_table_mut(&mut self, node: &mut InlineTable) {
        node.decor_mut().clear();

        visit_inline_table_mut(self, node);
    }

    fn visit_value_mut(&mut self, node: &mut Value) {
        node.decor_mut().clear();

        visit_value_mut(self, node);
    }
}

/// Result
type Result<T, E = Error> = std::result::Result<T, E>;

#[test]
fn test() -> Result<()> {
    // let config = include_str!("../../configs/pinaceae/cedrus/config.toml");
    let config = include_str!("../../configs/test/temp.toml");
    let parsed = config.parse::<Parsed>()?;
    println!("{parsed:?}");
    Ok(())
}
