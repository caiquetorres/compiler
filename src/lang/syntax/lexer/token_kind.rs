use std::fmt::{Debug, Display};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum TokenKind {
    Identifier,
    NumberLiteral,
    BooleanLiteral,
    CharLiteral,
    StringLiteral,
    LeftBrace,
    RightBrace,
    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,
    Plus,
    PlusEquals,
    Minus,
    MinusEquals,
    Slash,
    SlashEquals,
    Star,
    StarEquals,
    Mod,
    ModEquals,
    WhiteSpace,
    EndOfFile,
    Semicolon,
    Colon,
    Comma,
    LetKeyword,
    ConstKeyword,
    FunKeyword,
    IfKeyword,
    ElseKeyword,
    WhileKeyword,
    DoKeyword,
    ForKeyword,
    InKeyword,
    ReturnKeyword,
    BadToken,
    Equals,
    EqualsEquals,
    Pipe,
    PipePipe,
    PipeEquals,
    Ampersand,
    AmpersandAmpersand,
    AmpersandEquals,
    LessThan,
    LessThanLessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanGreaterThan,
    GreaterThanEquals,
    Exclamation,
    ExclamationEquals,
    Circumflex,
    CircumflexEquals,
    Tilde,
    TildeEquals,
    Dot,
    DotDot,
    DotDotEquals,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", convert_to_readable_string(self))
    }
}

fn convert_to_readable_string(kind: &TokenKind) -> String {
    let kind_str = format!("{:?}", kind);

    let mut readable_str = String::new();
    let mut last_was_upper = false;

    for c in kind_str.chars() {
        if c.is_ascii_uppercase() {
            if last_was_upper {
                readable_str.push(c);
            } else {
                readable_str.push(' ');
                readable_str.push(c.to_ascii_lowercase());
            }
            last_was_upper = true;
        } else {
            readable_str.push(c);
            last_was_upper = false;
        }
    }

    readable_str.trim().to_lowercase()
}
