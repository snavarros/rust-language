use std::collections::HashMap;

fn main() {
    let mut graph = HashMap::new();
    graph.insert('a', vec!('c','b'));
    graph.insert('b', vec!('d'));
    graph.insert('c', vec!('e'));
    graph.insert('d', vec!('f'));
    graph.insert('e', vec!());
    graph.insert('f', vec!());

    println!("{:?}", graph);
    depth_first_print(graph,'a');

    
}

fn depth_first_print(graph: HashMap<char,Vec<char>>, source:char){
    let mut stack = vec!(source);
    while !stack.is_empty() {
        let current = stack.pop().unwrap();
        println!("{:?} ",current);
        for value in  graph.get(&current).unwrap(){
            stack.push(*value);
        };
    }
}