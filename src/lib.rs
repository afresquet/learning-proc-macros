use std::collections::HashMap;

pub use metadata_derive::MetaData;
pub use reflective_derive::Reflective;

pub trait Reflective {
    fn name(&self) -> &'static str;
    fn field_names(&self) -> Vec<&'static str>;
}

pub trait MetaData {
    fn author(&self) -> &'static str;
    fn serial_version(&self) -> usize;
    fn fields_authors(&self) -> HashMap<&'static str, &'static str>;
}
