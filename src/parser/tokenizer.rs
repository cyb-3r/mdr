/// This is the `Tokenizer` struct.
/// It owns a `Vec<char>` and iterates over the `char`acters to tokenize the input text.
/// This one is optimized for the markdown syntax!
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
}

impl Tokenizer {
    /// Creates a new `Tokenizer` instance.
    /// It takes a `String` and breaks it down into a 'Vec<char>' for easier processing.
    pub fn new(text: String) -> Self {
        Tokenizer {
            text: text.chars().collect(),
            position: 0,
        }
    }
}

fn is_word(c: char) -> bool {
    !c.is_whitespace() && c != '*'
}

impl Iterator for Tokenizer {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        /* NOTE:
         * while in normal mode, the tokenizer should count whitespace characters
         * because 2 spaces define a new line based on the common mark specs.
         */

        while self.position < self.text.len() && self.text[self.position].is_whitespace() {
            self.position += 1;
        }

        if self.position >= self.text.len() {
            return None;
        }

        match self.text[self.position] {
            '*' => {
                let pos = self.position;
                self.position += 1;
                return Some(self.text[pos].to_string());
            }
            _ => {
                let start = self.position;
                while self.position < self.text.len() && is_word(self.text[self.position]) {
                    self.position += 1;
                }
                return Some(self.text[start..self.position].into_iter().collect());
            }
        }
    }
}
