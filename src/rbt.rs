use std::collections::LinkedList;

/// Red-Black trees are a type of self balancing binary search tree
/// balancing the tree preserves the worst case complexity of the
/// tree functionality i.e. inserts and deletes
/// As one caveat, this implementation avoids RC, and ARC usage
/// through utilization of vectors to store the nodes and the 
/// edges. This comes with the disadvantage that once the
/// vector reaches capacity it will be resized. Thus if
/// you need to continuously add nodes to the tree at runtime
/// with no known upper bound this implementation is not recommended
/*
 * The tree structure contains for pieces of information:
 *      Nodes: A vector filled with the Nodes
 *
 *      Edgelist: A vector of vectors (size three) contains the edges between each 
 *          0: parent of the node
 *          1: Left child
 *          2: right child
 *
 *      Empty: A stack of empty indicies in the vector. This is to save space so that the tree
 *      doesn't grow monotonically in memory
 *
 *      color: a vector of boolean values to denote the color (red or black) of the node
 *          true: red;
 *          false: black
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

impl<T: std::cmp::PartialOrd> Tree<T>  {
    /// Function to create a new Red-Black Tree. Returns an empty tree
    pub fn new() -> Tree<T> {
        Tree { graph: Vec::new(), edge_list: Vec::new(), empty: LinkedList::new(), color: Vec::new(), root: None}
    }

    /// With capacity function creates a new tree with the specified vector capacity.
    /// If the upper bound on the number of nodes you will need is known it is strongly
    /// recommended that you use this method to avoid potential O(n) resizing of vectors
    pub fn with_capacity(size: usize) -> Tree<T> {
        Tree { graph: Vec::with_capacity(size), edge_list: Vec::with_capacity(size), empty: LinkedList::new(), color: Vec::with_capacity(size), root: None }
    }

    /// Insert does exactly what it says, it inserts data into the tree, rebalancing if necessary
    pub fn insert (&mut self, input: T){
        let in_pnt: Option<usize>;
        if self.root.is_none() { // Tree is empty, add the root
            if !self.empty.is_empty() { // Empty stack has some value, reuse these indicies
                let opt_root = self.empty.pop_back(); // Get root value off the stack. This is popped as an Option
                let root_unwrapped = opt_root.unwrap();
                self.root = opt_root;
                self.edge_list[root_unwrapped] = vec![None, None, None];
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
                            is_empty = false;
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
    fn left_left_rotation(&mut self, idx: usize){
        let index = Some(idx);
        match self.edge_list[idx][0] {
            Some(p) => {
            // assign idx to parent
                match self.edge_list[p][0] {
                    Some(g) => {
                        if self.edge_list[g][1] == self.edge_list[idx][0] { // parent of idx is left node of grandfather
                            self.edge_list[g][1] = index;
                        } else {
                            self.edge_list[g][2] = index;
                        }
                    }
                    None => {
                        self.root = index; // Parent was the root
                    }
                }
        // assign right child to former parent's left child
                self.edge_list[p][1] = self.edge_list[idx][2];
        // update parent of former right child
                match self.edge_list[idx][2] {
                    Some(right) => {
                        self.edge_list[right][0] = self.edge_list[idx][0];
                    }
                    None => { // Do nothing
                    }
                }
        // assign parent to right child
                self.edge_list[idx][2] = self.edge_list[idx][0];
        // update parent of node
                self.edge_list[idx][0] = self.edge_list[p][0];
        // update parent of former parent
                self.edge_list[p][0] = index;
            }
            None => {
                unreachable!();
            }
        }
    }

    // Function to perform a right rotation along the grandfather node 
    fn right_right_rotation(&mut self, idx: usize){
        let index = Some(idx);
        match self.edge_list[idx][0] {
            Some(p) => {
            // assign idx to parent
                match self.edge_list[p][0] {
                    Some(g) => {
                        if self.edge_list[g][1] == self.edge_list[idx][0] { // parent of idx is left node of grandfather
                            self.edge_list[g][1] = index;
                        } else {
                            self.edge_list[g][2] = index;
                        }
                    }
                    None => {
                        self.root = index; // parent was the root
                    }
                }
        // assign left child to former parent's right child
                self.edge_list[p][2] = self.edge_list[idx][1];
        // update parent of former left child
                match self.edge_list[idx][1] {
                    Some(left) => {
                        self.edge_list[left][0] = self.edge_list[idx][0];
                    }
                    None => { // Do nothing
                    }
                }
        // assign parent to left child
                self.edge_list[idx][1] = self.edge_list[idx][0];
        // update parent of node
                self.edge_list[idx][0] = self.edge_list[p][0];
        // update parent of former parent
                self.edge_list[p][0] = index;
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
                        //if self.edge_list[g][1] == idx {
                            self.edge_list[g][1] = Some(child);
                        //} else {
                            //self.edge_list[g][2] = Some(child);
                        //}
                    }
                    None => {
                        unreachable!();
                        //self.root = Some(child);
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
        match self.edge_list[index][1] {
            Some(child) => {
                self.edge_list[child][0] = self.edge_list[index][0]; // set child parent to grandfather
                self.edge_list[index][0] = self.edge_list[index][1]; // set parent to right child
                self.edge_list[index][1] = self.edge_list[child][2]; // right child must become left child of index
                match self.edge_list[child][2] {
                    Some(rc) => {
                        self.edge_list[rc][0] = idx;
                    }
                    None => {

                    }
                }
                match self.edge_list[child][0] { // Place child as child of grandfather
                    Some(g) => {
                        //if self.edge_list[g][1] == idx {
                        //    self.edge_list[g][1] = Some(child);
                        //} else {
                            self.edge_list[g][2] = Some(child);
                        //}
                    }
                    None => {
                        unreachable!();
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

    /// Function to remove a given element from the tree. If the element is not in the tree,
    /// nothing is done.
    pub fn remove(&mut self, elem: T) {
        self.remove_recursive(self.root, &elem);
    }

    fn remove_recursive(&mut self, start: Option<usize>, elem: &T) {
        let index : Option<usize>;
        let in_order_successor: Option<usize>;
        if start == self.root { // Check if we were passed root. This avoids the overhead of a function call to search for the index if we already have it
            index = self.contains_recursive(start, &elem); // find the index of the element to be removed
        } else {
            index = start; // index was known
        }
        match index {
            Some(idx) => {
                let lcn = self.edge_list[idx][1].is_none(); // left child None
                let rcn = self.edge_list[idx][2].is_none(); // right child None
                let mut is_left_child: bool = false;
                let mut is_right_child: bool = false;
                if lcn && rcn { // both children are None, element to be deleted is leaf
                    match self.edge_list[idx][0] {
                        Some(p) => {
                            if self.edge_list[p][1] == index { // Determine if index is left or right child of parent
                                self.edge_list[p][1] = None; // Erase the connection
                            } else {
                                self.edge_list[p][2] = None;
                            }
                        }
                        None => { // element to be removed is the root and the only element in the tree
                            self.root = None;
                        }
                    }
                    self.empty.push_back(idx); // Mark index as free in the stack
                    if self.color[idx] { // 
                    }
                } else if lcn  && !rcn { // Left child is None, right child exists
                    let rc_idx = self.edge_list[idx][2].unwrap();
                    match self.edge_list[idx][0] {
                        Some(p) => {
                            if self.edge_list[p][1] == index{ // swap index for index's right child in parent node
                                self.edge_list[p][1] = self.edge_list[idx][2];
                                if !self.color[idx] && !self.color[rc_idx]{
                                    self.remove_helper(self.edge_list[p][2], false);
                                }
                            } else {
                                self.edge_list[p][2] = self.edge_list[idx][2];
                                if !self.color[idx] && !self.color[rc_idx]{
                                    self.remove_helper(self.edge_list[p][1], true);
                                }
                            }
                        }
                        None => {
                            self.root = self.edge_list[idx][2];
                            self.color[rc_idx] = false;
                        }
                    }
                    self.empty.push_back(idx);
                    let rc_idx = self.edge_list[idx][2].unwrap();
                    self.edge_list[rc_idx][0] = self.edge_list[idx][0];
                    if self.color[idx] || self.color[rc_idx] { // One of the two nodes are red
                        self.color[rc_idx] = false; // paint child black
                    }
                } else if rcn && !lcn { // right child is None, left child exists
                    let lc_idx = self.edge_list[idx][1].unwrap();
                    match self.edge_list[idx][0] {
                        Some(p) => {
                            if self.edge_list[p][1] == index{ // swap index for index's left child in parent node
                                self.edge_list[p][1] = self.edge_list[lc_idx][1];
                                if !self.color[idx] && !self.color[lc_idx]{
                                    self.remove_helper(self.edge_list[p][2], false);
                                }
                            } else {
                                self.edge_list[p][2] = self.edge_list[lc_idx][1];
                                if !self.color[idx] && !self.color[lc_idx]{
                                    self.remove_helper(self.edge_list[p][1], true);
                                }
                            }
                        }
                        None => {
                            self.root = self.edge_list[idx][1];
                            self.color[lc_idx] = false;
                        }
                    }
                    self.empty.push_back(idx);
                    self.edge_list[lc_idx][0] = self.edge_list[idx][0];
                    if self.color[idx] || self.color[lc_idx] { // One of the two nodes are red
                        self.color[lc_idx] = false; // paint child black
                    }
                } else { // Both children exist
                    // find in order successor
                    let ios = self.get_in_order_successor(self.edge_list[idx][2].unwrap()); // The in order successor must be in the right branch
                    let ios_c = self.color[ios];
                    in_order_successor = Some(ios);
                    match self.edge_list[idx][0] {
                        Some(p) => { // Change child of index's parent to index's in order successor
                            if self.edge_list[p][1] == index { // Check left child
                                self.edge_list[p][1] = in_order_successor;
                            } else { // else right child
                                self.edge_list[p][2] = in_order_successor;
                            }
                        }
                        None => {
                            self.root = in_order_successor;
                        }
                    }
                    let tmp_list = vec![self.edge_list[ios][0], self.edge_list[ios][1], self.edge_list[ios][2]]; // temp vector for swapping indices
                    if self.edge_list[idx][2] == in_order_successor { // In order successor will never be in the left branch, check if it is a child node
                        self.edge_list[ios] = vec![self.edge_list[idx][0], self.edge_list[idx][1], index]; // place index as the child instead
                        self.edge_list[idx] = vec![in_order_successor, tmp_list[1], tmp_list[2]];
                    } else {
                        self.edge_list[ios] = vec![self.edge_list[idx][0], self.edge_list[idx][1], self.edge_list[idx][2]];
                        self.edge_list[idx] = vec![tmp_list[0], tmp_list[1], tmp_list[2]];
                        match tmp_list[0] {
                            Some(ios_p) => {
                                if self.edge_list[ios_p][1] == in_order_successor{
                                    self.edge_list[ios_p][1] = index;
                                } else {
                                    self.edge_list[ios_p][2] = index;
                                }
                            }
                            None => {
                                unreachable!();
                            }
                        }
                    }
                    match self.edge_list[ios][1] {
                        Some(lc) => {
                            self.edge_list[lc][0] = in_order_successor;
                        }
                        None => {
                            
                        }
                    }
                    match self.edge_list[ios][2] {
                        Some(rc) => {
                            self.edge_list[rc][0] = in_order_successor;
                        }
                        None => {
                            
                        }
                    }
                    self.color[ios] = self.color[idx];
                    self.color[idx] = ios_c;
                    self.remove_recursive(index, elem);
                }
            }
            None => {
                // Element is not in tree, no need to remove
            }
        }
    }

    fn remove_helper(&mut self, index: Option<usize>, left: bool) {
        match index {
            Some(idx) => {
                let left_color: bool; // stores color for left child
                let right_color: bool; // stores color for right child
                match self.edge_list[idx][1] {
                    Some(left_child) => {
                        left_color = self.color[left_child];
                    }
                    None => { // None is black 
                        left_color = false;
                    }
                }
                match self.edge_list[idx][2] {
                    Some(right_child) => {
                        right_color = self.color[right_child];
                    }
                    None => { // None is black
                        right_color = false;
                    }
                }
                if !self.color[idx] { // If black
                    if left { // node is left sibling
                        if left_color { // left child is red
                            self.left_left_rotation(idx);
                        } else if right_color {
                            self.left_right_rotation(idx);
                        } else { // both children are black

                        }
                    } else { // must be right sibling
                        if right_color { // left child is red
                            self.right_right_rotation(idx);
                        } else if left_color {
                            self.right_left_rotation(idx);
                        } else { // both children are black
                        }
                        
                    }
                } else { // Is red
                    if left {
                        self.left_left_rotation(idx);
                    } else {
                        self.right_right_rotation(idx);
                    }
                }
            }
            None => { // Sibling is black and both children are black

            }
        }
    }
    // Finds the in order successor.
    fn get_in_order_successor(&self, index: usize) -> usize {
        let ret;
        match self.edge_list[index][1] {
            Some(idx) => {
                ret = self.get_in_order_successor(idx);
            }
            None => {
                // Left child is none, we must be at the smallest element in the right branch
                ret = index;
            }
        }
        ret
    }

    ///Function to search the tree for a given value. Returns true if found, false otherwise.
    pub fn contains(&self, input: T) -> bool{
        self.contains_recursive(self.root, &input).is_some() // If the item is found, an index will be returned
    }

    fn contains_recursive(&self, index: Option<usize>, input: &T) -> Option<usize> {
        let mut ret: Option<usize> = None;
        match index {
            Some(idx) => {
                let d = self.graph[idx].data.as_ref();
                if d == input { // Item found, return index
                    ret = index;
                } else if d > input { // check left
                    ret = self.contains_recursive(self.edge_list[idx][1], input);
                } else { // check right
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
        if self.root.is_none() { // Tree is empty
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

    /// Pre order traversal of the tree
    pub fn pre_order(&self) -> LinkedList<&T> {
        let mut ll: LinkedList<&T> = LinkedList::new();
        if self.root.is_none() { // Tree is empty
           // Do nothing 
        } else {
            ll.append(& mut self.pre_recursive(self.root));
        }
        ll
    }

    fn pre_recursive(&self, index: Option<usize>) -> LinkedList<&T> {
        let mut ll :LinkedList<&T> = LinkedList::new();
        match index {
            Some(i) => {
                let left: Option<usize> = self.edge_list[i][1];
                let right: Option<usize> = self.edge_list[i][2];
                ll.push_back(self.graph[i].data.as_ref());
                ll.append(&mut self.pre_recursive(left));
                ll.append(&mut self.pre_recursive(right));
            }
            None => { // Nothing to recurse into
                // Do nothing
            }
        }
        ll
    }
    
    /// Post order traversal of the tree
    pub fn post_order(&self) -> LinkedList<&T> {
        let mut ll: LinkedList<&T> = LinkedList::new();
        if self.root.is_none() { // Tree is empty
           // Do nothing 
        } else {
            ll.append(& mut self.post_recursive(self.root));
        }
        ll
    }

    fn post_recursive(&self, index: Option<usize>) -> LinkedList<&T> {
        let mut ll :LinkedList<&T> = LinkedList::new();
        match index {
            Some(i) => {
                let left: Option<usize> = self.edge_list[i][1];
                let right: Option<usize> = self.edge_list[i][2];
                ll.append(&mut self.post_recursive(left));
                ll.append(&mut self.post_recursive(right));
                ll.push_back(self.graph[i].data.as_ref());
            }
            None => { // Nothing to recurse into
                // Do nothing
            }
        }
        ll
    }
}
