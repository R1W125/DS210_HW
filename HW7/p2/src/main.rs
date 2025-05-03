mod tree; 
use tree::build_tree;
use tree::diameter_of_tree;
use tree::Node;

fn main() {
    let tree = build_tree();

    let mut diameter = 0;
    diameter_of_tree(Some(&tree), &mut diameter);

    println!("Diameter of the tree is: {}", diameter);
}

#[test]
fn diameter_works(){
    let mut root = Node::new();
    let mut child1 = Node::new();
    let child4 = Node::new();
    child1.set_left(child4);
    root.set_left(child1);
    let child2 = Node::new();
    root.set_right(child2);
    
    let boxed = Box::new(root);
    let mut result = 0;
    diameter_of_tree(Some(&boxed),&mut result);
    let expected = 3;
    assert_eq!(result, expected, "Fail!!!");
}

