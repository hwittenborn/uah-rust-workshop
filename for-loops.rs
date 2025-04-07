fn main() {
    // `vec!` defines a list of numbers. Since the input arguments could be any length, we use the
    // macro syntax.
    let numbers = vec!(1, 2, 3, 4, 5);

    // Iterate over all the numbers, and print them.
    for num in numbers {
        println!("The current number is {}", num);
    }
}
