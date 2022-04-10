use std::collections::BTreeMap;

use crate::model::{PropertyType, ResourceType};

pub(crate) trait Generatable {
    fn version(&self) -> &str;
    fn property_types(&self) -> &Option<BTreeMap<String, PropertyType>>;
    fn resource_types(&self) -> &BTreeMap<String, ResourceType>;
}
