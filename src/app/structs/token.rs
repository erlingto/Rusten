#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    DiagramStart,
    ClassStart,
    AttributesStart,
    AttributesEnd,
    AttributeStart,
    AttributeEnd,
    ConnectionArrow,
    Text,
    NewLine,
    End,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tokenType: TokenType,
    pub value: String,
}
