pub enum Node {
    Document(Vec<Node>),
    Heading { level: u8, content: String },
    Paragraph(Vec<Inline>),
}
