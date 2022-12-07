#![allow(unused_variables, unused_imports)]

use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

type FileData<'a> = (&'a str, u32);
type Tree<'a> = Rc<RefCell<Node<FileData<'a>>>>; // ps. Might have over complicated this with rc<refcell.

struct Node<T> {
    data: T,
    children: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            data,
            children: Vec::new(),
        }))
    }

    fn add_child(&mut self, child: Rc<RefCell<Self>>) {
        self.children.push(child);
    }
}

impl Node<FileData<'_>> {
    fn set_size(&mut self, size: u32) {
        self.data.1 = size;
    }

    // Assumes that empty directories are files too.
    fn is_dir(&self) -> bool {
        !self.children.is_empty()
    }

    // Get a vector of all the nodes in the tree (using DFS).
    fn dirs_as_vec(&self) -> Vec<FileData> {
        fn _dirs_as_vec_inner<'a: 'b, 'b>(child: &Node<FileData<'a>>, vec: &mut Vec<FileData<'b>>) {
            if child.is_dir() {
                vec.push(child.data);
                for child in child.children.iter() {
                    _dirs_as_vec_inner(&(*child).borrow(), vec)
                }
            }
        }
        assert!(self.is_dir() && self.data.0 == "/");
        let mut data_vec = vec![];
        for child in self.children.iter() {
            _dirs_as_vec_inner(&(*child).borrow(), &mut data_vec);
        }
        data_vec
    }
}

// DFSs to set the size of each directory.
fn fill_directory_sizes(root: Tree) -> u32 {
    let (mut size, is_dir) = (root.borrow().data.1, root.borrow().is_dir());
    for child in root.borrow().children.iter() {
        size += fill_directory_sizes(child.clone());
    }
    (*root).borrow_mut().set_size(size);
    size
}

fn build_tree(input: &str) -> Tree {
    let input = input.trim();

    // File system root node.
    let root = Node::new(("/", 0));

    // A history of directories we traversed.
    let mut traverse_directory_hist = vec![root.clone()];

    // Parse the input (skipping the first one as it's always `$ cd /`).
    for line in input.split('\n').skip(1) {
        if line.starts_with("$ cd") {
            // A command to change directory.
            let name = &line["$ cd ".len()..];
            if name == ".." {
                traverse_directory_hist.pop().unwrap();
            } else {
                let current_dir = traverse_directory_hist.last().unwrap().clone();
                // Get the new directory from the directories listed under `current_dir`.
                let new_dir = current_dir
                    .borrow()
                    .children
                    .iter()
                    .find(|node| node.borrow().data.0 == name)
                    .unwrap()
                    .clone();
                traverse_directory_hist.push(new_dir);
            }
        } else if line.starts_with("$ ls") {
            // noop, will index in the next iteration.
        } else {
            // These are the insides ($ ls) of `current_directory`.
            let current_directory = traverse_directory_hist.last().unwrap();
            let (name, size) = if line.starts_with("dir") {
                (&line["dir ".len()..], 0_u32)
            } else {
                let splitting: Vec<&str> = line.split_whitespace().collect();
                (splitting[1], splitting[0].parse().unwrap())
            };
            // Insert the file or directory.
            let new_node = Node::new((name, size));
            (**current_directory).borrow_mut().add_child(new_node);
        }
    }

    fill_directory_sizes(root.clone());
    root
}

pub fn part_one(input: &str) -> Option<u32> {
    let tree = build_tree(input);
    let root = tree.borrow();
    let flatten = root.dirs_as_vec();
    Some(
        flatten
            .iter()
            .filter(|(name, size)| name != &"/" && *size < 100_000)
            .map(|(_, s)| *s)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let tree = build_tree(input);
    let root = tree.borrow();
    let flatten = root.dirs_as_vec();
    let needed_size = 30_000_000 - (70_000_000 - root.data.1);
    flatten
        .iter()
        .filter(|(name, size)| name != &"/" && *size > needed_size)
        .map(|(_, s)| *s)
        .min()
}

fn main() {
    let input = &aoc::read_file("inputs", 7);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod t7 {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
