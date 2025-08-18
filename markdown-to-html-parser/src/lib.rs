use std::fmt::format;

#[derive(Debug, PartialEq)]
enum Token {
    Heading(usize),
    BoldStart,
    BoldEnd,
    ItalicStart,
    ItalicEnd,
    Text(String),
    NewLine,
}

#[derive(Debug, PartialEq)]
enum Node {
    Document(Vec<Node>),
    Heading(usize, Vec<Node>),
    Paragraph(Vec<Node>),
    Bold(Vec<Node>),
    Italic(Vec<Node>),
    Text(String),
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
            _ => {
                let mut buff = String::new();
                buff.push(c);
                while let Some(&next) = chars.peek() {
                    if next == '#' || next == '*' || next == '\n' {
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
        Node::Document(children) => children.iter().map(render).collect::<Vec<String>>().join(
            "
",
        ),
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
    }
}

fn render_all(nodes: &[Node]) -> String {
    nodes.iter().map(render).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Lexer tests
    #[test]
    fn test_lex_heading() {
        let input = "## Heading 2";
        let expected = vec![Token::Heading(2), Token::Text("Heading 2".to_string())];
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_lex_bold() {
        let input = "**bold text**";
        let expected = vec![
            Token::BoldStart,
            Token::Text("bold text".to_string()),
            Token::BoldEnd,
        ];
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_lex_italic() {
        let input = "*italic text*";
        let expected = vec![
            Token::ItalicStart,
            Token::Text("italic text".to_string()),
            Token::ItalicEnd,
        ];
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_lex_mixed_and_multiline() {
        let input = "### Header
Hello **world** in *Rust*!";
        let expected = vec![
            Token::Heading(3),
            Token::Text("Header".to_string()),
            Token::NewLine,
            Token::Text("Hello ".to_string()),
            Token::BoldStart,
            Token::Text("world".to_string()),
            Token::BoldEnd,
            Token::Text(" in ".to_string()),
            Token::ItalicStart,
            Token::Text("Rust".to_string()),
            Token::ItalicEnd,
            Token::Text("!".to_string()),
        ];
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_lex_no_space_after_heading() {
        let input = "#Heading";
        let expected = vec![Token::Heading(1), Token::Text("Heading".to_string())];
        assert_eq!(lex(input), expected);
    }

    // Parser tests
    #[test]
    fn test_parse_heading() {
        let tokens = vec![Token::Heading(1), Token::Text("Hello".to_string())];
        let expected = Node::Document(vec![Node::Heading(
            1,
            vec![Node::Text("Hello".to_string())],
        )]);
        assert_eq!(parse(&tokens), expected);
    }

    #[test]
    fn test_parse_paragraph() {
        let tokens = vec![
            Token::Text("This is a ".to_string()),
            Token::BoldStart,
            Token::Text("test".to_string()),
            Token::BoldEnd,
            Token::Text(".".to_string()),
        ];
        let expected = Node::Document(vec![Node::Paragraph(vec![
            Node::Text("This is a ".to_string()),
            Node::Bold(vec![Node::Text("test".to_string())]),
            Node::Text(".".to_string()),
        ])]);
        assert_eq!(parse(&tokens), expected);
    }

    #[test]
    fn test_parse_multiline() {
        let tokens = vec![
            Token::Heading(2),
            Token::Text("Title".to_string()),
            Token::NewLine,
            Token::Text("Some text.".to_string()),
        ];
        let expected = Node::Document(vec![
            Node::Heading(2, vec![Node::Text("Title".to_string())]),
            Node::Paragraph(vec![Node::Text("Some text.".to_string())]),
        ]);
        assert_eq!(parse(&tokens), expected);
    }

    #[test]
    fn test_parse_nested_styles() {
        let tokens = vec![
            Token::BoldStart,
            Token::Text("bold and ".to_string()),
            Token::ItalicStart,
            Token::Text("italic".to_string()),
            Token::ItalicEnd,
            Token::BoldEnd,
        ];
        let expected = Node::Document(vec![Node::Paragraph(vec![Node::Bold(vec![
            Node::Text("bold and ".to_string()),
            Node::Italic(vec![Node::Text("italic".to_string())]),
        ])])]);
        assert_eq!(parse(&tokens), expected);
    }

    // Render tests
    #[test]
    fn test_render_heading() {
        let node = Node::Heading(1, vec![Node::Text("Test".to_string())]);
        assert_eq!(render(&node), "<h1>Test</h1>");
    }

    #[test]
    fn test_render_paragraph() {
        let node = Node::Paragraph(vec![
            Node::Text("This is ".to_string()),
            Node::Bold(vec![Node::Text("bold".to_string())]),
            Node::Text(".".to_string()),
        ]);
        assert_eq!(render(&node), "<p>This is <strong>bold</strong>.</p>");
    }

    #[test]
    fn test_render_document() {
        let node = Node::Document(vec![
            Node::Heading(1, vec![Node::Text("Title".to_string())]),
            Node::Paragraph(vec![Node::Text("Content.".to_string())]),
        ]);
        assert_eq!(render(&node), "<h1>Title</h1>\n<p>Content.</p>");
    }
}
