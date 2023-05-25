fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7];
    let loopidx = 0;
    /* loop {
        println!("The {} of array is {}", loopidx, arr[loopidx]);
        loopidx = loopidx + 1;
    } */

    // standard loop

    for val in arr.iter() {
        println!("The of array is {}", val);
    }
}
