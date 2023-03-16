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

![bg](images/RustBookClubLogo.png)

---

<!-- _class: lead-->

# **Rust Book**

Recap on last week
_Structs, Enums and Pattern Matching_

---
<!-- _class: invert -->

#### Recap

- Week 1: Installed rust toolchain & Hello, world
  - https://www.rust-lang.org/tools/install

- Week 2: Basic Programming Concepts & Understanding Ownership
    - Types, control flow, mutability
    - Stack vs heap, copy trait, drop trait, moving ownership
- Week 3: Structs, Enums and Pattern Matching
    - Methods, traits, attaching data to enum values
    - Match and if let statements

---

<!-- _class: lead-->

# **Rust Book**

Chapter 7

_Managing Growing Projects 
with Packages, Crates, and Modules_

---

7\. Managing Growing Projects with Packages, Crates, and Modules
-
 

- 7.1 Packages and Crates
- 7.2 Defining Modules to Control Scope and Privacy
- 7.3 Paths for Referring to an Item in the Module Tree
- 7.4 Bringing Paths into Scope with the use Keyword
- 7.5 Separating Modules into Different Files



---
<!-- _class: invert-->

## 7.1 Packages and Crates

- Crates
   - A crate is a binary or library
   - The _crate root_ is a source file that the Rust compiler starts from and makes up the root module of your crate
- Packages
   - A package is _one or more crates_ that provide a set of functionality
   - A package contains a **Cargo.toml** file that describes how to build those crates.

---

## 7.1 Packages and Crates

### Calling `Cargo new <name>` 

  - Creates a package containing a **binary crate**
  - This will contain the file `src/main.rs`
  - Cargo will recognize this as the root binary crate with the same name as the package by default - so no need to add to Cargo.toml
  - Could also have called `Cargo new --bin <name>`

---

## 7.1 Packages and Crates

### Calling `Cargo new --lib <name>`

  - Creates a package containing a **library crate**
  - This will contain the file `src/lib.rs`
  - Cargo will recognize `src/lib.rs` as the root library create with the same name as the package by default - so no need to add to Cargo.toml


---
<!-- _class: invert-->

## 7.1 Packages and Crates

- A package can only contain **one library crate**
- A package can have **multiple binary crates** by placing additional files in the src/bin directory: each file will be a separate binary crate
- A crate will group related functionality together in a scope


---

## 7.2 Defining Modules to Control Scope and Privacy


- Use modules to group related definitions together and name why they’re related

---
<!-- _class: invert-->
#### 7.2 Defining Modules to Control Scope and Privacy

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```
---
<!-- _class: invert-->

#### 7.2 Defining Modules to Control Scope and Privacy

These modules are rooted under the implicit module node `crate`
```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```
---
### 7.3 Paths for Referring to an Item in the Module Tree

- Paths are absolute, or relative
```rust
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path (if defined in same crate as front_of_house)
    front_of_house::hosting::add_to_waitlist();
}
```
---
<!-- _class: invert-->
### 7.3 Paths for Referring to an Item in the Module Tree


  - An absolute path starts from a crate root by using a crate name or a literal crate
  - A relative path starts from the current module and uses self, super, or an identifier in the current module


---
### Public vs Private

##### Modules define the _privacy boundaries_

- All items in Rust (functions, methods, structs, enums, modules, and constants) are private by default
- Items in a parent module can’t use the private items inside child modules
- Items in child modules can use the items in their _ancestor_ modules
- Use `pub` keyword to make an item public

---
#### `pub` keyword

##### Pub keyword does not propogate to its children

```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {} // Not public
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist(); // ERROR                      

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```
---
#### `pub` keyword

##### Need to explicitly mark item `pub` to expose to ancestors

```rust
mod front_of_house {
    pub mod hosting { // Must be public to access from outside 'front_of_house'
        pub fn add_to_waitlist() {} // Must be public to be accessed from outside 'hosting'
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist(); // Fine since both hosting and 
}                                               // add_to_waitlist are public
```

---

#### `pub` and `super` keywords

- Without the `pub` keyword, only siblings and ancestors are accessible
- You can also use the `super` keyword to access relative ancestors (simlar to `..` for files) instead of hard-coding absolute
```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // Goes up a level, avoids need to use absolute path
    }
    fn cook_order() {}
}
```
---
### `pub` keyword - Structs and Enums

- Marking an struct public does **not** make its fields public
- Marking an enum public **does** expose all of its variants

---
### `pub` keyword - Structs

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn order(toast: &str, seasonal_fruit: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from(seasonal_fruit),
            }
        }
    }
}
```

---
### `pub` keyword - Structs

- You can call public functions which set private fields, but can't access the private fields directly

```rust
pub fn eat_at_restaurant() {
    // Order a breakfast of Rye toast with peaches
    let mut meal = back_of_house::Breakfast::order("Rye", "Peaches");

    meal.toast = String::from("Wheat"); // Fine - toast is public
    meal.seasonal_fruit = String::from("blueberries"); // ERROR
}

```
---

### `pub` keyword - Enums


```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```
- Only need to mark enum `pub` at top level

---


<!-- _class: invert-->
### 7.4 Bringing Paths into Scope with the `use` Keyword


```rust
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```
- Adding use crate::front_of_house::hosting makes hosting is now a valid name in that scope
- Similar to creating a symbolic link in the filesystem

---

###### Bringing parent path into scope rather than function directly makes it clear item is not defined locally - i.e.


```rust
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```
###### Preferred over
```rust
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
```

---

###### For structs an enums however, it is convention to include full path 


```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
}
```
###### More common than
```rust
use std::collections;

fn main() {
    let mut map = collections::HashMap::new();
}
```
###### Unless it introduces a name clash

---
#### 7.4 Bringing Paths into Scope with the `use` Keyword

- We can also alias paths to avoid name clashes or add clarity
```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```
---

#### 7.4 Bringing Paths into Scope with the `use` Keyword

- And we can re-export paths with `pub` keyword to avoid calling code needing extra use statements
```rust
pub mod front_of_house {
    pub enum Course {
        Starter, Main, Dessert
    }
}

mod back_of_house {
    pub use crate::front_of_house::Course;                                              
}

fn main() {
  let course = back_of_house::Course::Starter;
}
```
---
### 7.4 Bringing Paths into Scope with the `use` Keyword

To bring in external packages, we need to add them to our `Cargo.toml` to allow us to use them

```rust
[dependencies]
rand = "0.9.0"
```
This tells Cargo to download the rand package and any dependencies from crates.io and make rand available to our project. Then we can use it - e.g.
```rust
use rand::Rng;
```
---
### 7.4 Bringing Paths into Scope with the `use` Keyword

The standard library (std) is also a crate that’s external to our package, but comes with the rust compiler so no need to add to out `Cargo.toml`

---
### 7.4 Bringing Paths into Scope with the `use` Keyword

To import multiple items from the same crate or module you can use nested use statements
```rust
use std::{cmp::Ordering, io};               
```

---
### 7.4 Bringing Paths into Scope with the `use` Keyword


You can also do this when one is a submodule of the other using the self keyword e.g.:
```rust
use std::io;
use std::io::Write;

// Becomes:
use std::io::{self, Write};                                                                  
```
Or use the glob (*) operator to pull everything in

```rust
use std::collections::*;     // Use with caution...
```
---
<!-- _class: invert-->
## 7.5 Separating Modules into Different Files

You can split a module out into separate files using the `mod` keyword to declare your module where you want it, and putting its body in a separate file of the same name e.g.

---
#### 7.5 Separating Modules into Different Files

###### Filename: src/lib.rs
```rust
mod front_of_house; // Notice this doesn't need to be public
                    // It is still in the same crate even though defined in separate file
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```
###### Filename: src/front_of_house.rs

```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```
---
#### 7.5 Separating Modules into Different Files

- To break out the definition of `hosting` into its own file too, you would create a directory called `font_of_house` and add a file called `hosting.cs`

###### Filename: src/front_of_house.rs

```rust
pub mod hosting;
```
###### Filename: src/front_of_house/hosting.rs

```rust
pub fn add_to_waitlist() {}
```
---
<!-- _class: invert-->

### Chapter 7 Summary

- Split a package into multiple crates and a crate into modules 
- Refer to items defined in one module from another module with absolute or relative paths
- Bring paths into scope with `use` statements
- Module code is private by default, but you can make definitions public by adding the `pub` keyword


---

<!-- _class: lead-->

# **Rust Book**

Chapter 8
_Common Collections_


---

8\. Common Collections
-
- 8.1 Storing Lists of Values with Vectors
- 8.2 Storing UTF-8 Encoded Text with Strings
- 8.3 Storing Keys with Associated Values in Hash Maps

---

<!-- _class: invert-->

## 8.1 Storing Lists of Values with Vectors



---
<!-- _class: gaia-->

---

# 8.2 Storing UTF-8 Encoded Text with Strings

---

# 8.3 Storing Keys with Associated Values in Hash Maps


---


<!-- _class: lead -->
# End of chapter 8


---
<!-- _class: invert -->

# What next?

- **Chapter 9: Error Handling**
- **Chapter 10: Generic Types, Traits, and Lifetimes**
- _Chapter 11: Writing Automated Tests_?
- _Chapter 12: An I/O Project: Building a Command Line Program_?