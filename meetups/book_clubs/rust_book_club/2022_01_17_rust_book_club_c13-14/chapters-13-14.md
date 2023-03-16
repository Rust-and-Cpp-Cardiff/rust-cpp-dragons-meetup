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
- Week 2: Basic Programming Concepts & Understanding Ownership
- Week 3: Structs, Enums and Pattern Matching
- Week 4: Package, Crates and Modules, Vectors, Strings and Hashmaps
- Week 5: Error Handling & Generic Types, Traits, and Lifetimes
- Week 6: Writing Automated Tests & Building a Command Line Program

---

<!-- _class: lead-->

# **Rust Book**

Chapter 13

_Functional Language Features: Iterators and Closures_

---

13\. Functional Language Features: Iterators and Closures
-

- 13.1 Closures: Anonymous Functions that Can Capture Their Environment
- 13.2 Processing a Series of Items with Iterators
- 13.3 Improving Our I/O Project
- 13.4 Comparing Performance: Loops vs. Iterators

---
<!-- _class: invert-->

### Closures

- AKA
  - anonymous functions
  - function literals
  - lambda functions
  - lambda expressions

---

### What we'll cover

- What are closures
- Syntax
  - Parameters
  - Captures
- Capture lifetimes/closure traits
  - `Fn`, `FnMut`, `FnOnce`

---
<!-- _class: invert-->

### Closures

- Unnamed functions which can be assigned to variables and passed around as parameters
- Can capture state

---

### Sytax in Rust

##### Functions vs Closures

```rust
  fn function(i: i32) -> i32 { i + 1 }
```
```rust
  let closure           = |i     |          i + 1  ;
  let closure_annotated = |i: i32| -> i32 { i + 1 };
  // Type Annotations and `{}` are optional
  // These nameless functions are assigned to appropriately named variables.
```
```rust
  // Closure can also take no parameters
  let one = || 1;
```
---

### Sytax in Rust

##### Functions vs Closures

Calling code is identical
```rust
  let i = 1;
  // Call the function and closures.
  println!("function: {}", function(i));
  println!("closure: {}", closure(i));
  println!("closure_annotated: {}", closure_annotated(i));
  println!("closure returning one: {}", one());
```
The types are inferred by the compiler from the first use of the closure. All subsequent uses must use the same types

---
<!-- _class: invert-->

### Functions vs Closures - Captures

What makes closures different, along with convenience, is their ability to capture data

```rust
    let mut count = 0;
    println!("{}", count);
    // This closure takes `&count`
    let print_count = || {
        println!("{}", count);
    };
    
    print_count(); // Print count using closure
```

---
<!-- _class: invert-->

#### Captured Reference Lifetimes

- The captured reference lives until the last lambda use
- Usual reference rules apply - at any one time:
  - Many immutable references
  - One mutable reference

```rust
    let mut count = 0;
    println!("{}", count);
    // This closure takes `&count`
    let print_count = || {
        println!("{}", count);
    };
    
    println!("{}", count);
    // count = 6; // ERROR: Cannot assign to as already borrowed and borrow used later             
    print_count(); // Print count using closure
```


---
<!-- _class: invert-->

### Captured Reference Lifetimes

- Similarly, if count is modified then the reference taken will be mutable


```rust
    let mut count = 0;
    // This closure takes `&mut count`
    let mut inc = || {
        count += 1;
    };
    // let _count2: &i32 = &count; // ERROR Cannot borrow. 'count' already borrowed as mut&
    //                             // and mutable borrow used later

    inc(); // Increment count on mut&
```

---

### Captured Variables - by value

- For loop takes ownership of `vec!` so closure must capture by value
- Since `vec!` does not implement copy trait, ownership is moved
```rust
let data = vec![1, 2, 3];
let print_vec = || {
    for val in data {
        println!("Captured {:?}", val);
    }
};

print_vec();
// println!("Data still available? {:?}", data); // ERROR variable moved into closure 
                                                 // Vec<i32> doesn't implement copy
```

---

### Captured Variables - by value

This also means it can only be used once. It consumes the vector on its first call

```rust
let data = vec![1, 2, 3];
let print_vec = || {
    for val in data {
        println!("Captured {:?}", val);
    }
};
print_vec();
print_vec(); // ERROR closure cannot be invoked more than once because it moves 
             // the variable `data` out of its environment

             // note: this value implements `FnOnce`, which causes it to be moved 
             // when called
```

---
### Captured Variables - by value

We could have avoided this by value capture by changing the loop to a by reference loop

```rust
let data = vec![1, 2, 3];
let print_vec = || {
    for val in &data {
        println!("Captured {:?}", val);
    }
};

print_vec();
println!("Data still available? {:?}", data); // Fine
print_vec(); // Fine
```

---
### Captured Variables - by value

What if we **want** to move the data into the function?

e.g. for an action on a different thread

```rust
  let data = vec![1, 2, 3];

  std::thread::spawn(|| { // ERROR closure may outlive the current function, but it
                          // borrows `data`, which is owned by the current function
      for val in &data {
          println!("Captured {:?} by value", val);
      }
  });
```
---

#### Using 'move' keyword

Use `move` keyword when closure should take ownership of data e.g.

```rust
  let data = vec![1, 2, 3];

  std::thread::spawn(move || {
      for val in &data {
          println!("Captured {:?} by value", val);
      }
  });

  println!("Data still available? {:?}", data); // ERROR data was moved to the closure, 
                                                // so we cannot use it here
```

---
<!-- _class: invert-->

### Captured Variables - by value

On copyable types, a move is just a copy e.g. array of i32

```rust
let data = [1, 2, 3];

std::thread::spawn(move || {
    for val in &data {
        println!("Captured {:?} by value", val);
        thread::sleep(Duration::from_millis(1));
    }
});

// data was copied to the spawned thread so original data still available
println!("{:?}", data);
```

---

### So what are closures?

- Closures are essentially syntactic sugar for a special struct
- This struct can implement the following traits:
  - `Fn`
  - `FnMut`
  - `FnOnce`
- The compiler will check certain conditions to determine which of these traits to implement

---
### Closure Traits

  - `Fn` - closures which only take immutable references to captured variables (or no captures)
  - `FnMut` - closures which take mutable references to captured variables
  - `FnOnce` - closures that might consume captured variables
---

### Closure Traits

- All closures implement `FnOnce` because they can all be called at least once
- Closures that **don’t** move the captured variables also implement `FnMut`
- Closures that don’t need mutable access to the captured variables also implement `Fn`

---
<!-- _class: invert-->

### Passing Closures as Parameters

Closures can be passed as arguments by specifying a generic type parameter which implements `FnOnce`, `Fn` or `FnMut`
```rust
fn apply<F>(f: F) where
    F: FnOnce() {       // The closure takes no input and returns nothing.

    f();
}
```

```rust
fn apply_to_3<F>(f: F) -> i32 where  // Function takes a closure and returns an `i32`.
    F: Fn(i32) -> i32 {              // The closure takes an `i32` and returns an `i32`.

    f(3)
}
```

---
<!-- _class: invert-->

### Passing Closures as Parameters

We've used functions like these in previous chapters e.g. in the command line project in chapter 11


```rust
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
```


---

### Closure with iterators

Closures make operating on collections much cleaner

```rust
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`.
    println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
```
The `any` function on the trait `Iterator` returns true if any items in the collection satisfy our predicate (e.g. even)

---
### Closure with iterators

The `any` function on the trait `Iterator` returns true if any items in the collection satisfy our predicate (e.g. even)
```rust
    fn any<F>(&mut self, f: F) -> bool where
        F: FnMut(Self::Item) -> bool {}
```

- Closure
  - Parameter: an item from the collection
  - Return: a bool
  - Restrictions: closure must not consume its captures


---
<!-- _class: invert-->

### 13.2 Processing a Series of Items with Iterators

There are different types of iterator
- `iter()` - iterates over &T.
- `iter_mut()` - iterates over &mut T.
- `into_iter()` - which iterates over T.

To use these on a custom collection all you have to implement is `next` method for the `Iterator` trait, and associate a type Item. Usually this would be done on a separate type MyCollectionIter

---
<!-- _class: invert-->

### Implementing Iterators

Create your iterator struct, which implements `next`, has an `Item` associated type, and carries any state information needed

```rust
struct MyIntCollectionIter<'a> {
    collection: &'a MyIntCollection,
    index: usize
}
impl<'a> Iterator for MyIntCollectionIter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.collection.count() {
            return None
        }
        self.index += 1;
        return Some(self.collection.at(self.index - 1))                                        
    }
}
```

---
<!-- _class: invert-->

### Implementing Iterators

To use the iterator, put a function on your collection which returns one

```rust
impl MyIntCollection {
    fn iter(&self) -> MyIntCollectionIter {
        MyIntCollectionIter{collection: self, index: 0}                                   
    }
    ...
}
```
```rust
for val in my_int_coll.iter() {
  ...
}
```
---
<!-- _class: invert-->

### Implementing Iterators

To avoid having to call `iter()` explicitly, we can implement the `into_iter` trait on the collection itself

```rust
impl<'a> IntoIterator for &'a MyIntCollection {
    type Item = i32;
    type IntoIter = MyIntCollectionIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
```
---
<!-- _class: invert-->

### Implementing Iterators

So now we can simply write

```rust
for item in &collection {
    print!("{}, ", item)
}
```
Now we can use any standard `iterator` methods too

```rust
let my_sum: i32 = MyCollection::new().iter()
    .zip(MyCollection::new().iter().skip(1))                                     
    .map(|(a, b)| a * b)
    .filter(|x| x % 3 == 0)
    .sum();
```

---
<!-- _class: invert-->

### 13.2 Processing a Series of Items with Iterators

This chaining of methods on the Iterator trait works because `zip`, `map` and `filter` return other iterators. These are called **Iterator Adapters**

These are **lazily evaluated**, so nothing will happen until the results are collected with e.g. `collect()` or `sum()`

```rust
    let v1: Vec<i32> = vec![1, 2, 3];
    v1.iter().map(|x| x + 1); // Does nothing                                             

    let v2: Vec<i32> = vec![1, 2, 3];
    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect(); // Does work
    assert_eq!(v3, vec![2, 3, 4]);
```

---

#### 13.3 Improving our I/O Projects - Avoid clone by iterating over args directly

```rust
pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
    args.next();

    let query = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a query string"),
    };

    let filename = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a file name"),
    };
```

---

#### 13.3 Improving our I/O Projects - Collect matching lines with iterator operations - Before vs After

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {                             
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
```

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {                         
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

---

<!-- _class: invert-->

### Chapter 13 Summary

- Closures and iterators are **very rust** - use them to express your intent 💁‍♂️
- Raw loops are passé 🙅‍♀️
- The borrow checker will save you from yourself with captures
- The implementations of closures and iterators are such that runtime performance is not affected 🚀 (zero-cost abstractions 💖)


---

<!-- _class: lead-->

# **Rust Book**

Chapter 14
_More About Cargo and Crates.io_


---

14\. More About Cargo and Crates.io
-
- 14.1 Customizing Builds with Release Profiles
- 14.2 Publishing a Crate to Crates.io
- 14.3 Cargo Workspaces
- 14.4 Installing Binaries from Crates.io with `cargo install`
- 14.5 Extending Cargo with Custom Commands

---

- **Release Profiles** allow you to set compiler settings:

  - Optimization level
  - Amount of debug info
  - Debug assertions
  - Overflow checks
  - Link time optimizations
  - panic unwind or abort
  - Allow incremental compilation
  - Number of code gen units
  - Runtime path

---
### Release Profiles

Cargo comes with 4 built-in profiles: `dev`, `release`, `test`, and `bench`

```toml
[profile.dev]
opt-level = 0
debug = true
split-debuginfo = '...'  # Platform-specific.
debug-assertions = true
overflow-checks = true
lto = false
panic = 'unwind'
incremental = true
codegen-units = 256
rpath = false
```

---
### Release Profiles

Cargo comes with 4 built-in profiles: `dev`, `release`, `test`, and `bench`

```toml
[profile.release]
opt-level = 3
debug = false
split-debuginfo = '...'  # Platform-specific.
debug-assertions = false
overflow-checks = false
lto = false
panic = 'unwind'
incremental = false
codegen-units = 16
rpath = false
```

---
### Release Profiles

Cargo comes with 4 built-in profiles: `dev`, `release`, `test`, and `bench`

- `dev` has fewer optimizations and more checks
- `release` has more optimizations, and fewer checks
- `test` inherits from `dev`
- `bench` inherits from `release`

Cargo will select the appropriate profile depending on the command being run (i.e. run release, debug, test or bench)

---
### Release Profiles

- These profile settings will use default values unless set in the `cargo.toml` 
- Custom profiles can also be create in `cargo.toml`

```toml
[profile.release-lto]
inherits = "release"
lto = true
```

- The --profile flag can be used to choose this custom profile:

```sh
cargo build --profile release-lto
```

---
### Optimization levels

```
0: no optimizations
1: basic optimizations
2: some optimizations
3: all optimizations
"s": optimize for binary size
"z": optimize for binary size, but also turn off loop vectorization.
```

###### Can even take it further with profile guided optimizations https://doc.rust-lang.org/rustc/profile-guided-optimization.html (leverages llvm tooling, gathering data from a typical runtime use and passes that into cargo to inform what compiler optimizations should be used)

---
<!-- _class: invert-->

### 14.2 Publishing Crates: Doc

Triple slash comments will be converted into documentation when the crate is used or published

```rust
/// Writing some nice triple slash doc. These can contain **examples**:
/// ```rust
/// let query = "the".to_owned();
/// let filename = "poem.txt".to_owned();
/// let case_sensitive = true;
/// let config = command_line_program::Config {query, filename, case_sensitive};
/// command_line_program::run(config);
/// ```
/// Using markdown formatting.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    ...
```

---
<!-- _class: invert-->

Anyone using the library or reading the doc on crates.io will see this comment

![](images/rust-triple-slash-doc2.png)

---
<!-- _class: invert-->

#### 14.2 Publishing Crates: Doc

View the doc with `cargo doc --open`

![w:1000](images/rust-triple-slash-html.png)

---
<!-- _class: invert-->

#### 14.2 Publishing Crates: Doc
 
The doc will also show you any re-exports which make items needed by our crate accessible from our crate directly

```rust
// Inside art crate
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;
```

```rust
// Calling code
use art::mix;
use art::PrimaryColor;
```

---
#### 14.2 Publishing Crates: crates.io

- Key steps to publishing are
  - Create your account and retrieve API key
  - Login from command line to store API key on machine
  - Add metadata in your `cargo.toml` such as license, edition etc.
  - Then just `cargo publish`!

---
<!-- _class: invert-->

### 14.3 Cargo Workspaces

A set of packages that share the same Cargo.lock and output directory

#### Why?

-  When working on multiple crates same time, they are likely to have shared dependencies
- Workspaces allow you to use a shared build directory under the workspace root to prevent downloading and compiling the same dependency multiple times

---
### Cargo Workspaces

- Packages in a workspace will:
  - share a common Cargo.lock file which resides in the workspace root.
  - share a common output directory, which defaults to a directory named target in the workspace root.

The `[patch]`, `[replace]` and `[profile.*]` sections in Cargo.toml are only recognized in the root manifest, and ignored in member crates' manifests.

---
### Cargo Workspaces

- Workflow
  - Create directory
  - Create `cargo.toml` with workspace section containing members list

  ```toml
  [workspace]
  members = [
      "adder",                                                                          
  ]
  ```

  - Create or move member crates inside workspace directory
  - `cargo build` workspace to create output and `cargo.lock`

---
### Cargo Workspaces

Resulting folder structure:

```txt
├── Cargo.lock
├── Cargo.toml
├── my-binary-crate
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

By sharing one target directory, the crates can avoid unnecessary rebuilding of dependencies

---
### Cargo Workspaces

- Building crates inside a workspace will always output to workspace output directory
- To reference once crate from another we need to explicitly describe dependency as normal

  ```toml
  [dependencies]
  other-lib = { path = "../other-lib" }
  ```

---
### Cargo Workspaces

- Build and run
  - Build entire workspace with `cargo build`
  - Run specific binary crate within `cargo run -p my-app`

```txt
├── Cargo.lock
├── Cargo.toml
├── add-helper
│   ├── Cargo.toml                                                                         
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

---

### Cargo Workspaces

- Dependencies
  - Crates must reference dependencies from _their own_ `cargo.toml`
  - When we build the workspace the crate will be added to the workspace `Cargo.lock`
  - If another crate wants to use the dependency, it will also need to add it to its own `cargo.toml`
  - All crates will use the **same crate version** from the workspace `Cargo.lock`

---

### **Crate version caveat**

> Cargo only unifies semver-compatible versions. So if one package specifies 1.0 and another specifies 1.1, they will both use the most recent 1.x.y version. However, a third package specifies 2.0, then it will use the most recent 2.x.y version without affecting the other two.

### **Note about profiles**

Workspace profiles trump the inner crate profiles in `cargo.toml`

---
<!-- _class: invert-->
#### 14.4 `cargo install` binaries

- Allows you to install and use binary crates locally
- Can only be used to pull down crates with binary executables from crates.io

#### 14.5 cargo subcommands

- If a binary in your `$PATH` is named `cargo-something`, you can run it as if it was a Cargo subcommand by running `cargo something`
- It will also show up when you call `cargo --list`

---

<!-- _class: invert-->

## Chapter 14 Summary

### **Dang Cargo how you do so much**

- Play around with profiles to your hearts content
- Publishing is an absolute doddle
- Workspaces are great - letting your crates work together with ease
- Libraries are fun and all, but don't forget about the lowly binary - there are plenty of excellent ones on crates.io!
- The flexibility of cargo goes on (`cargo do-the-dishes`)

---

<!-- _class: lead -->
# End of chapter 14


---
<!-- _class: invert -->

# What next?

- **Chapter 15: Smart Pointers**
- _Chapter 16: Fearless Concurrency_
- _Chapter 17: Object Oriented Programming Features of Rust_
- _Chapter 18: Patterns and Matching_

---

<!-- _class: lead -->

### Dates coming up

3rd February - Smart Pointers

10th February - Speaker style meetup

17th February - Fearless Concurency
