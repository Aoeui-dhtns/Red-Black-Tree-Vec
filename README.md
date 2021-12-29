# red-black-tree
This is an implementation of red black trees in rust. This was an exercise in learning the do's and don'ts of the borrow checker by example.

Red black trees are a type of self balancing binary search tree. The two most important properties of red black trees are

1) Every possible path has the same number of black nodes from node to leaf.
2) A red node cannot be the child of another red node

All nodes are inserted into the tree as red (excluding the root, which is always black). If an insertion violates one of the above properties
(i.e. the red-red property) the tree must be recolored and rebalanced to restore the properties. The same goes for a deletion.

Mantaining these properties ensures that the tree keeps it's optimal runtime properties.

Because this was my first foray into rust, this implementation is built using a vector to store the nodes and edges of the tree.
This makes each insertion potentially very expensive as in the worst case (vector is full) all n elements will need to be moved to a larger
block of memory. If your maximum number of nodes is known, this isn't an issue as there is a provided function to specify the size allocated
for the vectors storing the edges and nodes. Deletions are unaffected in this implementation. A stack is used to keep track of "empty" indices in the arrray.

This implementation took heavy inspiration from this blog post: http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/
