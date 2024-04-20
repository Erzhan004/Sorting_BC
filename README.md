# Sorting_Rust
## Description
This library provides efficient and customizable sorting algorithms implemented in Rust. It includes implementations of popular sorting algorithms such as Quick Sort, Merge Sort, and Insertion Sort,Selection sort with support for custom comparison functions.
## Usage
To use this library, add the following to your Cargo.toml:

```sorting_library = { git = "https://github.com/Erzhan004/Sorting_BC.git" }```

Then, in your Rust application, you can use the sorting functions as follows:
```
use sorting_library::insertion_sort::insertion_sort;

fn main() {
    let mut numbers = vec![34, 7, 23, 32, 5, 62];
    insertion_sort(&mut numbers, &|a, b| a < b);
    println!("Sorted array: {:?}", numbers);
}
```
## Demo Screenshots

## License
This project is licensed under the MIT License - see the LICENSE file for details.
