fn main() {
    let my_array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let searched_element = 2;
    let mut element_found = false;
    let mut index_position: i8 = -1;
    for (index, element) in my_array.iter().enumerate() {
        print!("{} ", element);
        if *element == searched_element {
            element_found = true;
            index_position = index as i8;
        }
    }
    print!(
        "We searched for the number {} in the array...\n",
        searched_element
    );
    if element_found {
        print!(
            "The element we were searching for was found at index {}.\n",
            index_position
        )
    } else {
        print!("Unfortunately we couldn't find the element.\n")
    }
}
