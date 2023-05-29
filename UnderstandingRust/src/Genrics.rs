fn main() {
    let char_list = vec!['y', 'm', 'a', 'q'];

    let largest = large(char_list);

    println!("The Largest is {}", largest);
}

// T genric type

fn large<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest: T = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
