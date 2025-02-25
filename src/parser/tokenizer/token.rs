/// This is the `Token` struct.
/// It owns a `String` and two `usize` representing the column offset and line number.
/// It is not meant to be used on its own.
pub struct Token {
    content: String,
    col: usize,
    line: usize,
}

impl Token {
    /// Creates a new `Token` instance.
    /// It takes a `String`, an `usize` representing the column offset, and an `usize` representing the line number.
    pub fn new(content: String, col: usize, line: usize) -> Self {
        Token { content, col, line }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[\"{}\" | (l:{}, c:{})]",
            self.content, self.line, self.col
        )
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token {{ content: {:?}, line: {}, col: {} }}",
            self.content, self.line, self.col
        )
    }
}
