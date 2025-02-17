#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PieceTableBuffers {
    Original,
    Add,
}

#[derive(Debug)]
struct PieceTableEntry {
    buffer: PieceTableBuffers,
    start_index: usize,
    length: usize,
}

pub struct PieceTable {
    original_buffer: String,
    add_buffer: String,
    piece_table: Vec<PieceTableEntry>,
    length: usize,
}

impl PieceTable {
    pub fn new(original_buffer: &str) -> Self {
        let og_entry = PieceTableEntry {
            buffer: PieceTableBuffers::Original,
            start_index: 0,
            length: original_buffer.len(),
        };
        PieceTable {
            original_buffer: original_buffer.to_string(),
            add_buffer: String::new(),
            piece_table: vec![og_entry],
            length: original_buffer.len(),
        }
    }

    pub fn index(&self, idx: usize) -> char {
        let entry_idx = self.find_entry(idx);

        let mut offset: usize = 0;
        if entry_idx > 0 {
            for i in 0..entry_idx {
                offset += self.piece_table[i].length;
            }
        }

        let n = self.piece_table[entry_idx].start_index + (idx - offset);
        match self.piece_table[entry_idx].buffer {
            PieceTableBuffers::Original => self.original_buffer.chars().nth(n).unwrap(),
            PieceTableBuffers::Add => self.add_buffer.chars().nth(n).unwrap(),
        }
    }

    pub fn insert(&mut self, idx: usize, c: char) {
        if idx == self.length {
            let end_idx = self.piece_table.len() - 1;
            if self.piece_table[end_idx].buffer == PieceTableBuffers::Add
                && self.piece_table[end_idx].start_index + self.piece_table[end_idx].length
                    == self.add_buffer.len()
            {
                self.piece_table[end_idx].length += 1;
            } else {
                let new_entry = PieceTableEntry {
                    buffer: PieceTableBuffers::Add,
                    start_index: self.add_buffer.len(),
                    length: 1,
                };
                self.piece_table.push(new_entry);
            }
        } else {
            let entry_idx = self.find_entry(idx);

            let mut offset: usize = 0;
            if entry_idx > 0 {
                for i in 0..entry_idx {
                    offset += self.piece_table[i].length;
                }
            }

            if entry_idx > 0
                && (idx - offset) == 0
                && self.piece_table[entry_idx - 1].buffer == PieceTableBuffers::Add
                && self.piece_table[entry_idx - 1].start_index
                    + self.piece_table[entry_idx - 1].length
                    == self.add_buffer.len()
            {
                // In this case we can extend the entry length
                self.piece_table[entry_idx - 1].length += 1;
            } else if idx - offset == 0 {
                let new_entry = PieceTableEntry {
                    buffer: PieceTableBuffers::Add,
                    start_index: self.add_buffer.len(),
                    length: 1,
                };
                self.piece_table.insert(entry_idx, new_entry);
            } else if (idx - offset) + 1 == self.piece_table[entry_idx].length {
                let new_entry = PieceTableEntry {
                    buffer: PieceTableBuffers::Add,
                    start_index: self.add_buffer.len(),
                    length: 1,
                };
                self.piece_table.insert(entry_idx + 1, new_entry);
            } else {
                let split_entry = PieceTableEntry {
                    buffer: self.piece_table[entry_idx].buffer,
                    start_index: self.piece_table[entry_idx].start_index,
                    length: idx - offset,
                };

                self.piece_table[entry_idx].start_index = idx - offset;
                self.piece_table[entry_idx].length -= split_entry.length;
                self.piece_table.insert(entry_idx, split_entry);

                let new_entry = PieceTableEntry {
                    buffer: PieceTableBuffers::Add,
                    start_index: self.add_buffer.len(),
                    length: 1,
                };
                self.piece_table.insert(entry_idx + 1, new_entry);
            }
        }
        self.add_buffer.push(c);
        self.length += 1;
    }

    pub fn delete(&mut self, idx: usize) {
        let entry_idx = self.find_entry(idx);

        let mut offset: usize = 0;
        if entry_idx > 0 {
            for i in 0..entry_idx {
                offset += self.piece_table[i].length;
            }
        }

        println!("{idx}, {offset}");
        if (idx - offset) == 0 {
            self.piece_table[entry_idx].start_index += 1;
            self.piece_table[entry_idx].length -= 1;
            if self.piece_table[entry_idx].length == 0 {
                self.piece_table.remove(entry_idx);
            }
        } else if (idx - offset) + 1 == self.piece_table[entry_idx].length {
            self.piece_table[entry_idx].length -= 1;
        } else {
            let new_entry = PieceTableEntry {
                buffer: self.piece_table[entry_idx].buffer,
                start_index: self.piece_table[entry_idx].start_index,
                length: idx - offset,
            };

            self.piece_table[entry_idx].start_index += (idx - offset) + 1;
            self.piece_table[entry_idx].length -= new_entry.length + 1;
            self.piece_table.insert(entry_idx, new_entry)
        }
        self.length -= 1;
    }

    fn find_entry(&self, idx: usize) -> usize {
        let mut entry_idx: usize = 0;
        let mut total: usize = 0;

        while total + self.piece_table[entry_idx].length <= idx {
            total += self.piece_table[entry_idx].length;
            entry_idx += 1;
        }

        entry_idx
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut table = PieceTable::new("ipsum sit amet");
        table.insert(0, 'L');
        table.insert(1, 'o');
        table.insert(2, 'r');
        table.insert(3, 'e');
        table.insert(4, 'm');
        table.insert(5, ' ');

        assert_eq!(table.piece_table.len(), 2);
        assert_eq!(table.piece_table[0].buffer, PieceTableBuffers::Add);
        assert_eq!(table.piece_table[0].start_index, 0);
        assert_eq!(table.piece_table[0].length, 6);
        assert_eq!(table.piece_table[1].buffer, PieceTableBuffers::Original);
        assert_eq!(table.piece_table[1].start_index, 0);
        assert_eq!(table.piece_table[1].length, 14);
        assert_eq!(
            (0..table.len()).map(|i| table.index(i)).collect::<String>(),
            "Lorem ipsum sit amet".to_string()
        );

        table.insert(11, ' ');
        table.insert(12, 'd');
        table.insert(13, 'o');
        table.insert(14, 'l');
        table.insert(15, 'o');
        table.insert(16, 'r');

        assert_eq!(table.piece_table.len(), 4);
        assert_eq!(table.piece_table[0].buffer, PieceTableBuffers::Add);
        assert_eq!(table.piece_table[0].start_index, 0);
        assert_eq!(table.piece_table[0].length, 6);
        assert_eq!(table.piece_table[1].buffer, PieceTableBuffers::Original);
        assert_eq!(table.piece_table[1].start_index, 0);
        assert_eq!(table.piece_table[1].length, 5);
        assert_eq!(table.piece_table[2].buffer, PieceTableBuffers::Add);
        assert_eq!(table.piece_table[2].start_index, 6);
        assert_eq!(table.piece_table[2].length, 6);
        assert_eq!(table.piece_table[3].buffer, PieceTableBuffers::Original);
        assert_eq!(table.piece_table[3].start_index, 5);
        assert_eq!(table.piece_table[3].length, 9);
        assert_eq!(
            (0..table.len()).map(|i| table.index(i)).collect::<String>(),
            "Lorem ipsum dolor sit amet".to_string()
        );

        table.insert(26, ',');
        table.insert(27, ' ');
        table.insert(28, 'c');
        table.insert(29, 'o');
        table.insert(30, 'n');
        table.insert(31, 's');
        table.insert(32, 'e');
        table.insert(33, 'c');
        table.insert(34, 't');
        table.insert(35, 'e');
        table.insert(36, 't');
        table.insert(37, 'u');
        table.insert(38, 'r');

        assert_eq!(table.piece_table.len(), 5);
        assert_eq!(table.piece_table[0].buffer, PieceTableBuffers::Add);
        assert_eq!(table.piece_table[0].start_index, 0);
        assert_eq!(table.piece_table[0].length, 6);
        assert_eq!(table.piece_table[1].buffer, PieceTableBuffers::Original);
        assert_eq!(table.piece_table[1].start_index, 0);
        assert_eq!(table.piece_table[1].length, 5);
        assert_eq!(table.piece_table[2].buffer, PieceTableBuffers::Add);
        assert_eq!(table.piece_table[2].start_index, 6);
        assert_eq!(table.piece_table[2].length, 6);
        assert_eq!(table.piece_table[3].buffer, PieceTableBuffers::Original);
        assert_eq!(table.piece_table[3].start_index, 5);
        assert_eq!(table.piece_table[3].length, 9);
        assert_eq!(table.piece_table[4].buffer, PieceTableBuffers::Add);
        assert_eq!(table.piece_table[4].start_index, 12);
        assert_eq!(table.piece_table[4].length, 13);
        assert_eq!(
            (0..table.len()).map(|i| table.index(i)).collect::<String>(),
            "Lorem ipsum dolor sit amet, consectetur".to_string()
        );

        table.delete(6);
        table.delete(6);
        table.delete(6);
        table.delete(6);
        table.delete(6);

        assert_eq!(table.piece_table.len(), 4);
        assert_eq!(table.piece_table[0].buffer, PieceTableBuffers::Add);
        assert_eq!(table.piece_table[0].start_index, 0);
        assert_eq!(table.piece_table[0].length, 6);
        assert_eq!(table.piece_table[1].buffer, PieceTableBuffers::Add);
        assert_eq!(table.piece_table[1].start_index, 6);
        assert_eq!(table.piece_table[1].length, 6);
        assert_eq!(table.piece_table[2].buffer, PieceTableBuffers::Original);
        assert_eq!(table.piece_table[2].start_index, 5);
        assert_eq!(table.piece_table[2].length, 9);
        assert_eq!(table.piece_table[3].buffer, PieceTableBuffers::Add);
        assert_eq!(table.piece_table[3].start_index, 12);
        assert_eq!(table.piece_table[3].length, 13);
        assert_eq!(
            (0..table.len()).map(|i| table.index(i)).collect::<String>(),
            "Lorem  dolor sit amet, consectetur".to_string()
        );

        println!(
            "{:?}",
            (0..table.len()).map(|i| table.index(i)).collect::<String>()
        );
        println!("{:?}", table.original_buffer);
        println!("{:?}", table.add_buffer);
        for i in 0..table.piece_table.len() {
            println!("{:?}", table.piece_table[i]);
        }

        table.delete(18);
        table.delete(16);
        table.delete(16);

        assert_eq!(table.piece_table.len(), 5);
        assert_eq!(table.piece_table[0].buffer, PieceTableBuffers::Add);
        assert_eq!(table.piece_table[0].start_index, 0);
        assert_eq!(table.piece_table[0].length, 6);
        assert_eq!(table.piece_table[1].buffer, PieceTableBuffers::Add);
        assert_eq!(table.piece_table[1].start_index, 6);
        assert_eq!(table.piece_table[1].length, 6);
        assert_eq!(table.piece_table[2].buffer, PieceTableBuffers::Original);
        assert_eq!(table.piece_table[2].start_index, 5);
        assert_eq!(table.piece_table[2].length, 4);
        assert_eq!(table.piece_table[3].buffer, PieceTableBuffers::Original);
        assert_eq!(table.piece_table[3].start_index, 12);
        assert_eq!(table.piece_table[3].length, 2);
        assert_eq!(table.piece_table[4].buffer, PieceTableBuffers::Add);
        assert_eq!(table.piece_table[4].start_index, 12);
        assert_eq!(table.piece_table[4].length, 13);
        assert_eq!(
            (0..table.len()).map(|i| table.index(i)).collect::<String>(),
            "Lorem  dolor sitet, consectetur".to_string()
        );

        table.delete(5);
        table.delete(4);
        table.delete(3);

        assert_eq!(table.piece_table.len(), 5);
        assert_eq!(table.piece_table[0].buffer, PieceTableBuffers::Add);
        assert_eq!(table.piece_table[0].start_index, 0);
        assert_eq!(table.piece_table[0].length, 3);
        assert_eq!(table.piece_table[1].buffer, PieceTableBuffers::Add);
        assert_eq!(table.piece_table[1].start_index, 6);
        assert_eq!(table.piece_table[1].length, 6);
        assert_eq!(table.piece_table[2].buffer, PieceTableBuffers::Original);
        assert_eq!(table.piece_table[2].start_index, 5);
        assert_eq!(table.piece_table[2].length, 4);
        assert_eq!(table.piece_table[3].buffer, PieceTableBuffers::Original);
        assert_eq!(table.piece_table[3].start_index, 12);
        assert_eq!(table.piece_table[3].length, 2);
        assert_eq!(table.piece_table[4].buffer, PieceTableBuffers::Add);
        assert_eq!(table.piece_table[4].start_index, 12);
        assert_eq!(table.piece_table[4].length, 13);
        assert_eq!(
            (0..table.len()).map(|i| table.index(i)).collect::<String>(),
            "Lor dolor sitet, consectetur".to_string()
        );
    }
}
