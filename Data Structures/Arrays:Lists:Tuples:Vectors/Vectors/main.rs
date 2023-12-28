fn main() {
    // Create an empty vector of integers
    let mut my_list: Vec<i32> = Vec::new();

    // Add elements to the vector
    my_list.push(1);
    my_list.push(2);
    my_list.push(3);

    // Access elements using indexing
    println!("First element: {}", my_list[0]);

    // Iterate over the elements
    for element in &my_list {
        println!("Element: {}", element);
    }

    // Modify an element
    my_list[1] = 10;

    // Remove an element
    let removed_element = my_list.pop(); // Removes the last element
    println!("Removed element: {:?}", removed_element);

    // Print the updated vector
    println!("Updated vector: {:?}", my_list);
}
