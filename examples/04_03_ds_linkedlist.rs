use std::collections::LinkedList;
fn main(){
    let mut ll = LinkedList::new();
    ll.push_back('a');
    ll.push_back('b');
    ll.push_back('c');
    ll.push_front('0');

    println!("{:?}", ll);

    ll.pop_front();
    ll.pop_back();

    println!("{:?}", ll);

    // ll.clear() remove all elements from the linked list

    // contains() if a element exist in the linked list 
}