fn main() {
    let number_list = vec![21, 23, 12, 52, 6, 3, 723, 53];

    let largest_number = largest(&number_list);

    println!("The largest number is {}", largest_number);

    let number_list = vec![21, 23, 12, 52, 6, 3, 8, 80, 53];

    let largest_number = largest(&number_list);

    println!("The largest number is {}", largest_number);
}

fn largest(list: &[i32]) -> &i32 {
    let mut largest_number = &list[0];

    for i in list {
        if i > largest_number {
            largest_number = i
        }
    }

    largest_number
}
