#[derive(Debug, Clone)]
enum BlockType {
    Document,
    Paragraph,
    Heading(usize),
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

    pub fn create_paragraph(line: &str) -> Block {
        Block::create(BlockType::Paragraph, Some(line.trim().to_string()))
    }

    pub fn create_heading(line: &str) -> Block {
        let lvl = line.split(' ').next().unwrap().len().clamp(1, 6);
        Block::create(
            BlockType::Heading(lvl),
            Some(line.trim_matches('#').trim().to_string()),
        )
    }

    pub fn create_list_item(line: &str) -> Block {
        Block::create(
            BlockType::ListItem,
            Some(line.trim_matches(['-', '*']).trim().to_string()),
        )
    }

    pub fn split_block(content: String) -> Block {
        let mut document = Block::create(BlockType::Document, None);
        for line in content.lines() {
            // all of those ifs could be replaced by a single
            // match statement. just need to figure out how..
            if line.trim().is_empty() {
                continue;
            }
            if line.trim().starts_with("#") {
                document.append_block(Block::create_heading(line));
                continue;
            }
            if line.trim().starts_with("- ") {
                document.append_block(Block::create_list_item(line));
                continue;
            }
            document.append_block(Block::create_paragraph(line));
        }
        document
    }
}
