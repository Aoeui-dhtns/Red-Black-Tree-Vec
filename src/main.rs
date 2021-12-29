use rbt::Tree;

mod rbt;
fn get_tree() -> Tree<i32> {
    let mut new_tree: Tree<i32> = Tree::with_capacity(100);
    new_tree.insert(40);
    //new_tree.remove(40);
    new_tree.insert(10);
    new_tree.insert(20);
    new_tree.insert(30);
    new_tree.insert(50);
    new_tree.insert(45);
    new_tree.insert(11);
    new_tree.insert(55);
    new_tree.insert(60);
    new_tree.insert(65);
    new_tree.insert(70);
    new_tree.insert(66);
    new_tree.remove(50);
    new_tree.insert(50);
    new_tree.remove(10);
    new_tree.remove(65);
    new_tree.remove(66);
    new_tree
}
fn main() {
    let new_tree :Tree<i32> = get_tree();
    
    let mut ll = new_tree.in_order();
    println!("In order:");
    while !ll.is_empty() {
        let out = ll.pop_front();
        match out {
            Some(o) => {
                print!("{}, ", o);
            }
            None => {

            }
        }
    }
    print!("\n");
    ll = new_tree.pre_order();
    println!("Pre Order");
    while !ll.is_empty() {
        let out = ll.pop_front();
        match out {
            Some(o) => {
                print!("{}, ", o);
            }
            None => {

            }
        }
    }
    print!("\n");
    ll = new_tree.post_order();
    println!("Post Order");
    while !ll.is_empty() {
        let out = ll.pop_front();
        match out {
            Some(o) => {
                print!("{}, ", o);
            }
            None => {

            }
        }
    }
    print!("\n");
    let find = new_tree.contains(10);
    println!("{}", find);
    let find = new_tree.contains(15);
    println!("{}", find);
    let find = new_tree.contains(45);
    println!("{}", find);
    let find = new_tree.contains(50);
    println!("{}", find);
    
}
