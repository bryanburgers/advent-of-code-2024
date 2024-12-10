use std::{
    collections::HashSet,
    fmt::Debug,
    num::NonZero,
    ops::{Deref, DerefMut},
};

use bindings::exports::aoc2024::day09::solver;

#[allow(warnings)]
mod bindings;

struct Component;

#[allow(unused_macros)]
macro_rules! info {
    ($($arg:tt)*) => {
        bindings::aoc::base::debug::info(&format!($($arg)*));
    };
}

impl solver::Guest for Component {
    fn solve_a(input: Vec<u8>) -> u64 {
        let mut disk = Disk::from(input);

        for i in (0..disk.len()).rev() {
            if !disk[i].is_free() {
                let first_free = disk.first_free();
                if i >= first_free {
                    disk.swap(i, first_free);
                }
            }
        }

        disk.checksum()
    }

    fn solve_b(input: Vec<u8>) -> u64 {
        // This is a huge mess and I don't care.
        let mut disk = Disk::from(input);

        let mut file_ids_checked = HashSet::new();

        let mut last_checked = disk.len() - 1;
        loop {
            if let Block::File(file_id) = disk[last_checked] {
                let mut len = 1;
                let mut pos = last_checked;
                for i in 1.. {
                    let new_idx = last_checked - i;

                    if disk[new_idx] == Block::File(file_id) {
                        len = i + 1;
                        pos = new_idx;
                        if new_idx == 0 {
                            break;
                        }
                        continue;
                    } else {
                        break;
                    }
                }
                last_checked = pos.saturating_sub(1);

                if file_ids_checked.insert(file_id) {
                    if let Some(idx) = disk.find_first_free_space_of_len(pos, len) {
                        disk.swap_chunk(idx, pos, len);
                    }
                }
            } else {
                last_checked = last_checked.saturating_sub(1);
            }

            if last_checked == 0 {
                break;
            }
        }

        disk.checksum()
    }
}

bindings::export!(Component with_types_in bindings);

#[derive(Default)]
struct Disk(Vec<Block>);

impl From<Vec<u8>> for Disk {
    fn from(input: Vec<u8>) -> Self {
        let mut disk = Disk::default();
        for (idx, &val) in input.iter().enumerate() {
            if idx % 2 == 0 {
                let file_id = FileId::from(idx as u16 / 2);
                for _ in 0..val {
                    disk.push(Block::File(file_id));
                }
            } else {
                for _ in 0..val {
                    disk.push(Block::Free);
                }
            }
        }
        disk
    }
}

impl Disk {
    pub fn first_free(&self) -> usize {
        self.iter().position(|block| block.is_free()).unwrap()
    }

    pub fn last_filled(&self) -> usize {
        self.iter().rposition(|block| !block.is_free()).unwrap()
    }

    pub fn checksum(&self) -> u64 {
        let mut checksum = 0_u64;
        for (idx, &block) in self.iter().enumerate() {
            if let Block::File(file_id) = block {
                let idx = idx as u64;
                checksum = checksum.saturating_add(idx.saturating_mul(file_id.value() as u64));
            }
        }
        checksum
    }

    pub fn find_first_free_space_of_len(&self, pos: usize, len: usize) -> Option<usize> {
        for i in 0..pos {
            if i + len < self.len() && self[i..][..len].iter().all(|block| block.is_free()) {
                return Some(i);
            }
        }
        None
    }

    pub fn swap_chunk(&mut self, start_a: usize, start_b: usize, len: usize) {
        for i in 0..len {
            self.swap(start_a + i, start_b + i);
        }
    }
}

impl Deref for Disk {
    type Target = Vec<Block>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..=self.last_filled() {
            match self.0[i] {
                Block::Free => write!(f, ".")?,
                Block::File(file_id) => write!(f, "{}", file_id.value())?,
            }
        }
        Ok(())
    }
}

impl DerefMut for Disk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Block {
    Free,
    File(FileId),
}

impl Block {
    fn is_free(&self) -> bool {
        matches!(self, Block::Free)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct FileId(NonZero<u16>);

impl Debug for FileId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl From<u16> for FileId {
    fn from(value: u16) -> Self {
        Self(NonZero::new(value + 1).unwrap())
    }
}

impl From<u8> for FileId {
    fn from(value: u8) -> Self {
        FileId::from(value as u16)
    }
}

impl FileId {
    fn value(self) -> u16 {
        self.0.get() - 1
    }
}

const _: () = const {
    if std::mem::size_of::<Block>() != 2 {
        panic!("Block must be 2 byte");
    }
    ()
};
