
use std::collections::HashMap;
fn main(){

    // Create a HashMap
    let mut hashmap:HashMap<i32, String> = HashMap::new();

    // Add a value 
    hashmap.insert(1, "Apple".to_string());
    hashmap.insert(2, String::from("Banana"));
    hashmap.insert(3, String::from("Orange"));
    // Access values

    let first_fruit = hashmap.get(&1).unwrap();
    println!("first fruit : {:?}", first_fruit);

    // Remove elements
    let remove_first = hashmap.remove(&1).unwrap();
    println!("Fruit removed : {:?}",remove_first);
    println!("1st Fruits Availables{:?}", hashmap);

    // Change elements inserting the same key
    hashmap.insert(1, String::from("Apple"));
    hashmap.insert(3, String::from("Strawberry"));
    println!("2nd Fruits Availables{:?}", hashmap);

    // Others methods
    // Length
    println!("Number of elements :{:?}", hashmap.len());

    // For in a hashmap, 
    for (key,fruit) in hashmap.clone(){
        println!("{key}: {fruit}");
    }

    for fruit in hashmap.values(){
        println!("{fruit}");
    }

    for key in hashmap.keys(){
        println!("{key}");
    }

    println!("hashmap constain the value 6 ? {}", hashmap.contains_key(&6));

}