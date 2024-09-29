use std::{cell::RefCell, rc::Rc};

// RefCell allows for borrowing of data. Very annoying to deal with
// In the future, I may just use Box<T>.
type TreeNodeRef = Rc<RefCell<TreeNode>>;

// BST data structure
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

// functions that act on the BinarySearchTree struct
impl BinarySearchTree {
  /**
   * Create new binary search tree from nothing
   * Returns into itself an empty BST
   */
  pub fn new() -> Self {
    BinarySearchTree { root: None }
  }

  /**
   * Inserts a node into itself with the given value
   * takes as input the i32 value which to insert
   * returns void
   */
  pub fn insert(&mut self, val: i32) {
      let new_node = Rc::new(RefCell::new(TreeNode {
        val,
        left: None,
        right: None,
      }));

      // if root is empty, make this new node the root
      // if not, insert it as usual using the insert_node function
      match &self.root {
        Some(root) => BinarySearchTree::insert_node(root.clone(), new_node),
        None => self.root = Some(new_node),
      }
  }

  /**
   * Function to insert node following BST rules
   * takes as input 2 TreeNodes: the root of the subtree and the node to insert
   * returns void
   */
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
      drop(current_borrow);
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
      drop(current_borrow);
    }
  }

  /**
   * Function that takes a vector of i32s and returns a new BST from those values
   * uses a for loop to repeatedly insert values in the order they appear in the vector
   * returns itself, a new BST
   */
  pub fn from_vec(values: Vec<i32>) -> Self {
    let mut bst = BinarySearchTree::new();
    for value in values {
      bst.insert(value);
    }
    bst
  }

  /**
   * Wrapper function to print in-order traversal of the tree
   */
  pub fn in_order_traversal(&self) {
    if let Some(root) = &self.root {
      BinarySearchTree::in_order_helper(root.clone(), 0);
    }
  }

  // Helper function for in-order traversal
  fn in_order_helper(node: TreeNodeRef, depth: i32) {
    let node_borrow = node.borrow();
    // if left exists, pursue left subtree
    if let Some(left) = &node_borrow.left {
      BinarySearchTree::in_order_helper(left.clone(), depth+1);
    }
    // print value
    println!("{}: {}", node_borrow.val, depth);
    // if right exists, pursue right subtree
    if let Some(right) = &node_borrow.right {
      BinarySearchTree::in_order_helper(right.clone(), depth+1);
    }
  }

/**
 * Wrapper function to delete node with given value
 */
  pub fn delete(&mut self, key: i32) {
    if let Some(root) = &self.root {
      self.root = Self::delete_node(root.clone(), key);
    }
  }

  /**
   * Function to delete a node
   * Takes as input the root of the subtree (useful for recursion)
   * and the key value which to delete
   * returns the root of the updated tree
   */
  fn delete_node(root: TreeNodeRef, key: i32) -> Option<TreeNodeRef> {
    // the theme of this function is annoying Rust memory safety.
    // I'm not even sure if this works all of the time, it's that
    // confusing and I coded it.
    // ! NOTE: you cannot return an empty tree, so do not try to delete the root of a 1-element BST

    let mut current = Some(root.clone());
    let mut parent: Option<TreeNodeRef> = None;
    let mut is_left_child = false;

    // if the root of the subtree is the key, and the root has only 1 child, this takes care of that
    // sort of a special case since otherwise my implementation hinges on the parent value.
    let other_node_ref = if let Some(other_node_ref) = current.clone() { other_node_ref } else { todo!() };
    let other_node_borrow = other_node_ref.borrow();
    if other_node_borrow.val == key {
      // right subtree empty
      if other_node_borrow.right.is_none() && other_node_borrow.left.is_some() {
        return other_node_borrow.left.clone();
      }
      // left subtree empty
      if other_node_borrow.left.is_none() && other_node_borrow.right.is_some() {
        return other_node_borrow.right.clone();
      }
    }

    // this is also a theme of this function. What does dropping mean? Not really sure,
    // it's like relenquishing control, but sometimes it works sometimes it causes more
    // issues than it fixes. Again, easily the most annoying part of Rust.
    drop(other_node_borrow);

    // given it is not the root, find the node to delete and its parent
    while let Some(node_ref) = current.clone() {
      let node_borrow = node_ref.borrow();

      if key < node_borrow.val {
        // update parent and current values
        parent = Some(node_ref.clone());
        // need to clone since Rust will not let you take ownership otherwise
        current = node_borrow.left.clone();
        is_left_child = true; // Move left
      } else if key > node_borrow.val {
        // update parent and current values
        parent = Some(node_ref.clone());
        current = node_borrow.right.clone();
        is_left_child = false; // Move right
      } else {
        // node to delete found
        break;
      }
    }

    // if the node DNE, return the original tree
    if current.is_none() {
      return Some(root); // Node not found
    }

    // prepare to delete node 
    let node_to_delete = current.unwrap();
    let mut node_borrow = node_to_delete.borrow_mut();

      // Case 1: Node has no children (leaf node)
      if node_borrow.left.is_none() && node_borrow.right.is_none() {
        if let Some(parent_ref) = parent {
          let mut parent_borrow = parent_ref.borrow_mut();
          if is_left_child {
            parent_borrow.left = None; // Remove parent's reference to this node
          } else {
            parent_borrow.right = None; // Remove parent's reference to this node
          }
        }
        return Some(root); // Return the root of the modified tree
      }

    // Case 2: Node has one child
    // left subtree is empty
    if node_borrow.left.is_none() {
      if let Some(parent_ref) = parent {
        let mut parent_borrow = parent_ref.borrow_mut();
        if is_left_child {
          parent_borrow.left = node_borrow.right.clone(); // link parent to right child
        } else {
          parent_borrow.right = node_borrow.right.clone(); // link parent to right child
        }
      }
      return Some(root); // Return the root of the modified tree
    }
    // right subtree is empty
    if node_borrow.right.is_none() {
      if let Some(parent_ref) = parent {
        let mut parent_borrow = parent_ref.borrow_mut();
        if is_left_child {
          parent_borrow.left = node_borrow.left.clone(); // Link to left child
        } else {
          parent_borrow.right = node_borrow.left.clone(); // Link to left child
        }
      }
      return Some(root); // Return the root of the modified tree
    }

    // Case 3: Node has two children (hardest case)

    // Clone the right subtree to find the in-order successor (minimum value in the right subtree)
    let right_subtree = node_borrow.right.clone().unwrap();
    let successor = Self::find_min(right_subtree.clone());

    let successor_borrow = successor.borrow();

    // Store the successor value
    let successor_value = successor_borrow.val;

    // although potentially sub-optimal, I had to use this little expression
    // here to take care of the case in which the node to delete has 
    // 2 children, but the right child is a leaf. The reason being I am not
    // good enough with Rust to figure out how to call the delete function
    // using the node_to_delete as the root, so it would never get the parent.
    // Had I been using C++ or Java, I'm sure this would be trivial, but c'est la vie.
    if successor_borrow.left.is_none() && successor_borrow.right.is_none() {
      node_borrow.val = successor_value;
      node_borrow.right = None;
      return Some(root);
    }

    // drop borrow
    // Rust memory laws are very confusing. You must drop them to avoid
    // these recursive "already borrowed" errors.
    drop(successor_borrow);

    // delete minimum element in right subtree
    let new_right_subtree = Self::delete_node(right_subtree, successor_value);

    // drop borrow
    drop(node_borrow);

    // assign the new right subtree
    let mut node_to_delete_borrow = node_to_delete.borrow_mut();
    node_to_delete_borrow.right = new_right_subtree;

    // update the current node's value to the successor's value
    node_to_delete_borrow.val = successor_value;

    // drop borrow to node_to_delete
    drop(node_to_delete_borrow);

    Some(root) // Return the root of the modified tree
  }

  /**
   * Function to find the in-order successor of a subtree
   * Takes as input the root of the subtree
   * returns a reference to the minimum element
   */
  fn find_min(node: TreeNodeRef) -> TreeNodeRef {
    let mut current = node;

    loop {
      // Bunsure about this weird syntax, but it works to borrow the left child
      let left_child = {
        let current_borrow = current.borrow();
        current_borrow.left.clone() // Clone the left child
      };

      // if left child exists recurse down subtree
      if let Some(left) = left_child {
        current = left; // Move to the left child
      } else {
        break; // No left child, current is the minimum
      }
    }
    current // Return the leftmost node
  }

}