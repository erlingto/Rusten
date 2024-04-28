use leptos::{create_rw_signal, RwSignal, SignalGet};
use leptos_use::core::Position;
use log::debug;

use crate::app::structs::{
    connectionItem::ConnectionItem,
    moveBoxItem::MoveBoxItem,
    token::{Token, TokenType},
    MoveBoxAttribute::MoveBoxAttribute,
};

pub fn parseToken(word: String) -> Token {
    match word.as_str() {
        "classDiagram" => {
            return Token {
                tokenType: TokenType::DiagramStart,
                value: word,
            }
        }
        ":::mermaid" => {
            return Token {
                tokenType: TokenType::DiagramStart,
                value: word,
            }
        }
        "class" => {
            return Token {
                tokenType: TokenType::ClassStart,
                value: word,
            }
        }
        "+" => {
            return Token {
                tokenType: TokenType::AttributeStart,
                value: word,
            }
        }
        "{" => {
            return Token {
                tokenType: TokenType::AttributesStart,
                value: word,
            }
        }
        "}" => {
            return Token {
                tokenType: TokenType::AttributesEnd,
                value: word,
            }
        }
        "-->" => {
            return Token {
                tokenType: TokenType::ConnectionArrow,
                value: word,
            }
        }
        "\n" => {
            return Token {
                tokenType: TokenType::NewLine,
                value: word,
            }
        }
        _ => {
            return Token {
                tokenType: TokenType::Text,
                value: word,
            }
        }
    }
}

pub fn splitText(line: String) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut current_word = String::new();
    for c in line.chars() {
        if c == '\n' {
            if (!current_word.is_empty()) {
                words.push(current_word.clone());
                current_word.clear();
            }
            words.push(c.to_string());
        } else if (!c.is_whitespace()) {
            current_word.push(c);
        } else {
            if !current_word.is_empty() {
                words.push(current_word.clone());
                current_word.clear();
            }
        }
    }
    if !current_word.is_empty() {
        words.push(current_word);
    }
    words
}

pub fn parseText(line: String) -> Vec<Token> {
    let mut tokens = vec![];
    let mut split: Vec<String> = splitText(line);
    split.reverse();
    let mut word = split.pop();
    while word.is_some() {
        let token = parseToken(word.unwrap());
        tokens.push(token);
        word = split.pop();
    }
    tokens
}

pub fn createState(
    tokens: Vec<Token>,
    counter: i32,
) -> (Vec<RwSignal<MoveBoxItem>>, Vec<RwSignal<ConnectionItem>>) {
    let mut tokens = tokens;
    tokens.reverse();
    let mut items: Vec<RwSignal<MoveBoxItem>> = vec![];
    let mut connections: Vec<RwSignal<ConnectionItem>> = vec![];
    while (tokens.len() > 0) {
        let tokenO = tokens.pop();
        if (tokenO.is_some()) {
            let token = tokenO.clone().unwrap();
            if (token.tokenType == TokenType::DiagramStart) {
                continue;
            }
            if token.tokenType == TokenType::ClassStart {
                HandleClass(&token, &mut tokens, &mut items, counter);
            } else if token.tokenType == TokenType::Text {
                HandleConnection(&mut tokens, tokenO, &items, &mut connections, counter);
            }
        }
    }
    (items, connections)
}

fn FilterNewLines(tokens: &mut Vec<Token>) {
    let mut peekToken = tokens[tokens.len() - 1].clone();
    while (peekToken.tokenType == TokenType::NewLine) {
        tokens.pop();
        peekToken = tokens[tokens.len() - 1].clone();
    }
}

fn SearchName(tokens: &mut Vec<Token>) -> String {
    let mut name = String::from("");
    FilterNewLines(tokens);
    let nameToken = tokens.pop().unwrap();
    if (nameToken.tokenType == TokenType::Text) {
        name = nameToken
            .value
            .chars()
            .filter(|x| x.is_alphanumeric())
            .collect();
    }
    name
}

fn HandleClass(
    token: &Token,
    tokens: &mut Vec<Token>,
    items: &mut Vec<RwSignal<MoveBoxItem>>,
    counter: i32,
) {
    let mut attributes = vec![];
    let name = SearchName(tokens);
    FilterNewLines(tokens);
    let peekToken = tokens[tokens.len() - 1].clone();
    if peekToken.tokenType == TokenType::AttributesStart {
        let attributeToken = tokens.pop();
        attributes = HandleAttributes(&attributeToken, tokens);
    }

    let item = create_rw_signal(MoveBoxItem {
        position: create_rw_signal(Position { x: 0.0, y: 0.0 }),
        realPosition: create_rw_signal(Position { x: 0.0, y: 0.0 }),
        size: create_rw_signal(Position { x: 20.0, y: 20.0 }),
        isDragging: create_rw_signal(false),
        key: format!("{}:{}", counter, items.len().to_string()),
        value: create_rw_signal(name),
        attributes: create_rw_signal(attributes.clone()),
        should_render: create_rw_signal(true),
    });
    items.push(item);
}

fn HandleAttributes(token: &Option<Token>, tokens: &mut Vec<Token>) -> Vec<MoveBoxAttribute> {
    let mut attributes = vec![];
    let mut attributeValue = String::from("");
    let mut attTokenO = token.clone();
    while (attTokenO.clone().is_some()
        && attTokenO.clone().unwrap().tokenType != TokenType::AttributesEnd)
    {
        let mut textTokenO = attTokenO.clone();
        while (textTokenO.is_some() && textTokenO.clone().unwrap().tokenType != TokenType::NewLine)
        {
            let textToken = textTokenO.clone().unwrap();
            if (textToken.tokenType == TokenType::Text) {
                attributeValue = format!("{} {}", attributeValue, textToken.value);
            }
            textTokenO = tokens.pop();
        }
        if (!attributeValue.is_empty()) {
            attributes.push(MoveBoxAttribute {
                key: attributes.len().to_string(),
                value: create_rw_signal(attributeValue.clone()),
            });
            attributeValue = String::from("");
        }
        attTokenO = tokens.pop();
    }
    attributes
}

fn HandleConnection(
    tokens: &mut Vec<Token>,
    token: Option<Token>,
    items: &Vec<RwSignal<MoveBoxItem>>,
    connections: &mut Vec<RwSignal<ConnectionItem>>,
    counter: i32,
) {
    let arrowToken = tokens.pop();
    if (arrowToken.is_none()) {
        return;
    }
    assert!(arrowToken.unwrap().tokenType == TokenType::ConnectionArrow);
    let from = token.unwrap();
    let to = tokens.pop().unwrap();
    let fromName: String = from.value.chars().filter(|x| x.is_alphanumeric()).collect();
    let toName: String = to.value.chars().filter(|x| x.is_alphanumeric()).collect();
    assert!(to.tokenType == TokenType::Text);
    let fromItem = items.iter().find(|x| x.get().value.get() == fromName);
    let toItem = items.iter().find(|x| x.get().value.get() == toName);
    if (fromItem.is_some() && toItem.is_some()) {
        let connection = create_rw_signal(ConnectionItem {
            key: format!("{}:{}", counter, connections.len().to_string()),
            from: *fromItem.unwrap(),
            to: *toItem.unwrap(),
        });
        connections.push(connection);
    }
}

pub fn importDiagram(
    text: String,
    counter: i32,
) -> (Vec<RwSignal<MoveBoxItem>>, Vec<RwSignal<ConnectionItem>>) {
    let tokens = parseText(text);
    let (items, connections) = createState(tokens, counter);
    (items, connections)
}
