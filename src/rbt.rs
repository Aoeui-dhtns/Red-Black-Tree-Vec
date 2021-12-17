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
    data: T,
}

impl<T> Tree<T>  {
    /// Function to create a new Red-Black Tree. Returns an empty tree
    pub fn new() -> Tree<T> {
        Tree { graph: Vec::new(), edge_list: Vec::new(), empty: LinkedList::new(), color: Vec::new(), root: None}
    }
    /// Insert does exactly what it says, it inserts data into the tree, rebalancing if necessary
    pub fn insert(&mut self, input: T) {
        if self.graph.len() == self.empty.len() { // Tree is empty, add the root
            if !self.empty.is_empty() {

            } else {
                self.graph.push(Node { data: input});
                self.edge_list.push(vec![None, None, None]); // The root has no parents, and no siblings. How sad :(
            }
        }
    }

    /// in_order traverses the tree and returns a list of the nodes in order
    pub fn in_order(self) -> LinkedList<T>
        where T: Copy,
    {
        let mut stack: LinkedList<T> = LinkedList::new();
        if self.graph.len() == self.empty.len() { // Tree is empty
           // Do nothing 
        } else {
            match self.root {
                Some(r) => {
                    let left_child = self.edge_list[r][1];
                    let right_child = self.edge_list[r][2];
                    stack.push_back(self.graph[r].data);
                    self.in_order_recursive(left_child);
                    self.in_order_recursive(right_child);
                }
                None => { unreachable!() }
            }
        }
        stack
    }
    // Helper function to build the stack
    fn in_order_recursive(&self, index: Option<usize>) -> LinkedList<T>{
        LinkedList::new()
    }
}
