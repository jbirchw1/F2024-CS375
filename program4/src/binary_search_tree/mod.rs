use std::{cell::RefCell, rc::Rc};

// TODO: add notes here on Rc and RefCell
type TreeNodeRef = Rc<RefCell<TreeNode>>;

// TODO: add comments 

// bst data structure
#[derive(Debug, Clone)]
pub struct BinarySearchTree {
  root: Option<TreeNodeRef>,
}

// node data structure
#[derive(Debug, Clone)]
pub struct TreeNode {
  val: i32,
  left: Option<TreeNodeRef>,
  right: Option<TreeNodeRef>,
}

impl BinarySearchTree {
  // add (specifially) BST functions here
  // that is, anything that starts with the root
  // probably such as in order traversals and whatnot
  pub fn new() -> Self {
    BinarySearchTree { root: None }
  }

// Function to insert a value into the BST
  pub fn insert(&mut self, val: i32) {
      let new_node = Rc::new(RefCell::new(TreeNode {
        val,
        left: None,
        right: None,
      }));

      match &self.root {
        Some(root) => BinarySearchTree::insert_node(root.clone(), new_node),
        None => self.root = Some(new_node),
      }
  }

  // Helper function to insert a node into the correct position
  fn insert_node(current: TreeNodeRef, new_node: TreeNodeRef) {
    let mut current_borrow = current.borrow_mut();
    if new_node.borrow().val < current_borrow.val {
      // Go left
      match current_borrow.left {
        Some(ref left_child) => {
          BinarySearchTree::insert_node(left_child.clone(), new_node);
        }
        None => {
          current_borrow.left = Some(new_node);
        }
      }
    } else {
      // Go right
      match current_borrow.right {
        Some(ref right_child) => {
          BinarySearchTree::insert_node(right_child.clone(), new_node);
        }
        None => {
          current_borrow.right = Some(new_node);
        }
      }
    }
  }

  // Function to create a BST from a vector of i32s
  pub fn from_vec(values: Vec<i32>) -> Self {
    let mut bst = BinarySearchTree::new();
    for value in values {
      bst.insert(value);
    }
    bst
  }

  // In-order traversal to print the values
  // TODO: add depth
  pub fn in_order_traversal(&self) {
    if let Some(root) = &self.root {
      BinarySearchTree::in_order_helper(root.clone(), 0);
    }
  }

  // Helper function for in-order traversal
  fn in_order_helper(node: TreeNodeRef, depth: i32) {
    let node_borrow = node.borrow();
    if let Some(left) = &node_borrow.left {
        BinarySearchTree::in_order_helper(left.clone(), depth+1);
    }
    println!("{}: {}", node_borrow.val, depth);
    if let Some(right) = &node_borrow.right {
        BinarySearchTree::in_order_helper(right.clone(), depth+1);
    }
  }

  // Function to delete a node with a specified key
  pub fn delete(&mut self, key: i32) {
    if let Some(root) = &self.root {
        self.root = BinarySearchTree::delete_node(root.clone(), key);
    }
}

// Helper function to delete a node recursively
fn delete_node(node: TreeNodeRef, key: i32) -> Option<TreeNodeRef> {
    let mut node_borrow = node.borrow_mut();

    if key < node_borrow.val {
        if let Some(left) = node_borrow.left.take() {
            node_borrow.left = BinarySearchTree::delete_node(left, key);
        }
        return Some(node.clone());
    } else if key > node_borrow.val {
        if let Some(right) = node_borrow.right.take() {
            node_borrow.right = BinarySearchTree::delete_node(right, key);
        }
        return Some(node.clone());
    }

    // Now, we found the node to delete (key == node_borrow.val)

    // Case 1: Node has no children (leaf)
    if node_borrow.left.is_none() && node_borrow.right.is_none() {
        return None;
    }

    // Case 2: Node has one child
    if node_borrow.left.is_none() {
        return node_borrow.right.clone();
    }
    if node_borrow.right.is_none() {
        return node_borrow.left.clone();
    }

    // Case 3: Node has two children
    // Find the in-order successor (smallest value in the right subtree)
    let successor = BinarySearchTree::find_min(node_borrow.right.clone().unwrap());
    node_borrow.val = successor.borrow().val;

    // Delete the in-order successor
    node_borrow.right = BinarySearchTree::delete_node(node_borrow.right.clone().unwrap(), successor.borrow().val);
    
    Some(node.clone())
}

// Helper function to find the minimum value node (in-order successor)
fn find_min(node: TreeNodeRef) -> TreeNodeRef {
  let mut current = node;
  
  // Use a loop to find the minimum node
  loop {
      // Create a block scope to drop the borrow after checking left child
      let left_child = {
          let current_borrow = current.borrow(); // Borrow inside this block
          current_borrow.left.clone() // Clone the left child (Option<TreeNodeRef>)
      };

      if let Some(left) = left_child {
          current = left; // Move to the left child
      } else {
          break; // No left child, we've found the minimum node
      }
  }

  current
}
  
}