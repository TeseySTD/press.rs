use crate::compressor::{MAX_CODE_WIDTH, MAX_ENTRY_COUNT};

use super::INITIAL_CODE_WIDTH;

struct BitWriter {
    buffer: u32,
    cursor: u8,
    output: Vec<u8>,
}

impl BitWriter {
    fn new() -> Self {
        Self {
            buffer: 0,
            cursor: 0,
            output: Vec::new(),
        }
    }
    fn write(&mut self, code: u16, width: u8) {
        let mask = (1 << width) - 1;
        self.buffer |= (code as u32 & mask) << self.cursor;
        self.cursor += width;

        while self.cursor >= 8 {
            let byte = self.buffer as u8;
            self.buffer >>= 8;
            self.cursor -= 8;

            self.output.push(byte);
        }
    }
    fn flush(&mut self) {
        if self.cursor > 0 {
            self.output.push(self.buffer as u8);
            self.cursor = 0;
            self.buffer = 0;
        }
    }
}

#[derive(Debug, Clone)]
enum PrefixTreeNode {
    NoChild,
    Leaf { child_char: u8, child_index: u16 },
    HasNodes { child_indices: Vec<u16> },
}

struct PrefixTree {
    nodes: Vec<PrefixTreeNode>,
    code_size: u8,
    code_count: usize,
}

impl PrefixTree {
    fn new(code_size: u8) -> Self {
        let mut nodes = Vec::with_capacity(MAX_ENTRY_COUNT);
        let code_count = 1 << code_size;
        nodes.resize(code_count + 2, PrefixTreeNode::NoChild);
        Self {
            nodes,
            code_size,
            code_count,
        }
    }

    #[inline(always)]
    fn reset(&mut self) {
        self.nodes.clear();
        self.nodes.resize(self.code_count + 2, PrefixTreeNode::NoChild)
    }

    #[inline(always)]
    fn find_word(&self, prefix_index: u16, next_char: u8) -> Option<u16> {
        let prefix = &self.nodes[prefix_index as usize];
        match prefix {
            &PrefixTreeNode::NoChild => None,
            &PrefixTreeNode::Leaf {
                child_char,
                child_index,
            } => {
                if child_char == next_char {
                    Some(child_index)
                } else {
                    None
                }
            }
            PrefixTreeNode::HasNodes { child_indices } => {
                let child_index = child_indices[next_char as usize];
                if child_index > 0 {
                    Some(child_index)
                } else {
                    None
                }
            }
        }
    }

    /// Add a word to the tree and returns index of new node
    #[inline(always)]
    fn add(&mut self, prefix_index: u16, k: u8) -> u16 {
        let new_index = self.nodes.len() as u16;
        let prefix_index = prefix_index as usize;

        let mut old_node = &mut self.nodes[prefix_index];

        match &mut old_node {
            PrefixTreeNode::NoChild => {
                self.nodes[prefix_index] = PrefixTreeNode::Leaf {
                    child_char: k,
                    child_index: new_index,
                };
            }
            PrefixTreeNode::Leaf {
                child_char: char,
                child_index: index,
            } => {
                let mut children = vec![0; self.code_count];
                children[*char as usize] = *index;
                children[k as usize] = new_index;
                self.nodes[prefix_index] = PrefixTreeNode::HasNodes {
                    child_indices: children,
                };
            }
            PrefixTreeNode::HasNodes {
                child_indices: children,
            } => {
                children[k as usize] = new_index;
            }
        };
        self.nodes.push(PrefixTreeNode::NoChild);
        return new_index;
    }
}

pub fn lzw_compress(data: &[u8]) -> Vec<u8> {
    let mut tree = PrefixTree::new(INITIAL_CODE_WIDTH);
    let mut writer = BitWriter::new();

    let mut bytes = data.iter();
    let k = bytes.next();
    if k.is_none() {
        return vec![];
    }

    let mut write_size = INITIAL_CODE_WIDTH + 1;
    let clear_code = 1 << INITIAL_CODE_WIDTH;
    let end_of_information = (1 << INITIAL_CODE_WIDTH) + 1;
    let mut size_increase_mask = 1 << write_size;

    writer.write(clear_code, write_size);

    let mut prefix_index = *k.unwrap() as u16;

    for byte in bytes {
        if let Some(child_index) = tree.find_word(prefix_index, *byte) {
            prefix_index = child_index;
        } else {
            let index_of_new_entry = tree.add(prefix_index, *byte);
            writer.write(prefix_index, write_size);
            prefix_index = *byte as u16;
            
            if index_of_new_entry == size_increase_mask {
                if write_size < MAX_CODE_WIDTH {
                    write_size += 1;
                } else {
                    writer.write(clear_code, MAX_CODE_WIDTH);
                    write_size = INITIAL_CODE_WIDTH + 1;
                    tree.reset();
                }
                size_increase_mask = 1 << write_size;
            }
        }
    }
    writer.write(prefix_index, write_size);
    writer.write(end_of_information, write_size);

    writer.flush();
    return writer.output;
}
