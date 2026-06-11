mod tree;

use tree::BST;

fn main() {
    let mut bst = BST::new();
    for i in 1..=70 {
        bst.insert(i);
    }
    bst.pretty_print();
    for i in 10..=30 {
        bst.delete(&i);
    }
    for i in 1..=9 {
        assert!(bst.contains(&i));
    }
    for i in 10..=30 {
        assert!(!bst.contains(&i));
    }
    for i in 31..=63 {
        assert!(bst.contains(&i));
    }
    bst.delete(&1);
    bst.delete(&3);
    bst.delete(&7);
    bst.pretty_print();
    println!("BST operation successful!");
    println!("inorder BST: {:?}", bst.get_as_inorder_vec());
}
