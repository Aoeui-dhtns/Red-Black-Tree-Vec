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
struct Tree<T> {
   graph: Vec<Node<T>> ,
   edge_list: Vec<Vec<Option<usize>>>, // Parent, left child, right child
   empty: LinkedList<usize>, // Linked list so that we don't have to worry about allocation
    // Linked list has a is_empty function. That will be useful for insertions
    color: Vec<bool>,
}
// Nodes simply contain the data
struct Node<T> {
    data: T,
}

impl<T> Tree<T>  {
    /// Function to create a new Red-Black Tree. Returns an empty tree
    pub fn new() -> Tree<T> {
        Tree { graph: Vec::new(), edge_list: Vec::new(), empty: LinkedList::new(), color: Vec::new()}
    }
}
