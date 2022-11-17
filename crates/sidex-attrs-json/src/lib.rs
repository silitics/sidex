pub enum JsonType {
    Number,
    Boolean,
    String,
    Null,
    Object,
    Array,
}

pub struct JsonUnionType {
    pub types: Vec<JsonType>,
}
