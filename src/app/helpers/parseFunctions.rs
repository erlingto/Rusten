use leptos::RwSignal;

use crate::app::structs::{
    connectionItem::ConnectionItem,
    moveBoxItem::MoveBoxItem,
    token::{Token, TokenType},
    MoveBoxAttribute::MoveBoxAttribute,
};

pub fn parseBasicWord(word: String) -> Token {
    match word.as_str() {
        "classDiagram" => {
            return Token {
                tokenType: Token::ClassDiagram,
                value: word,
            }
        }
        "class" => {
            return Token {
                tokenType: Token::ClassStart,
                value: word,
            }
        }
        "+" => {
            return Token {
                tokenType: Token::AttributeStart,
                value: word,
            }
        }
        "{" => {
            return Token {
                tokenType: Token::AttributesStart,
                value: word,
            }
        }
        "}" => {
            return Token {
                tokenType: Token::AttributesEnd,
                value: word,
            }
        }
        "-->" => {
            return Token {
                tokenType: Token::ConnectionArrow,
                value: word,
            }
        }
        _ => {
            return Token {
                tokenType: Token::Text,
                value: word,
            }
        }
    }
}

pub fn parseLine(line: String) -> Vec<Token> {
    let mut tokens = vec![];
    let mut split = line.split_whitespace();
    let mut word = split.next();
    while word.is_some() {
        let token = parseToken(word);
        tokens.push(token);
    }
    tokens
}

pub fn createState(
    tokens: Vec<Token>,
) -> (Vec<RwSignal<ConnectionItem>>, Vec<RwSignal<MoveBoxItem>>) {
    let mut items = vec![];
    let mut connections = vec![];
    while (tokens.len() > 0) {
        let token = tokens.pop();
        if (token.is_some()) {
            match token.unwrap().tokenType {
                TokenType::AttributesStart => {
                    let attributes = vec![];
                    let attributeValue = String::from("");
                    let attToken = token;
                    while (attToken.is_some() && attToken.unwrap().tokenType != TokenType::AttributesEnd) {
                        let attToken = attToken.unwrap();
                        if (attToken.tokenType == TokenType::Text) {
                            attributeValue = format!("{} {}", attributeValue, attToken.value);
                        } else {
                            attributes.push(create_rw_signal(MoveBoxAttribute {
                                key: attributes.len().to_string(),
                                value: create_rw_signal(attributeValue),
                            }));
                        }
                    }
                    if (attToken.is_some()) {
                        let attToken = attToken.unwrap();
                        while (attToken.tokenType == TokenType::Text) {
                                attributeValue = format!("{} {}", attributeValue, attToken.value);
                            } else {
                                attributes.push(create_rw_signal(MoveBoxAttribute {
                                    key: attributes.len().to_string(),
                                    value: create_rw_signal(attributeValue),
                                }));
                            }
                        }
                    }
                TokenType::DiagramStart => todo!(),
                TokenType::ClassStart => todo!(),
                TokenType::AttributesEnd => todo!(),
                TokenType::AttributeStart => todo!(),
                TokenType::AttributeEnd => todo!(),
                TokenType::ConnectionArrow => todo!(),
                TokenType::Text => todo!(),
                TokenType::NewLine => todo!(),
                TokenType::End => todo!(),
                }
                TokenType::Text => {
                    let arrowString = split.next();
                    let arrowToken = parseBasicWord(arrowString);
                    assert!(arrowToken.tokenType == Token::ConnectionArrow);

                    items.push(create_rw_signal(parseConnectionStart(conn)));
                };
                _ => {}
            }
        }
    }
    (connections, items)
}

pub fn parseDiagram(text: String) -> (Vec<Token>) {
    let mut tokens = vec![];
    let mut lines = text.lines();

    while let Some(line) = lines.next() {
        let lineTokens = parseLine(line.to_string());
        tokens.extend(lineTokens);
    }

    while line.is_some() {
        let l = line.unwrap();
        if l.contains("classDiagram") {
            line = lines.next();
            continue;
        }

        if l.contains("class") {
            let mut att = vec![];
            let name = String::from(l.split("`").collect::<Vec<&str>>()[1]);
            let mut attLine = lines.next();
            let mut keyCount = 0;
            while attLine.is_some() {
                let attL = attLine.unwrap();
                if attL.contains("}") {
                    break;
                }
                if attL.contains("+") {
                    att.push(MoveBoxAttribute {
                        value: create_rw_signal(attL.replace("+", "").trim().to_string()),
                        key: keyCount.to_string(),
                    });
                    keyCount += 1;
                }
                attLine = lines.next();
            }
            newItems.push(create_rw_signal(MoveBoxItem {
                position: create_rw_signal(Position { x: 0.0, y: 0.0 }),
                realPosition: create_rw_signal(Position { x: 0.0, y: 0.0 }),
                value: create_rw_signal(name),
                key: format!(
                    "{}:{}",
                    importCount.get().to_string(),
                    classCount.to_string()
                ),
                attributes: create_rw_signal(att),
                isDragging: create_rw_signal(false),
                size: create_rw_signal(Position { x: 20.0, y: 20.0 }),
            }));
            classCount += 1;
        } else if l.contains("-->") {
            let mut split = l.split("-->");
            let mut from = split.next().unwrap().trim().to_string();
            let mut to = split.next().unwrap().trim().to_string();

            from = String::from(from.split("`").collect::<Vec<&str>>()[1]);
            to = String::from(to.split("`").collect::<Vec<&str>>()[1]);

            let toItem = newItems.iter().find(|x| x.get().value.get() == to);
            let fromItem = newItems.iter().find(|x| x.get().value.get()) == from;
        };
    }
}
