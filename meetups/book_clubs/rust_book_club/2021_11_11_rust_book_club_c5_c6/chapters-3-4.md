---
marp: true
theme: gaia
paginate: true
math: katex
---

![bg](images/wallpaper2020.png)

---
<!-- _class: lead invert-->


## twitter: [@rustcpp_cardiff](https://twitter.com/rustcpp_cardiff)
## email: [rustandcppcardiff@gmail.com](rustandcppcadiff@gmail.com)
## discord: [https://discorg.gg/DGSpkYP](https://discorg.gg/DGSpkYP)


---

<!-- _class: lead invert-->

# Objectives

Learn and share
Introduce new people
Challenge ourselves
Real world solutions
Advance our ability

---

![bg](images/support3.png)

---
<!-- _class: invert-->

# Conduct

- Privacy
- [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct)
  - Just be wonderful
- [Pacman rule](https://www.ericholscher.com/blog/2017/aug/2/pacman-rule-conferences/)

---

<!-- _class: lead invert-->

![](images/pacman.png)


---

![bg](images/RustBookClubLogo.jpg)

---

<!-- _class: lead-->

# **Rust Book**

Recap on last week
_Setup_

---
<!-- _class: invert -->

#### Recap

- Week 1: Installed rust toolchain
  - Instructions: https://www.rust-lang.org/tools/install
  - Chapters 1 & 2 of the book (Hello, world & Hello, cargo)

- Week 2: Started getting into the language
  - Basic Programming Concepts
    - Types, control flow, mutability
  - Understanding Ownership
    - Stack vs heap, copy trait, drop trait, moving ownership


---

<!-- _class: lead-->

# **Rust Book**

Chapter 5
_Using Structs to Structure Related Data_

---

5\. Using Structs to Structure Related Data
- 

- 5.1. Defining and Instantiating Structs
- 5.2. An Example Program Using Structs
- 5.3. Method Syntax


---
<!-- _class: invert-->

# 5.1 Defining and Instantiating Structs

### Struct vs Tuple
 - Named data members (fields)
 - Don't rely on order
 - Instantiated with the struct keyword

---
<!-- _class: invert-->

### Defining a struct type and creating a instance

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

```rust
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
```
---


### Shorthand notation when variables and fields have same name

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```
- Instead or `username: username` you can just put `username` since variable and field names match

---
<!-- _class: invert-->

## Creating a new user from an existing user

```rust
    let user1 = User {
        active: true,
        username: String::from("Toph"),
        email: String::from("Toph@avatar.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        ..user1
    };
```
- Just like assignment, this invalidates the `user1` reference because `String` doesn't implement copy trait

---
### Assign struct members vs assign struct

- References to `user1` only still valid if all fields used in User type implement copy trait
```rust
    let user2 = User {
        ..user1
    };
```
- References to `user1` only still valid if User type implements Copy trait (only possible if all fields in User type implement copy trait)
```rust
    let user2 = user1;
```
---
<!-- _class: gaia-->

```rust
    let user1 = User {
        active: true,
        username: String::from("Toph"),
        email: String::from("Toph@avatar.com"),
    };

    let user2 = User {
        username: String::from("Iroh"),
        ..user1
    };

    println!("{}", user1.username);
    println!("{}", user1.active);

    print_user_active(&user2);
    print_user_active(&user1);
}

fn print_user_active(user: &User) {
    println!("{}", user.active);
}
```

---
<!-- _class: gaia-->

## Error

```rust
error[E0382]: borrow of partially moved value: `user1`
  --> src\main.rs:26:23
   |
17 |       let user2 = User {
   |  _________________-
18 | |         username: String::from("Ciara"),
19 | |         ..user1
20 | |     };
   | |_____- value partially moved here
...
26 |       print_user_active(&user1);
   |                         ^^^^^^ value borrowed here after partial move
   |
   = note: partial move occurs because `user1.email` has type `String`, which does not implement the `Copy` trait
```
- Once you've partially moved from a struct, you can no longer borrow it

---
<!-- _class: gaia-->

#### You CAN still borrow the valid parts

```rust
    let user1 = User {
        active: true,
        username: String::from("Ciara"),
        email: String::from("Ciara@rustbookclub.com"),
    };

    let user2 = User {
        username: String::from("Ciara"),
        ..user1
    };

    print_user_active(&user2.active);
    print_user_active(&user1.active);
}

fn print_user_active(user_active: &bool) { // Can't borrow the full user, but can borrow user.active
    println!("{}", user_active);
}
```

---
<!-- _class: invert -->

## Tuple structs

- No names associated with their fields

```rust
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
```
- Strict type, unlike a tuple `Color` and `Point` below cannot be used interchangeably
---
<!-- _class: invert -->
## Unit-like structs

- No data
- e.g. here AlwaysEqual has no data, so every instance is equal to every other instance

```rust
    struct AlwaysEqual;

    let subject = AlwaysEqual;
```
- Unit-like structs may still have traits implemented on them
---


## 5.2 An Example Program Using Structs

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", area(&rect1));
}
fn area(rectangle: &Rectangle) -> u32 { // By reference so caller maintains ownership
    rectangle.width * rectangle.height
}
```
---

<!-- _class: invert -->
## Implementing traits e.g. `Debug`

```rust
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1);
}
```

- Print syntax `{:?}` relies on the Debug trait being implemented

---

<!-- _class: invert -->
## Implementing traits e.g. `Debug`

- Can either implement manually
- Or derive it by adding `#[derive(Debug)]` attribute before struct definition

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```
---

<!-- _class: invert -->
## Implementing traits e.g. `Debug`

- Allows you to debug print using `{:?}`

```rust
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1); // rect1 is Rectangle { width: 30, height: 50 }
```
- Or debug pretty-print `{:#?}`
```rust
    println!("rect1 is {:#?}", rect1)   // rect1 is Rectangle {                                    
                                        //     width: 30,
                                        //     height: 50,
                                        // }
```
---
<!-- _class: gaia -->

### What is derive doing?

- Procedural macro (Chapter 19)
- Operates on Rust Syntax and generates new code (not just find and replace)
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```
- expand macro with `rustc -Zunpretty=expanded src/main.rs` if using nightly rust compiler (instead of stable)
<!-- - Anyone fancy doing a talk on writing these? - https://blog.logrocket.com/procedural-macros-in-rust/ -->


---
<!-- _class: gaia -->

```rust
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Rectangle { width: ref __self_0_0, height: ref __self_0_1 } => {
                let debug_trait_builder =
                    &mut ::core::fmt::Formatter::debug_struct(f, "Rectangle");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder,
                                                    "width", &&(*__self_0_0));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder,
                                                    "height",
                                                    &&(*__self_0_1));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
```

---
<!-- _class: invert -->
## Implementing traits e.g. `Debug`

- As well as printing to console, you can use debug format to print to standard error console stream instead of standard output console stream

```rust
    let rect1 = Rectangle {
        width: dbg!(30 * 2), // Still works!
        height: 50,
    };
    dbg!(&rect1);
```
- Outputs the input value, so can be slipped in to help debugging




---
<!-- _class: -->

# 5.3 Method Syntax

- Similar to functions except
  - Defined within context of a struct
  - Take self as first parameter

---
<!-- _class: invert-->
### 5.3 Method Syntax

- Methods go in implementation block for our struct


```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

---
<!-- _class: invert-->
### 5.3 Method Syntax

`&self` is short for `self: &Self`

Within an impl block, the type `Self` is an alias for the type that the impl block is for
```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```
Similarly for `self` and `&mut self`

---
<!-- _class: invert-->
### 5.3 Method Syntax

The rust feature _automatic referencing and dereferencing_  allows us to write:
```rust
p1.distance(&p2);
```
instead of
```rust
(&p1).distance(&p2);
```
---

### Associated functions
- Alongside methods, you can have associated functions in the impl which don't take self e.g.
```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```
- Creates a square rectangle with `let sq = Rectangle::square(3);`

---
<!-- _class: invert-->
## Struct Summary
- Groups related data
- Can implement traits
  - Some can be automatically dervied with attributes e.g. `#[derive(Debug)]`
- Behavior can be defined in an `impl` block
   - Methods which take `self (instance)
   - Other Associated functions (static)


---

<!-- _class: lead-->

# **Rust Book**

Chapter 6
_Enums and Pattern Matching_


---

6\. Enums and Pattern Matching
-
- 6.1. Defining an Enum
- 6.2. The match Control Flow Operator
- 6.3. Concise Control Flow with if let

---

<!-- _class: invert-->

## 6.1 Defining an Enum

- Can store just enum values
```rust
enum IpAddrKind {
    V4,
    V6,
}
```
- Or values with additional data
```rust
enum IpAddr {
    V4(String),
    V6(String),
}
```

---
<!-- _class: invert-->
## 6.1 Defining an Enum

- Each enum variant can store different amounts and types of daya e.g.
```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
```
---
<!-- _class: invert-->
## Defining methods on enums
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```
---
## `Option` enum

```rust
enum Option<T> {
    None,
    Some(T),
}
```
- Where T is the generic type parameter to allow any type to be used e.g.
```rust
let some_number = Some(5);
let some_string = Some("a string");     // Type inferred

let absent_number: Option<i32> = None;  // Type annotated
```
---
<!-- _class: invert -->
### `Option` enum methods

- Check the values
```rust
let x: Option<u32> = Some(2);
assert_eq!(x.is_some(), true);
assert_eq!(x.in_none(), false);
```
- Use `expects` function to error is value not set
```rust
let name: Option<&str> = None;
x.expect("Expected a name. Cannot continue"); // panics with error
```


---
<!-- _class: invert -->
### `Option` enum methods
- Use `unwrap_or` to use fallback value if `None` set
```rust
assert_eq!(Some("car").unwrap_or("bike"), "car");
assert_eq!(None.unwrap_or("bike"), "bike");
```
- Use `zip` to combine optionals
```rust
let x = Some(1);
let y = Some("hi");
let z = None::<u8>;

assert_eq!(x.zip(y), Some((1, "hi")));
assert_eq!(x.zip(z), None);
```
---
<!-- _class: gaia -->

### `Option` enum methods
- Use `as_ref` to cast `Option<String>` to `Option<&String>` manipulate without consuming
```rust
let text: Option<String> = Some("Hello, world!".to_string());
let text_length: Option<usize> = text.as_ref().map(|s| s.len()); // By getting `Option<&String>`
println!("still can print text: {:?}", text);                    // text is still usable
```
- Use `as_mut` to convert from `&mut Option<T>` to `Option<&mut T>`.
```rust
let mut x = Some(2);                                                                          
match x.as_mut() {
    Some(v) => *v = 42,
    None => {},
}
```
---

# 6.2 The `match` Control Flow Operator

- Allows you to compare a value against a series of patterns and then execute code based on which pattern matches
- Patterns can be made up of literal values, variable names, wildcards, and many other things
- Matches in Rust are exhaustive

---
<!-- _class: invert -->

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

---
<!-- _class: invert -->

### 6.2 The `match` Control Flow Operator

- If a pattern (e.g. `Coin::Penny`) matches the value, then the code for that pattern is executed
- If pattern is not matched it continues to next arm

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

---

### 6.2 The `match` Control Flow Operator
- At first glance looks similar to C++ switch statement, but infact much more powerful
  - C++ switch only works with integral values i.e. int, char and enum
  - C++ switch can only be used for checking equality
  - C++ switch cannot be used as an expression

---
<!-- _class: invert -->

### Can match on a range of values

```rust
fn main() {
    let x = 1;
    let message = match x {
        0 | 1 => "not many",  // check if matches either to 0 or 1
        2..=9 => "a few",     // check if its in a range of `[2, 9]`
        10..15 => "too many", // check if its in a range of `[10, 15)`
        // else clause, if neither of the cases matches then "lots of
        // many" will be assigned
        _ => "lots of many",
    };

    assert_eq!(message, "a few");
}
```
---
<!-- _class: invert -->

### Can bind to parts of the matched value

- For example, if our enum has associated data
```rust
#[derive(Debug)] 
enum UsState {
    Alabama,
    Alaska,
    // etc.
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```
---
<!-- _class: invert -->

### Can bind to parts of the matched value
- We can get at this data in the match
```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```
---
## Particularly useful with `Option<T>` type

- Match none or some, and bind to the associated data 

```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
```
---
<!-- _class: invert -->

## Making matches exhaustive

- Compiler will complain if not all possible patterns are satisfied by match
- We can make our match ehaustive with catch-all and placeholder match arms (`_`) 

---
<!-- _class: invert -->

Named catch-all if you want to use the value
```rust
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
```
or placeholder (`_`) if you don't
```rust
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
```


---

# 6.3 Concise Control Flow with `if let`

- Works like a match that matches one pattern and then ignores all other values
```rust
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
```
- Removes a bit of boiler plate code for single match use cases
- Can also have with an `else` statement

---


<!-- _class: lead -->
# End of chapter 6


---
<!-- _class: invert -->

# What next?

- **Chapter 7: Managing Growing Projects with Packages, Crates, and Modules**
- **Chapter 8: Common Collections**
- _Chapter 9: Error Handling_?
- _Chapter 10: Generic Types, Traits, and Lifetimes_?