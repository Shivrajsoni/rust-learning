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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading() {
        let input = "## Heading 2";
        let expected = vec![Token::Heading(2), Token::Text("Heading 2".to_string())];
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_bold() {
        let input = "**bold text**";
        let expected = vec![
            Token::BoldStart,
            Token::Text("bold text".to_string()),
            Token::BoldEnd,
        ];
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_italic() {
        let input = "*italic text*";
        let expected = vec![
            Token::ItalicStart,
            Token::Text("italic text".to_string()),
            Token::ItalicEnd,
        ];
        assert_eq!(lex(input), expected);
    }

    #[test]
    fn test_mixed_and_multiline() {
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
    fn test_no_space_after_heading() {
        let input = "#Heading";
        let expected = vec![Token::Heading(1), Token::Text("Heading".to_string())];
        assert_eq!(lex(input), expected);
    }
}

