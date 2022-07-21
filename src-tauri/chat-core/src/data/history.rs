use super::object::Object;

pub type TimeToken = u64;

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub message: Object,
    pub timetoken: TimeToken,
    pub metadata: Object,
}
