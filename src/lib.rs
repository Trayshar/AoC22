use std::borrow::Borrow;
use std::cell::{RefCell, Ref, RefMut};
use std::fs::File;
use std::io::{self, BufRead};
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