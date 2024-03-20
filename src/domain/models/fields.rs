use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub id: Uuid,
    pub name: String,
    pub value: String,
}
