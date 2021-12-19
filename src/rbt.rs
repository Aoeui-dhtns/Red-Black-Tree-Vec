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
pub struct Tree<'a, T> {
   graph: Vec<Node<'a, T>> ,
   edge_list: Vec<Vec<Option<usize>>>, // Parent, left child, right child
   empty: LinkedList<usize>, // Linked list so that we don't have to worry about allocation
    // Linked list has a is_empty function. That will be useful for insertions
    color: Vec<bool>,
    root: Option<usize>, // index with the root of the tree
}
// Nodes simply contain the data
struct Node<'a, T> {
    data: &'a T,
}

impl<'a, T: std::cmp::PartialOrd /*+ std::marker::Copy*/> Tree<'a, T>  {
    /// Function to create a new Red-Black Tree. Returns an empty tree
    pub fn new() -> Tree<'static, T> {
        Tree { graph: Vec::new(), edge_list: Vec::new(), empty: LinkedList::new(), color: Vec::new(), root: None}
    }
    /// Insert does exactly what it says, it inserts data into the tree, rebalancing if necessary
    pub fn insert (&mut self, input: &'a T){
        let mut in_pnt: Option<usize> = None;
        if self.graph.len() == self.empty.len() { // Tree is empty, add the root
            if !self.empty.is_empty() { // Empty stack has some value, reuse these indicies
                let opt_root = self.empty.pop_back(); // Get root value off the stack. This is popped as an Option
                let root_unwrapped = opt_root.unwrap();
                self.root = opt_root;
                self.edge_list.push(vec![None, None, None]);
                self.graph[root_unwrapped] = Node {data: input};
                self.color[root_unwrapped] = false;
            } else {
                self.graph.push(Node { data: input});
                self.edge_list.push(vec![None, None, None]); // The root has no parents, and no siblings. How sad :(
                self.root = Some(0);
                self.color.push(false);
            }
        } else{ // The tree has nodes
            in_pnt =  self.insert_helper(&input, self.root); // recursively insert
        }
        // Data inserted, rebalance
        match in_pnt {
            Some(i) => {
                self.insert_rebalance(i); // rebalance the tree
            }
            None =>{
                // Nothing was inserted.
            }
        }
    }

    // Private method to help recursively insert a node
    fn insert_helper(&mut self, input: &'a T, index: Option<usize>) -> Option<usize> {
        let idx = index.unwrap();
        let mut ret: Option<usize> = None;
        //TODO Add color assignment
        if input < &self.graph[idx].data { // Move left
            if self.edge_list[idx][1].is_some(){ // check that it's not None
                ret = self.insert_helper(input, self.edge_list[idx][1]);
            } else { // insert
                if self.empty.is_empty(){ // need to add a new element
                    ret = Some(self.graph.len());
                    self.graph.push(Node{data: input});
                    self.edge_list[idx][1] = ret;
                    self.edge_list.push(vec![index, None, None]);

                } else {
                    ret = self.empty.pop_back();
                    self.edge_list[idx][1] = ret;
                    match ret {
                        Some(r) => {
                            self.graph[r].data = input;
                            self.edge_list[r] = vec![index, None, None];
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
                ret = self.insert_helper(input, self.edge_list[idx][2]);
            } else { // insert
                if self.empty.is_empty(){ // need to add a new element
                    ret = Some(self.graph.len());
                    self.graph.push(Node{data: input});
                    self.edge_list[idx][2] = ret;
                    self.edge_list.push(vec![index, None, None]);
                } else {
                    ret = self.empty.pop_back();
                    self.edge_list[idx][2] = ret;
                    match ret {
                        Some(r) => {
                            self.graph[r].data = input;
                            self.edge_list[r] = vec![index, None, None];
                        }
                        None => {
                            unreachable!();
                        }
                    }
                }
            }
        }
        ret
    }
    
        // Private helper function to rebalance the tree after an insert
        fn insert_rebalance(&mut self, _index: usize){
            //TODO implement function
        }
    
        ///Function to search the tree for a given value. Returns true if found, false otherwise.
        pub fn contains(&self, input: &T) -> bool{
            self.contains_recursive(self.root, input)
        }

        fn contains_recursive(&self, index: Option<usize>, input: &T) -> bool {
            let mut ret = false;
            match index {
                Some(idx) => {
                    if self.graph[idx].data == input {
                        ret = true;
                    } else if self.graph[idx].data > input {
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
    pub fn in_order(&self) -> LinkedList<T> 
        where T: Copy,
    {
        let mut ll: LinkedList<T> = LinkedList::new();
        if self.graph.len() == self.empty.len() { // Tree is empty
           // Do nothing 
        } else {
            ll.append(& mut self.ino_recursive(self.root));
        }
        ll
    }
    // Helper function to recursively build the linked list for in order traversal
    fn ino_recursive(&self, index: Option<usize>) -> LinkedList<T>
        where T: Copy,
    {
        let mut ll :LinkedList<T> = LinkedList::new();
        match index {
            Some(i) => {
                let left: Option<usize> = self.edge_list[i][1];
                let right: Option<usize> = self.edge_list[i][2];
                ll.append(&mut self.ino_recursive(left));
                ll.push_back(*self.graph[i].data);
                ll.append(&mut self.ino_recursive(right));
            }
            None => { // Nothing to recurse into
                // Do nothing
            }
        }
        ll
    }
}
