use std::{
    collections::{HashMap, HashSet},
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

        for i in (0..disk.len()).rev() {
            if !disk[i].is_free() {
                let first_free = disk.first_free();
                if i >= first_free {
                    disk.swap(i, first_free);
                }
            }
        }

        disk.checksum() as u64
    }

    fn solve_b(_input: Vec<u8>) -> u64 {
        0
    }
}

bindings::export!(Component with_types_in bindings);

#[derive(Default)]
struct Disk(Vec<Block>);

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

#[derive(Debug, Copy, Clone)]
enum Block {
    Free,
    File(FileId),
}

impl Block {
    fn is_free(&self) -> bool {
        matches!(self, Block::Free)
    }
}

#[derive(Copy, Clone)]
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
