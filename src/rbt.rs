use std::collections::LinkedList;

///Red-Black trees are a type of self balancing binary search tree
/*
 * The tree structure contains for pieces of information:
 *      Nodes: A vector filled with the Nodes
 *      Edgelist: A vector of vectors (size three) contains the edges between each 
 *      Empty: A stack of empty indicies in the vector. This is to save space so that the tree
 *      doesn't grow monotonically in memory
 *      color: a vector of boolean values to denote the color (red or black) of the node
 */
pub struct Tree<T> {
   graph: Vec<Node<T>> ,
   edge_list: Vec<Vec<Option<usize>>>, // Parent, left child, right child
   empty: LinkedList<usize>, // Linked list so that we don't have to worry about allocation
    // Linked list has a is_empty function. That will be useful for insertions
    color: Vec<bool>,
    root: Option<usize>, // index with the root of the tree
}
// Nodes simply contain the data
struct Node<T> {
    data: Box<T>,
}

impl<'a, T: std::cmp::PartialOrd /*+ std::marker::Copy*/> Tree<T>  {
    /// Function to create a new Red-Black Tree. Returns an empty tree
    pub fn new() -> Tree<T> {
        Tree { graph: Vec::new(), edge_list: Vec::new(), empty: LinkedList::new(), color: Vec::new(), root: None}
    }
    /// Insert does exactly what it says, it inserts data into the tree, rebalancing if necessary
    pub fn insert (&mut self, input: T){
        let in_pnt: Option<usize>;
        if self.graph.len() == self.empty.len() { // Tree is empty, add the root
            if !self.empty.is_empty() { // Empty stack has some value, reuse these indicies
                let opt_root = self.empty.pop_back(); // Get root value off the stack. This is popped as an Option
                let root_unwrapped = opt_root.unwrap();
                self.root = opt_root;
                self.edge_list.push(vec![None, None, None]);
                self.graph[root_unwrapped] = Node {data: Box::new(input)};
                self.color[root_unwrapped] = false;
            } else {
                self.graph.push(Node { data: Box::new(input)});
                self.edge_list.push(vec![None, None, None]); // The root has no parents, and no siblings. How sad :(
                self.root = Some(0);
                self.color.push(false);
            }
        } else{ // The tree has nodes
            let is_empty: bool;
            (in_pnt, is_empty) =  self.insert_helper(&input, self.root); // recursively insert
            match in_pnt {
                Some(i) => {
                    // Data inserted, rebalance
                    if is_empty {
                        self.graph.push(Node { data: Box::new(input) });
                    } else {
                        self.graph[i].data = Box::new(input);
                    }
                    self.insert_rebalance(i); // recolor and rebalance the tree if necessary
                }
                None =>{
                    // Nothing was inserted.
                }
        }
        }
    }

    // Private method to help recursively insert a node
    fn insert_helper(&mut self, input: &T, index: Option<usize>) -> (Option<usize>, bool) {
        let idx = index.unwrap();
        let mut is_empty : bool = true;
        let mut ret: Option<usize> = None;
        if input < &self.graph[idx].data { // Move left
            if self.edge_list[idx][1].is_some(){ // check that it's not None
                (ret, is_empty) = self.insert_helper(input, self.edge_list[idx][1]);
            } else { // insert
                if self.empty.is_empty(){ // need to add a new element
                    ret = Some(self.graph.len());
                    self.edge_list[idx][1] = ret;
                    self.edge_list.push(vec![index, None, None]);
                    self.color.push(true);
                    is_empty = true;

                } else {
                    ret = self.empty.pop_back();
                    self.edge_list[idx][1] = ret;
                    match ret {
                        Some(r) => {
                            self.edge_list[r] = vec![index, None, None];
                            self.color[r] = true;
                            is_empty = false
                        }
                        None => {
                            unreachable!();
                        }
                    }
                }
            }
        }
        if input > &self.graph[idx].data { // Move right
            if self.edge_list[idx][2].is_some(){ // check that it's not None
                (ret, is_empty) = self.insert_helper(input, self.edge_list[idx][2]);
            } else { // insert
                if self.empty.is_empty(){ // need to add a new element
                    ret = Some(self.graph.len());
                    self.edge_list[idx][2] = ret;
                    self.edge_list.push(vec![index, None, None]);
                    self.color.push(true);
                    is_empty = true;
                } else {
                    ret = self.empty.pop_back();
                    self.edge_list[idx][2] = ret;
                    match ret {
                        Some(r) => {
                            self.edge_list[r] = vec![index, None, None];
                            self.color[r] = true;
                            is_empty = true;
                        }
                        None => {
                            unreachable!();
                        }
                    }
                }
            }
        }
        (ret, is_empty)
    }
    
        // Private helper function to rebalance the tree after an insert
    fn insert_rebalance(&mut self, index: usize) {
        //Recolor
        match self.edge_list[index][0] { // Check if parent is None
            Some(p) => {
                match self.edge_list[p][0] { // check grandfather
                    Some(g) => {
                        if self.edge_list[g][1] == Some(p) { // Node's parent is the left branch
                            match self.edge_list[g][2] {
                                Some(u) => {
                                    if self.color[u] { // Uncle is red
                                        // Change uncle and parent to 
                                        self.color[u] = false;
                                        self.color[p] = false;
                                        self.color[g] = true;
                                        self.insert_rebalance(g);
                                    } else {
                                        if self.edge_list[p][1] == Some(index) { // LL case
                                            self.left_left_rotation(p);
                                        } else { // LR case
                                            self.left_right_rotation(p);
                                        }
                                    }
                                }
                                None => {
                                    // No uncle exists. All None branches are black
                                    if self.edge_list[p][1] == Some(index) { // LL case
                                        self.left_left_rotation(p);
                                    } else { // LR case
                                            self.left_right_rotation(p);
                                    }
                                }
                            }
                        } else { // Node's parent must be right branch
                            match self.edge_list[g][2] {
                                Some(u) => {
                                    if self.color[u] { // Uncle is red
                                        // Change uncle and parent to 
                                        self.color[u] = false;
                                        self.color[p] = false;
                                        self.color[g] = true;
                                        self.insert_rebalance(g);
                                    }
                                    else {
                                        if self.edge_list[p][1] == Some(index) { // RL case
                                            self.right_left_rotation(p);
                                        } else { // RR case
                                            self.right_right_rotation(p);
                                        }
                                    }
                                }
                                None => {
                                    // No uncle exists. All None branches are black
                                    if self.edge_list[p][1] == Some(index) { // RL case
                                        self.right_left_rotation(p);
                                    } else { // RR case
                                        self.right_right_rotation(p);
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        // Node's parent is the root thus the node has no uncle
                    }
                }
            }
            None => {
                // Node is the root
            }
        }
    }

    // Function to perform a right rotation along the grandfather node 
    fn left_left_rotation(&mut self, index: usize) {
        let rc = self.edge_list[index][2]; // right child of node
        let idx = Some(index);
        let c = self.color[index];
        match self.edge_list[index][0] {
            Some(p) => {
                self.edge_list[index][2] = self.edge_list[index][0]; // Assign right child to parent
                match self.edge_list[p][0] {
                    Some(g) => {
                        if self.edge_list[g][1] == self.edge_list[index][0] {
                            // Parent is left child of grandfather
                            self.edge_list[g][1] = idx;
                        } else {
                            // Parent is right child
                            self.edge_list[g][2] = idx;
                        }
                    }
                    None => {
                        // p was the root
                        self.root = idx;
                    }
                }
                self.edge_list[index][0] = self.edge_list[p][0]; // Assign parent to grandfather
                self.edge_list[p][0] = self.edge_list[p][1]; // Assign parent to node at index
                self.edge_list[p][1] = rc; // Assign left child to previous right child of node at index
                // swap colors
                self.color[index] = self.color[p];
                self.color[p] = c;
            }
            None => {
                unreachable!();
            }
        }
    }

    // Function to perform a right rotation along the grandfather node 
    fn right_right_rotation(&mut self, index: usize) {
        let lc = self.edge_list[index][1]; // left child of node
        let idx = Some(index);
        let c = self.color[index];
        match self.edge_list[index][0] {
            Some(p) => {
                self.edge_list[index][1] = self.edge_list[index][0]; // Assign left child to parent
                match self.edge_list[p][0] {
                    Some(g) => {
                        if self.edge_list[g][1] == self.edge_list[index][0] {
                            // Parent is left child of grandfather
                            self.edge_list[g][1] = idx;
                        } else {
                            // Parent is right child
                            self.edge_list[g][2] = idx;
                        }
                    }
                    None => {
                        // p was the root
                        self.root = idx;
                    }
                }
                self.edge_list[index][0] = self.edge_list[p][0]; // Assign parent to grandfather
                self.edge_list[p][0] = self.edge_list[p][2]; // Assign parent to node at index
                self.edge_list[p][2] = lc; // Assign left child to previous right child of node at index
                // swap colors
                self.color[index] = self.color[p];
                self.color[p] = c;
            }
            None => {
                unreachable!();
            }
        }
    }
    
    // Left rotation around index
    fn left_right_rotation(&mut self, index: usize) {
        let idx = Some(index);
        match self.edge_list[index][2] {
            Some(child) => {
                self.edge_list[child][0] = self.edge_list[index][0]; // set child parent to grandfather
                self.edge_list[index][0] = self.edge_list[index][2]; // set parent to right child
                self.edge_list[index][2] = self.edge_list[child][1]; // left child must become right child of index
                match self.edge_list[child][1] {
                    Some(lc) => {
                        self.edge_list[lc][0] = idx; // set node at index as parent of left child of child
                    }
                    None => {

                    }
                }
                match self.edge_list[child][0] { // place child as child of grandfather
                    Some(g) => {
                        if self.edge_list[g][1] == idx {
                            self.edge_list[g][1] = Some(child);
                        } else {
                            self.edge_list[g][2] = Some(child);
                        }
                    }
                    None => {
                        self.root = Some(child);
                    }
                }
                self.edge_list[child][1] = idx; // index becomes left child
                self.left_left_rotation(child);
            }
            None => {
                unreachable!();
            }
        }
    }

    // Right rotation around index
    fn right_left_rotation(&mut self, index: usize) {
        let idx = Some(index);
        match self.edge_list[index][2] {
            Some(child) => {
                self.edge_list[child][0] = self.edge_list[index][0]; // set child parent to grandfather
                self.edge_list[index][0] = self.edge_list[index][1]; // set parent to right child
                self.edge_list[index][1] = self.edge_list[child][2]; // left child must become right child of index
                match self.edge_list[child][2] {
                    Some(rc) => {
                        self.edge_list[rc][0] = idx;
                    }
                    None => {

                    }
                }
                match self.edge_list[child][0] { // Place child as child of grandfather
                    Some(g) => {
                        if self.edge_list[g][1] == idx {
                            self.edge_list[g][1] = Some(child);
                        } else {
                            self.edge_list[g][2] = Some(child);
                        }
                    }
                    None => {
                        self.root = Some(child);
                    }
                }
                self.edge_list[child][2] = idx; // index becomes left child
                self.right_right_rotation(child);
            }
            None => {
                unreachable!();
            }
        }
    }
        ///Function to search the tree for a given value. Returns true if found, false otherwise.
    pub fn contains(&self, input: T) -> bool{
        self.contains_recursive(self.root, &input)
    }

    fn contains_recursive(&self, index: Option<usize>, input: &T) -> bool {
        let mut ret = false;
        match index {
            Some(idx) => {
                let d = self.graph[idx].data.as_ref();
                if d == input {
                    ret = true;
                } else if d > input {
                    ret = self.contains_recursive(self.edge_list[idx][1], input);
                } else {
                    ret = self.contains_recursive(self.edge_list[idx][2], input);
                }
            }
            None => {
                // Do nothing
            }
        }
        ret
    }

    /// in_order traverses the tree and returns a list of the nodes in depth first order
    pub fn in_order(&self) -> LinkedList<&T> {
        let mut ll: LinkedList<&T> = LinkedList::new();
        if self.graph.len() == self.empty.len() { // Tree is empty
           // Do nothing 
        } else {
            ll.append(& mut self.ino_recursive(self.root));
        }
        ll
    }
    // Helper function to recursively build the linked list for in order traversal
    fn ino_recursive(&self, index: Option<usize>) -> LinkedList<&T> {
        let mut ll :LinkedList<&T> = LinkedList::new();
        match index {
            Some(i) => {
                let left: Option<usize> = self.edge_list[i][1];
                let right: Option<usize> = self.edge_list[i][2];
                ll.append(&mut self.ino_recursive(left));
                ll.push_back(self.graph[i].data.as_ref());
                ll.append(&mut self.ino_recursive(right));
            }
            None => { // Nothing to recurse into
                // Do nothing
            }
        }
        ll
    }
}
