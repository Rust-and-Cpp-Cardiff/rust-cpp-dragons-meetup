// use std::io;

fn main() {

    // 3.1 Mutability
    // let mut x = 5;
    // println!("x: {}", x);
    // x = 10;
    // println!("x: {}", x);

    // println!("Hello, world!");

    // 3.2 Type annotations
    let _guess: u32 = "42".parse().expect("Not a number!");
    let _guess = "42".parse::<u32>().expect("Not a number!");

    // 3.2 Integer literals
    // println!("x: {}", 98_222);
    // println!("x: {}", 0xff);
    // println!("x: {}", 0o77);
    // println!("x: {}", 0b1111_0000);
    // println!("x: {}", b'A');

    // 3.2 Wrapping add
    // let big_u8: u8 = 200;

    // println!("Input add:");
    // let mut to_add = String::new();
    // io::stdin()
    //     .read_line(&mut to_add)
    //     .expect("Failed to read line");
    // let to_add: u8 = to_add.trim().parse().expect("Not a number!");

    // let wrapped: u8 = big_u8.wrapping_add(to_add);
    // println!("wrapped {}+{}={}", big_u8, to_add, wrapped);
    // let (overflowing, overflowed) = big_u8.overflowing_add(to_add);
    // println!("overflowing {}+{}={}, overflowed? {}", big_u8, to_add, overflowing, overflowed);
    // let saturating: u8 = big_u8.saturating_add(to_add);
    // println!("saturating {}+{}={}", big_u8, to_add, saturating);
    // let checked: u8 = big_u8.checked_add(to_add).expect("Overflowed!");
    // println!("checked {}+{}={}", big_u8, to_add, checked);

    // 3.2 Array bounds checking
    // let a = [1, 2, 3, 4, 5];
    // println!("Please enter an array index.");

    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");

    // let _element = a[index];

    // 3.3 Named loops
    // let mut grid = [[0u8; 4]; 6];
    // let mut row = 0;
    // let mut column = 0;
    // let mut counter = 0;
    // 'row_loop: loop {
    //     if row >= grid.len() {
    //       break 'row_loop;
    //     }
    //     'column_loop: loop {
    //         if column >= grid[0].len(){
    //             break 'column_loop;
    //         }
    //         grid[row][column] = counter;
    //         column += 1;
    //         counter += 1;
    //     }
    //     row += 1;
    //     column = 0;

    //     println!("{}", counter);
    // }

    // for row in grid.iter(){
    //     for val in row.iter(){
    //         print!("{}, ", val);
    //     }
    //     println!("");
    // }

    // let mut row = 0;
    // let mut column = 0;
    // 'row_loop: loop {
    //     if row >= grid.len() {
    //       break 'row_loop;
    //     }
    //     'column_loop: loop {
    //         if column >= grid[0].len(){
    //             break 'column_loop;
    //         }
    //         print!("{}, ", grid[row][column]);
    //         column += 1;
    //     }
    //     print!("\n");
    //     row += 1;
    //     column = 0;
    // }

    // 3.3 Return values from loops
    // let mut counter = 45;
    // let result = loop {
    //     if counter % 12 == 0 {
    //         break counter / 12;
    //     }
    //     if counter % 23 == 0 {
    //         break counter / 23
    //     }
    //     counter += 1;
    // };

    // println!("The result is {}", result);
    // println!("The counter is {}", counter);

    // 3.3 for loops
    // for number in 1..11 {
    //     println!(" {}", number);
    // }

    // 4.2 References and borrowing
    // let mut s = String::from("hello");

    // change(&mut s);

    // println!("{}", s);

    // // Multiple borrows
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    
    // let r2 = &mut s; // ERROR
    // println!("{}, {}", r1, r2);

    // // Scoped borrow
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // println!("{}", r1);

    // let r2 = &mut s; // Fine
    // println!("{}", r2);

    // 4.3 String slice
    let my_string = String::from("Hello world how are you?");
    let string_first_word = first_word(&my_string);
    println!("{}", string_first_word);
    let string_second_word = second_word(&my_string);
    println!("{}", string_second_word);

    let string_slice = "I am a 😻";
    let cat_slice = &string_slice[7..11]; // 😻
    //let bad_slice = &string_slice[7..8]; // PANICS!

    println!("{}", cat_slice); // Prints 😻

    // 4.3 Array slice
    let my_array = [6, 7, 8, 9, 10, 11, 12];
    let my_array_slice = &my_array[3..6];
    println!("{:?}", my_array_slice);
    for item in my_array {
        println!("{}", item);
    }

    println!("{:?}", my_array);


    let vec1 = vec!['p', 'a', 'r', 't', 'y', '🥳', '🎈'];
    let emoji_slice = &vec1[5..];
    println!("{:?}", emoji_slice);

    println!("{:#x}", 0xff);    // Prints 0xff

    let arr1 = ['p', 'a', 'r', 't', 'y', '🥳', '🎈'];
    for item in &vec1 {
        println!("{}", item);
    }
    println!("{:?}", vec1); // Fine

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    let mut second = false;
    let mut start = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if second {
                return &s[start..i];
            }
            start = i + 1;            
            second = true;
        }
    }

    &s[..]
}

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }
