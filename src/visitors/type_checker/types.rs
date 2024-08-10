#[derive(Clone, PartialEq, Debug)]
pub enum Type {
    Number,
    Boolean,
    String,
    Function,
    Unknown,
    Nil,
    List,
}
