/// This is the `Tokenizer` struct.
/// It owns a `Vec<char>` and iterates over the `char`acters to tokenize the input text.
/// It also keeps track of the current position in the document and offset from the beginning of a line.
///
/// # Example
/// ```
/// // to iterate over tokens in a String you :
/// // - create an instance of the Tokenizer struct
/// // - then iterate over it using a for loop.
/// let tk = Tokenizer::new(String::from("Hello, world!"));
/// for token in tk {
///     println!("{}", token);
/// }
/// ```
pub struct Tokenizer {
    text: Vec<char>,
    position: usize,
    offset: usize,
}

/// This is the `Token` struct.
/// It owns a `String` and an `usize` representing the column offset.
/// It is not meant to be used on its own.
pub struct Token {
    content: String,
    col_offset: usize,
}

pub fn test_tokenizer(tokenizer: &mut Tokenizer) {
    for token in tokenizer {
        if token.content.chars().next().unwrap() == '\n' {
            println!("(\\n) ");
            continue;
        }
        print!("{} ", token);
    }
}

impl Tokenizer {
    /// Creates a new `Tokenizer` instance.
    /// It takes a `String` and breaks it down into a 'Vec<char>' for easier processing.
    pub fn new(text: String) -> Self {
        Tokenizer {
            text: text.chars().collect(),
            position: 0,
            offset: 0,
        }
    }
}

impl Token {
    /// Creates a new `Token` instance.
    /// It takes a `String` and an `usize` representing the column offset.
    pub fn new(content: String, col_offset: usize) -> Self {
        Token {
            content,
            col_offset,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[\"{}\" | {}]", self.content, self.col_offset)
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token {{ content: {:?}, col_offset: {} }}",
            self.content, self.col_offset
        )
    }
}

fn is_special_char(c: char) -> bool {
    matches!(c, '*' | '[' | ']' | '(' | ')' | '{' | '}')
}

fn is_word(c: char) -> bool {
    !c.is_whitespace() && !is_special_char(c)
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while self.position < self.text.len() && self.text[self.position].is_whitespace() {
            if self.text[self.position] == '\n' {
                self.position += 1;
                self.offset = 0;
                return Some(Token::new(String::from("\n"), self.offset));
            }
            self.position += 1;
            self.offset += 1;
        }

        if self.position >= self.text.len() {
            return None;
        }

        match self.text[self.position] {
            '*' | '[' | ']' | '(' | ')' => {
                let (pos, line_pos) = (self.position, self.offset);
                self.position += 1;
                self.offset += 1;
                return Some(Token::new(self.text[pos].to_string(), line_pos));
            }
            _ => {
                let (start, line_pos) = (self.position, self.offset);
                while self.position < self.text.len() && is_word(self.text[self.position]) {
                    self.position += 1;
                    self.offset += 1;
                }
                return Some(Token::new(
                    self.text[start..self.position].into_iter().collect(),
                    line_pos,
                ));
            }
        }
    }
}
