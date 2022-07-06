pub struct TreeNode<T> {
    data: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

pub struct BinarySearchTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: std::cmp::PartialOrd + std::fmt::Display> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }
    pub fn insert(&mut self, data: T) {
        let root = self.root.take();
        self.root = self.insert_rec(root, data);
    }
    fn insert_rec(&mut self, node: Option<Box<TreeNode<T>>>, data: T) -> Option<Box<TreeNode<T>>> {
        match node {
            Some(mut n) => {
                if n.data < data {
                    n.right = self.insert_rec(n.right, data);
                } else {
                    n.left = self.insert_rec(n.left, data);
                }
                Some(n)
            }
            None => Some(Box::new(TreeNode {data: data, left: None, right: None})),
        }
    }
    pub fn find(&self, val: T) -> bool {
        self.find_rec(&self.root, val)
    }
    fn find_rec(&self, node: &Option<Box<TreeNode<T>>>, val: T) -> bool {
        match node {
            Some(n) => {
                if n.data == val {
                    true
                } else if n.data < val {
                    self.find_rec(&n.right, val)
                } else {
                    self.find_rec(&n.left, val)
                }
            }
            None => false,
        }
    }
    pub fn in_order(&self, node: &Option<Box<TreeNode<T>>>) {
        if let Some(n) = node {
            self.in_order(&n.left);
            println!("{}", &n.data);
            self.in_order(&n.right);
        }
    }
}

fn main(){
    let mut bst = BinarySearchTree::new();
    bst.insert(2);
    bst.insert(1);
    bst.insert(3);
    bst.insert(10);
    bst.insert(5);
    bst.insert(2);
    bst.in_order(&bst.root);
    
    println!("find  2 : {}", bst.find(2));
    println!("find  5 : {}", bst.find(5));
    println!("find 15 : {}", bst.find(15));
}
