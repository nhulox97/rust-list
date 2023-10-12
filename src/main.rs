use list::List;

fn main() {
    let mut my_list = List::new();

    my_list.push(1);
    my_list.push(3);
    my_list.push(6);

    println!("Value at index 1 is: {:?}", my_list.get(1));

    println!("Insert operation result: {:?}", my_list.insert(10, 1));

    println!("Value at index 1 is: {:?}", my_list.get(1));

    println!("Insert operation result: {:?}", my_list.insert(30, 3));

    println!("Value at index 3 is: {:?}", my_list.get(3));
    println!("Value at index 4 is: {:?}", my_list.get(4));
}
