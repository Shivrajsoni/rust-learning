use std::fmt::format;
pub mod test;
#[derive(Debug, PartialEq)]
enum Token {
    Heading(usize),
    BoldStart,
    BoldEnd,
    ItalicStart,
    ItalicEnd,
    Text(String),
    NewLine,
    Link { text: String, url: String },
}

#[derive(Debug, PartialEq)]
enum Node {
    Document(Vec<Node>),
    Heading(usize, Vec<Node>),
    Paragraph(Vec<Node>),
    Bold(Vec<Node>),
    Italic(Vec<Node>),
    Text(String),
    Link { text: String, url: String },
}

fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut bold_active = false;
    let mut italic_active = false;
    while let Some(c) = chars.next() {
        match c {
            '#' => {
                let mut level = 1;
                while let Some('#') = chars.peek() {
                    chars.next();
                    level += 1;
                }
                // Headings are typically followed by a space
                if chars.peek() == Some(&' ') {
                    chars.next();
                }
                tokens.push(Token::Heading(level));
            }
            '*' => {
                if chars.peek() == Some(&'*') {
                    chars.next(); // consume the second '*'
                    if bold_active {
                        tokens.push(Token::BoldEnd);
                    } else {
                        tokens.push(Token::BoldStart);
                    }
                    bold_active = !bold_active;
                } else {
                    if italic_active {
                        tokens.push(Token::ItalicEnd);
                    } else {
                        tokens.push(Token::ItalicStart);
                    }
                    italic_active = !italic_active;
                }
            }
            '\n' => {
                tokens.push(Token::NewLine);
            }
            '[' => {
                let mut text = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == ']' {
                        break;
                    }
                    text.push(chars.next().unwrap());
                }

                // Check for the full link syntax: [text](url)
                if chars.peek() == Some(&']') {
                    chars.next(); // consume ']'
                    if chars.peek() == Some(&'(') {
                        chars.next(); // consume '('
                        let mut url = String::new();
                        while let Some(&ch) = chars.peek() {
                            if ch == ')' {
                                break;
                            }
                            url.push(chars.next().unwrap());
                        }
                        if chars.peek() == Some(&')') {
                            chars.next(); // consume ')'
                            tokens.push(Token::Link { text, url });
                        } else {
                            // This is a malformed link, like [text](url
                            // Treat all parts as plain text.
                            tokens.push(Token::Text("[".to_string()));
                            tokens.push(Token::Text(text));
                            tokens.push(Token::Text("]".to_string()));
                            tokens.push(Token::Text("(".to_string()));
                            tokens.push(Token::Text(url));
                        }
                    } else {
                        // This is just text in brackets, like [text]
                        tokens.push(Token::Text("[".to_string()));
                        tokens.push(Token::Text(text));
                        tokens.push(Token::Text("]".to_string()));
                    }
                } else {
                    // No closing bracket found, like [text
                    tokens.push(Token::Text("[".to_string()));
                    tokens.push(Token::Text(text));
                }
            }

            _ => {
                let mut buff = String::new();
                buff.push(c);
                while let Some(&next) = chars.peek() {
                    if next == '#' || next == '*' || next == '\n' || next == '[' {
                        break;
                    }
                    buff.push(chars.next().unwrap());
                }
                tokens.push(Token::Text(buff));
            }
        }
    }
    tokens
}

fn parse(tokens: &[Token]) -> Node {
    let mut nodes = Vec::new();
    // We split the tokens by NewLine to get logical "lines" or "blocks".
    // This is a simpler way to group tokens for paragraphs or headings.
    for line_tokens in tokens.split(|tok| *tok == Token::NewLine) {
        if line_tokens.is_empty() {
            continue;
        }
        match &line_tokens[0] {
            Token::Heading(level) => {
                // The rest of the tokens on the line are the heading's content.
                let content = parse_inlines(&line_tokens[1..]);
                nodes.push(Node::Heading(*level, content));
            }
            // Anything else that is not a heading, we'll treat as a paragraph.
            _ => {
                let content = parse_inlines(line_tokens);
                nodes.push(Node::Paragraph(content));
            }
        }
    }
    Node::Document(nodes)
}

// This is our powerful helper function to handle text styles.
// It can even handle nesting, like **bold *and* italic**.
fn parse_inlines(tokens: &[Token]) -> Vec<Node> {
    let mut nodes = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            Token::Text(text) => {
                nodes.push(Node::Text(text.clone()));
                i += 1;
            }
            Token::BoldStart => {
                i += 1; // Consume BoldStart
                // Find the matching BoldEnd
                let end_pos = tokens[i..]
                    .iter()
                    .position(|t| matches!(t, Token::BoldEnd))
                    .map_or(tokens.len(), |pos| i + pos);

                // Recursively parse the content inside the bold tags
                let inner_nodes = parse_inlines(&tokens[i..end_pos]);
                nodes.push(Node::Bold(inner_nodes));

                i = end_pos;
                if i < tokens.len() {
                    i += 1; // Consume BoldEnd
                }
            }
            Token::ItalicStart => {
                i += 1; // Consume ItalicStart
                // Find the matching ItalicEnd
                let end_pos = tokens[i..]
                    .iter()
                    .position(|t| matches!(t, Token::ItalicEnd))
                    .map_or(tokens.len(), |pos| i + pos);

                // Recursively parse the content inside the italic tags
                let inner_nodes = parse_inlines(&tokens[i..end_pos]);
                nodes.push(Node::Italic(inner_nodes));

                i = end_pos;
                if i < tokens.len() {
                    i += 1; // Consume ItalicEnd
                }
            }
            Token::Link { text, url } => {
                nodes.push(Node::Link {
                    text: text.clone(),
                    url: url.clone(),
                });
                i += 1;
            }
            // We shouldn't encounter these here if our block parsing is correct, but we'll skip them.
            Token::Heading(_) | Token::NewLine | Token::BoldEnd | Token::ItalicEnd => {
                i += 1;
            }
        }
    }
    nodes
}

fn render(node: &Node) -> String {
    match node {
        Node::Document(children) => children
            .iter()
            .map(render)
            .collect::<Vec<String>>()
            .join("\n"),
        Node::Heading(level, children) => {
            format!("<h{}>{}</h{}>", level, render_all(children), level)
        }
        Node::Paragraph(children) => {
            format!("<p>{}</p>", render_all(children))
        }
        Node::Bold(children) => {
            format!("<strong>{}</strong>", render_all(children))
        }
        Node::Italic(children) => {
            format!("<em>{}</em>", render_all(children))
        }
        Node::Text(text) => text.clone(),
        Node::Link { text, url } => {
            format!("<a href=\"{}\">{}</a>", url, text)
        }
    }
}

fn render_all(nodes: &[Node]) -> String {
    nodes.iter().map(render).collect()
}
