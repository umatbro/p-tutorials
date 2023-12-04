mod tree;
mod parser;
use std::{fs, io::{BufReader, BufRead}, borrow::{BorrowMut, Borrow}};

use parser::parse;
use tree::Tree;
use tree::Directory;
use parser::ParseResult::*;
use parser::ElementType as ElType;
use parser::OperationType as OpType;


fn main() {
    let file = fs::File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut tree = Tree::new();
    for line in reader.lines() {
        let l = line.unwrap();
        let res = parse(&l).unwrap().1;
        match res {
            Operation(op_type) => {
                match op_type {
                    OpType::Cd(dirname) => {
                        tree.change_dir(&dirname).unwrap();
                    },
                    OpType::Ls => {},
                }
            },
            Element(el_type) => {
                match el_type {
                    ElType::Dir { name } => {
                        let current_dir = tree.current_dir();
                        Directory::add_directory(&current_dir, name);
                    },
                    ElType::File { name, size } => {
                        let current_dir = tree.current_dir();
                        Directory::add_file(&current_dir, name, size);
                    },
                }
            },
        }
    }

    let mut result: u64 = 0;
    tree.change_dir("/").unwrap();
    let root = tree.current_dir();
    for dir in root.get_subdirs() {
        let size = dir.calc_size();
        if size <= 100000 {
            result += size;
        }
    }
    println!("Part 1 result is {}", result);

    let used_space = tree.current_dir().calc_size();
    let limit = 7 * 10_u64.pow(7);
    let space_required = 3 * 10_u64.pow(7);
    let unused_space = limit - used_space;
    println!("Used space: {}. Unused space {}", used_space, unused_space);
    assert_eq!(limit, 70000000);

    let result2: u64 = root
        .get_subdirs()
        .iter()
        .map(|d| d.calc_size())
        .filter(|size| unused_space + size > space_required )
        .min()
        .unwrap();
    
    println!("Part 2 result is {}", result2);
}
