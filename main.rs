#[derive(Debug)]
enum LinkedList {
    Normal(i32, Box<LinkedList>),
    Last(i32)
}

fn main() {
    let linked_list = LinkedList::Normal(1, Box::new(LinkedList::Normal(2, Box::new(LinkedList::Last(3)))));

    println!("{:?}", linked_list);
}
