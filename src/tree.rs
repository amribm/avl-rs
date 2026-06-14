use std::{
    fmt::Display,
    mem::{replace, swap, take},
};

#[derive(Default, Debug)]
enum BSTNode<T: Ord + Display> {
    #[default]
    Nil,
    Node {
        left: ChildNode<T>,
        right: ChildNode<T>,
        value: T,
        height: i32,
    },
}

type ChildNode<T> = Box<BSTNode<T>>;

impl<T: Ord + Display> BSTNode<T> {
    fn new(value: T) -> Self {
        BSTNode::Node {
            left: Box::new(BSTNode::Nil),
            right: Box::new(BSTNode::Nil),
            value: value,
            height: 0,
        }
    }

    fn contains(self: &ChildNode<T>, val: &T) -> bool {
        match **self {
            Self::Nil => false,
            Self::Node {
                ref left,
                ref right,
                ref value,
                ..
            } => {
                if val == value {
                    return true;
                } else if val < value {
                    return left.contains(val);
                } else {
                    return right.contains(val);
                }
            }
        }
    }

    fn insert_balanced(self: &mut ChildNode<T>, val: T) -> bool {
        match **self {
            Self::Nil => {
                **self = Self::new(val);
            }
            Self::Node {
                ref mut left,
                ref mut right,
                ref value,
                ..
            } => {
                if val == *value {
                    return false;
                } else if val < *value {
                    left.insert_balanced(val);
                } else {
                    right.insert_balanced(val);
                }

                self.update_height();
            }
        }

        self.rebalance();
        true
    }

    fn delete_balanced(self: &mut ChildNode<T>, val: &T) -> bool {
        match **self {
            Self::Nil => false,
            Self::Node {
                ref mut left,
                ref mut right,
                ref mut value,
                ..
            } => {
                let deleted = if val < value {
                    left.delete_balanced(val)
                } else if val > value {
                    right.delete_balanced(val)
                } else {
                    let has_left = !matches!(**left, Self::Nil);
                    let has_right = !matches!(**right, Self::Nil);

                    match (has_right, has_left) {
                        (false, false) => {
                            **self = Self::Nil;
                        }
                        (true, false) => *self = take(self.get_right()),
                        (false, true) => *self = take(self.get_left()),
                        (true, true) => {
                            let smallest_node = right.take_smallest_in_subtree();

                            if let Self::Node {
                                value: smallest_value,
                                ..
                            } = *smallest_node
                            {
                                *value = smallest_value
                            }
                        }
                    };

                    true
                };
                if deleted {
                    self.update_height();
                    self.rebalance();
                }

                deleted
            }
        }
    }

    fn get_height(self: &ChildNode<T>) -> i32 {
        match **self {
            Self::Nil => -1,
            Self::Node { height, .. } => height,
        }
    }

    fn get_left<'a>(self: &'a mut ChildNode<T>) -> &'a mut ChildNode<T> {
        match **self {
            Self::Nil => panic!("tried to get left of BST Node"),
            Self::Node { ref mut left, .. } => left,
        }
    }

    fn get_right<'a>(self: &'a mut ChildNode<T>) -> &'a mut ChildNode<T> {
        match **self {
            Self::Nil => panic!("tried to get right of BST Node"),
            Self::Node { ref mut right, .. } => right,
        }
    }

    fn update_height(self: &mut ChildNode<T>) {
        match **self {
            Self::Nil => (),
            Self::Node {
                ref left,
                ref right,
                ref mut height,
                ..
            } => {
                *height = left.get_height().max(right.get_height()) + 1;
            }
        }
    }

    fn rotate_right(self: &mut ChildNode<T>) {
        let lr = take(self.get_left().get_right());

        let left = replace(self.get_left(), lr);
        let mut s = replace(self, left);
        swap(self.get_right(), &mut s);

        self.get_right().update_height();
        self.update_height();
    }

    fn rotate_left(self: &mut ChildNode<T>) {
        let rl = take(self.get_right().get_left());

        let right = replace(self.get_right(), rl);
        let mut s = replace(self, right);
        swap(self.get_left(), &mut s);

        self.get_left().update_height();
        self.update_height();
    }

    fn is_imbanced(self: &ChildNode<T>) -> bool {
        match **self {
            Self::Nil => false,
            Self::Node {
                ref left,
                ref right,
                ..
            } => left.get_height().abs_diff(right.get_height()) > 1,
        }
    }
    fn left_heavy(self: &ChildNode<T>) -> bool {
        match **self {
            Self::Nil => false,
            Self::Node {
                ref left,
                ref right,
                ..
            } => left.get_height() > right.get_height(),
        }
    }

    fn right_heavy(self: &ChildNode<T>) -> bool {
        match **self {
            Self::Nil => false,
            Self::Node {
                ref left,
                ref right,
                ..
            } => left.get_height() < right.get_height(),
        }
    }

    fn take_smallest_in_subtree(self: &mut ChildNode<T>) -> ChildNode<T> {
        match **self {
            Self::Nil => panic!("empty subtree"),
            Self::Node { ref mut left, .. } => {
                if let Self::Nil = **left {
                    let right = take(self.get_right());
                    let s = take(self);

                    **self = *right;
                    s
                } else {
                    let smallest = left.take_smallest_in_subtree();
                    self.update_height();
                    self.rebalance();
                    smallest
                }
            }
        }
    }

    fn rebalance(self: &mut ChildNode<T>) {
        if !self.is_imbanced() {
            return;
        }

        if self.left_heavy() {
            let left = self.get_left();

            if left.left_heavy() {
                self.rotate_right();
            } else {
                left.rotate_left();
                self.rotate_right();
            }
        } else {
            let right = self.get_right();
            if right.right_heavy() {
                self.rotate_left();
            } else {
                right.rotate_right();
                self.rotate_left();
            }
        }
    }

    fn inorder(mut self: ChildNode<T>, arr: &mut Vec<T>) {
        if let Self::Nil = *self {
            return;
        }
        let left = take(self.get_left());
        let right = take(self.get_left());
        left.inorder(arr);
        let curr = take(&mut *self);
        if let Self::Node { value, .. } = curr {
            arr.push(value);
        }
        right.inorder(arr);
    }
    fn is_nil(self: &ChildNode<T>) -> bool {
        if let Self::Nil = **self {
            return true;
        }
        false
    }

    fn pretty_print_tree(self: &ChildNode<T>) {
        if self.is_nil() {
            return;
        }

        if let Self::Node { ref value, .. } = **self {
            println!("{}", value);
        }

        self.print_subtree("".to_string());
    }

    fn print_subtree(self: &ChildNode<T>, prefix: String) {
        if self.is_nil() {
            return;
        }

        if let Self::Node {
            ref right,
            ref left,
            ref value,
            ..
        } = **self
        {
            let has_left = !left.is_nil();
            let has_right = !right.is_nil();

            if !has_right && !has_left {
                return;
            }

            print!("{}", prefix);
            if has_left && has_right {
                print!("├──");
            } else if !has_left && has_right {
                print!("└──");
            }

            if has_right {
                let print_strand = has_left
                    && has_right
                    && (if let Self::Node {
                        left: ref r_left,
                        right: ref r_right,
                        ..
                    } = **right
                    {
                        !r_left.is_nil() || !r_right.is_nil()
                    } else {
                        false
                    });
                if let Self::Node { ref value, .. } = **right {
                    println!("{}", value);
                }

                let new_prefix = prefix.clone() + if print_strand { "|   " } else { "   " };
                right.print_subtree(new_prefix);
            }

            if has_left {
                if has_right {
                    print!("{}", prefix);
                }
                if let Self::Node { ref value, .. } = **left {
                    println!("└── {}", value);
                }
                left.print_subtree(prefix + "   ");
            }
        }
    }
}

#[derive(Debug)]
pub struct BST<T: Ord + Display> {
    root: ChildNode<T>,
    size: u32,
}

impl<T: Ord + Display> BST<T> {
    pub fn new() -> BST<T> {
        BST {
            root: Box::new(BSTNode::Nil),
            size: 0,
        }
    }

    pub fn contains(&self, val: &T) -> bool {
        self.root.contains(val)
    }

    pub fn insert(&mut self, val: T) -> bool {
        let inserted = self.root.insert_balanced(val);
        if inserted {
            self.size += 1;
        }
        inserted
    }

    pub fn delete(&mut self, val: &T) -> bool {
        let deleted = self.root.delete_balanced(val);
        if deleted {
            self.size -= 1;
        }
        deleted
    }

    pub fn pretty_print(&self) {
        self.root.pretty_print_tree();
    }

    pub fn get_as_inorder_vec(self) -> Vec<T> {
        let mut res = Vec::new();
        self.root.inorder(&mut res);
        res
    }
}
