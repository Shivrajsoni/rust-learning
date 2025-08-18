use crate::*;

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

#[test]
fn test_render_text() {
    let node = Node::Document(vec![Node::Text("Text Data".to_string())]);
    assert_eq!(render(&node), "Text Data");
}

#[test]
fn test_render_all() {
    let node = vec![Node::Document(vec![
        Node::Heading(1, vec![Node::Text("Title".to_string())]),
        Node::Paragraph(vec![
            Node::Text("This is ".to_string()),
            Node::Bold(vec![Node::Text("bold".to_string())]),
            Node::Text(".".to_string()),
        ]),
    ])];

    assert_eq!(
        render_all(&node),
        "<h1>Title</h1>\n<p>This is <strong>bold</strong>.</p>"
    );
}

#[test]
fn test_render_link() {
    let node = Node::Document(vec![Node::Link {
        text: "github".to_string(),
        url: "https://github.com/Shivrajsoni".to_string(),
    }]);
    assert_eq!(
        render(&node),
        "<a href=\"https://github.com/Shivrajsoni\">github</a>",
    );
}
