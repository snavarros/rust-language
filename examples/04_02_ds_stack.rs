/// To implement a stack, we can use easily the methods 
/// from the vector library of rust

/// A stack use a LIFO methodology
/// Last In is the First Out
/// Like when stack dishes on the kitchen and you take the first upper one


#[derive(Debug)]
struct Stack<T>{
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn pop(&mut self)-> Option<T>{
        self.stack.pop()
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

}



fn main() {
    let mut stack:Stack<usize> = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);

    stack.pop();

    println!("{:?}", stack)

}