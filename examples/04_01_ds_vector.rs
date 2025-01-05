
/*
    Vector and array just can store values from the same type
    Vector is a dynamic (resizable) data structure that can store a list of elements of the same type. 
    Being a resizable data structure, vectors can grow and shrink at runtime.
*/

fn main() {

    // vec![] is a macro like println!()
    let five_times_one = vec![1;5];
    println!("Vector example 1 {:?}", five_times_one);

    let mut fruits =vec!["🍇", "🍉", "🍑", "🥝", "🍒"];

    println!("first fruit = {}", fruits[0]);
    println!("second fruit = {:?}", fruits.get(1));

    // Adding a value 
    fruits.push("🍋");

    println!("Fruits = {:?}", fruits);

    // Remove by index 
    let deleted = fruits.remove(2);
    println!("Deleting {} from {:?}",deleted, fruits);

    let deleting_last_element = fruits.pop();
    println!("Deleting {:?} from {:?}",deleting_last_element, fruits);
 

}