use molecule::Counter;
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

/// Fatty acid
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FattyAcid {
    pub label: String,
    #[serde(with = "formula")]
    pub formula: Counter,
    pub data: Data,
}

impl FattyAcid {
    pub fn new(label: String, formula: Counter, tag123: f64, dag1223: f64, mag2: f64) -> Self {
        Self {
            label,
            formula,
            data: Data {
                tag123,
                dag1223,
                mag2,
            },
        }
    }
}

/// Data
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Data {
    pub tag123: f64,
    pub dag1223: f64,
    pub mag2: f64,
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

mod formula {
    use molecule::Counter;
    use serde::{de::Error, Deserialize, Deserializer, Serializer};

    pub(super) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Counter, D::Error> {
        Ok(String::deserialize(deserializer)?
            .parse()
            .map_err(Error::custom)?)
    }

    pub(super) fn serialize<S: Serializer>(
        counter: &Counter,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&counter.to_string())
    }
}

#[test]
fn test() -> Result<()> {
    // let config = include_str!("../../configs/pinaceae/cedrus/config.toml");
    let config = include_str!("../../configs/test/temp.toml");
    let parsed = config.parse::<Parsed>()?;
    println!("{parsed:?}");
    Ok(())
}
