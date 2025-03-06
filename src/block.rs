#[derive(Debug, Clone)]
enum BlockType {
    Document,
    Paragraph,
    Heading(u8),
    List(bool, u16),
    ListItem,
    Separator,
}

#[derive(Debug)]
pub struct Block {
    kind: BlockType,
    open: bool,
    content: Option<String>,
    childs: Vec<Block>,
}

impl Block {
    fn create(kind: BlockType, content: Option<String>) -> Block {
        Block {
            kind,
            open: true,
            content,
            childs: Vec::new(),
        }
    }

    fn append_block(&mut self, child: Block) {
        if !self.open {
            return;
        }
        self.childs.push(child);
    }
}

pub fn split_block(content: String) -> Block {
    let mut document = Block::create(BlockType::Document, None);
    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if line.starts_with("#") {
            document.append_block(Block::create(BlockType::Heading(1), Some(line.to_string())));
            continue;
        }
        document.append_block(Block::create(BlockType::Paragraph, Some(line.to_string())));
    }
    document
}
