use std::borrow::Borrow;
use std::cell::{RefCell, Ref, RefMut};
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Mul, Sub, Add, Div};
use std::path::Path;
use std::rc::Rc;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
//
// See https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item=String>> 
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().map(|l| l.expect("Couldn't read line?!")))
}

/// Reads the file of todays AoC, line by line
pub fn read_aoc_file(day: u32) -> impl Iterator<Item=String> {
    read_lines(format!("./res/day{}.txt", day)).expect("Couldn't read file!")
}

/// Checks for duplicate values in a slice
pub fn contains_duplicates<T: PartialEq>(slice: &[T]) -> bool {
    // see https://stackoverflow.com/a/46766782
    (1..slice.len()).any(|i| slice[i..].contains(&slice[i - 1]))
}

/// Returns three mutable borrows from a slice.
/// Panics if the indices are not sorted in ascending order or are not disjoint 
pub fn split_mut_three<'a, I>(arr: &'a mut [I], indices: (usize, usize, usize)) -> (&'a mut I, &'a mut I, &'a mut I) {
    assert!(indices.0 < indices.1);
    assert!(indices.1 < indices.2);

    let (l, r) = arr.split_at_mut(indices.2);
    let mut_two = &mut r[0];

    let (l, r) = l.split_at_mut(indices.1);
    let mut_one = &mut r[0];

    let (_l, r) = l.split_at_mut(indices.0);
    let mut_zero = &mut r[0];

    (mut_zero, mut_one, mut_two)
}

#[derive(Debug)]
pub struct TreeNode<T> {
    // Internal field holding all the data. Rc counts references, RefCell lets us mutate the data
    _ref: Rc<RefCell<PointerTreeNodeInternal<T>>>
}

#[derive(Debug)]
struct PointerTreeNodeInternal<T> {
    value: T,
    parent: Option<TreeNode<T>>,
    children: Vec<TreeNode<T>>
}

// Implementing this manually, because derive(Clone) needs T: Clone which isn't actually necessary for this implementation
impl <T> Clone for TreeNode<T> {
    fn clone(&self) -> Self {
        Self { _ref: self._ref.clone() }
    }
}

impl <T> TreeNode<T> {
    fn new(value: T, parent: Option<TreeNode<T>>) -> TreeNode<T> {
        TreeNode {
            _ref: Rc::new(RefCell::new(PointerTreeNodeInternal {
                value,
                parent,
                children: Vec::new()
            }))
        }
    }

    pub fn new_root(value: T) -> TreeNode<T> {
        TreeNode::new(value, None)
    }

    pub fn add_child(&mut self, value: T) {
        self._ref.borrow_mut().children.push(TreeNode::new(value, Some(self.clone())))
    }

    pub fn parent(&self) -> Option<TreeNode<T>> {
        RefCell::borrow(self._ref.borrow()).parent.clone()
    }

    pub fn children(&self) -> Vec<TreeNode<T>> {
        RefCell::borrow(self._ref.borrow()).children.clone()
    }

    pub fn has_children(&self) -> bool {
        !RefCell::borrow(self._ref.borrow()).children.is_empty()
    }

    pub fn value(&self) -> Ref<T> {
        let b = RefCell::borrow(self._ref.borrow());
        Ref::map(b, |data| &data.value)
    }

    pub fn value_mut(&self) -> RefMut<T> {
        let b = RefCell::borrow_mut(self._ref.borrow());
        RefMut::map(b, |data| &mut data.value)
    }

    pub fn get_root(self) -> TreeNode<T> {
        let mut current_dir = self;
        while let Some(d) = current_dir.parent() {
            current_dir = d;
        }
        current_dir
    }
}

pub fn dijkstra<'a, AF, CF, C>(node_count: usize, start: usize, adjacents: AF, costs: CF) 
where C: Sized + Clone + From<u8>,
AF: Fn(usize) -> &'a [usize],
CF: Fn(usize,usize) -> Option<C> {
    assert!(start < node_count, "Start node index larger than node count!");
    let mut costs: Vec<Option<C>> = vec![None; node_count];

    let mut visited: Vec<usize> = Vec::new();
    let mut node = start;
    while visited.len() < node_count {
        visited.push(node);
        let adjs = adjacents(node);
        for adj in adjs.iter() {
            
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add(),
    Mul(),
    Sub(),
    Div()
}

impl Operation {
    pub fn apply<T>(&self, lhs: &T, rhs: &T) -> T where T: Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T>{
        match self {
            Operation::Add() => *lhs + *rhs,
            Operation::Mul() => *lhs * *rhs,
            Operation::Sub() => *lhs - *rhs,
            Operation::Div() => *lhs / *rhs,
        }
    }

    pub fn inverse(&self) -> Operation {
        match self {
            Operation::Add() => Operation::Sub(),
            Operation::Mul() => Operation::Div(),
            Operation::Sub() => Operation::Add(),
            Operation::Div() => Operation::Mul(),
        }
    }

    pub fn isCommutative(&self) -> bool {
        match self {
            Operation::Add() => true,
            Operation::Mul() => true,
            Operation::Sub() => false,
            Operation::Div() => false,
        }
    }
}

impl TryFrom<&str> for Operation {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.chars().next() {
            Some('+') => Ok(Operation::Add()),
            Some('*') => Ok(Operation::Mul()),
            Some('-') => Ok(Operation::Sub()),
            Some('/') => Ok(Operation::Div()),
            Some(err) => Err(format!("Unknown operation: '{}'", err)),
            None => Err("Empty string!".to_owned())
        }
    }
}