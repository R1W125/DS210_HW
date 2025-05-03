use rand::Rng;

pub struct Node {
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>, 
}

impl Node {
    pub fn new() -> Self {
        Node{
            left: None,
            right: None,
        }
    }
    pub fn set_left(&mut self, child: Node) {
        self.left = Some(Box::new(child));
    }
    pub fn set_right(&mut self, child: Node) {
        self.right = Some(Box::new(child));
    }
}

pub fn build_tree() -> Box<Node> {
    let mut root = Node::new();
    let mut nodes = vec![&mut root];
    let mut rng = rand::thread_rng();

    while nodes.len() < 120 {
        let parent = nodes.remove(0);

        let ccount = rng.gen_range(1..=2);
        if nodes.len() < 120 {
            if ccount == 1 {
                // Add left child only
                let left_child = Node::new();
                parent.set_left(left_child);
                nodes.push(parent.left.as_mut().unwrap());
            } else {
                // Add both left and right children separately to avoid mutable borrow error
                let left_child = Node::new();
                parent.set_left(left_child);

                if nodes.len() < 120 {
                    let right_child = Node::new();
                    parent.set_right(right_child);
                }
                nodes.extend([parent.left.as_mut().unwrap().as_mut(), parent.right.as_mut().unwrap().as_mut()]);
            }
        }
    }
    Box::new(root)
}

pub fn diameter_of_tree(node: Option<&Box<Node>>, diameter: &mut i32) -> i32 {
    if let Some(n) = node {
        let left_height = diameter_of_tree(n.left.as_ref(), diameter);
        let right_height = diameter_of_tree(n.right.as_ref(), diameter);

        // Update the diameter (longest path passing through this node)
        *diameter = (*diameter).max(left_height + right_height);

        // Return the height of this node
        1 + left_height.max(right_height)
    } else {
        0
    }
}