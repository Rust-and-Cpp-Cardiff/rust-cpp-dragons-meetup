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
## discord (message on meetup for link)

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
<!-- _class: invert -->

#### Recap

- **Chapters 1- 6:** Installation, Essential Concepts, Ownership, Structs, Enums and Pattern Matching
- **Chapters 7 - 12:** Packages, Crates and Modules, Vectors, Strings and Hashmaps, Error Handling, Generic Types, Traits and Lifetimes, Writing Automated Tests, and Building a Command Line Program
- **Chapters 13 - 16:** Functional language features: iterators & closures, more about Cargo and Crates.io, Smart pointers, Fearless Concurrency
- **Chapters 17 - 18:** Objective oriented programming features of Rust, and Patterns and Matching

---

<!-- _class: lead-->

# **Rust Book**

Chapter 19

_Advanced Features_

---

## 19. Advanced Features

- 19.1 Unsafe Rust
- 19.2 Advanced traits
- 19.3 Advanced types
- 19.4 Advanced functions and closures
- 19.5 Macros

---
<!-- _class: lead -->

# Unsafe Rust

---
<!-- _class: invert -->

# Unsafe Rust

> ### Trust me, I know what I’m doing.

Unsafe keyword gives you access to programming features for which the compiler can't provide its usual safety guarantees.

---

# Unsafe Rust

Usual borrow checker rules apply, but in an `unsafe` block you can also:

- Dereference a raw pointer
- Call another unsafe function or method
- Access or modify a mutable static variable
- Implement an unsafe trait
- Access fields of unions

---
<!-- _class: invert -->

## Unsafe Rust - Raw Pointers

Unsafe rust allows you to dereference raw pointers:
- `*const T`
- `*mut T`

You can create instances of these in safe code, you just can't derefernce them unless you are in an `unsafe` block

---
<!-- _class: invert -->
## Unsafe Rust - Raw Pointers

Raw pointers are special because they:

- Can ignore reference borrowing rules:
  - So can have both immutable and mutable pointers, or multiple mutable pointers to the same location
- Aren’t guaranteed to point to valid memory
- Are allowed to be null
- Don’t implement any automatic cleanup

---
## Unsafe Rust - Raw Pointers

### Why?

- Performance
- Interfacing with other languages
- Interfacing with hardware

---
## Unsafe Rust - Raw Pointers

### How?

You can cast an immutable and a mutable reference into its corresponding raw pointer types

```rust
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}
```

---
## Unsafe Rust - Raw Pointers

In that case, the pointer is valid, but we can also create invalid ones which might go boom:

```rust
let address = 0x0usize;
let r = address as *const i32;

unsafe {
    println!("r is: {}", *r);
}
```
`error: process didn't exit successfully (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)`

---
<!-- _class: invert -->

## Unsafe Rust - Calling an Unsafe function

A function marked `unsafe` can only be called from a `unsafe` block.

#### Why?

Functions marked unsafe indicating the calling code needs to be careful.

Calling it using an `unsafe` block indicates to the compiler that we think we know how to use this function properly, and have convinced ourselves that we’re fulfilling the contract of the function.

---

## Unsafe Rust - Calling an Unsafe function

```rust
unsafe fn use_with_caution() {
    println!("Might be dangerous if used incorrectly... read the doc");
}

fn call_at_will_i_am_safe() {

    println!("No calling conditions required for safety...");

    unsafe { // I promise, I'm using this correctly
        use_with_caution();
    }
    
}
```

---
## Unsafe Rust - Calling an Unsafe function

Functions containing an `unsafe` block do not need to be called from within an unsafe block.

Many standard library functions have unsafe code e.g. `split_at_mut` which allows you to get two mutable slices in a tuple

Conceptually, the unsafe block says "I promise the two slices I return won't overlap"

---
## Unsafe Rust - Calling an Unsafe function

What we want to write, but can't:

```rust
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    (&mut values[..mid], &mut values[mid..])
}
```
***Note:** `assert!` fires in release and debug builds. This means our unsafe code is correct, but the borrow checker can't verify it*

---

```rs
error[E0499]: cannot borrow `*values` as mutable more than once at a time
 --> src/main.rs:6:31
  |
1 | fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  |                         - let's call the lifetime of this reference `'1`
...
6 |     (&mut values[..mid], &mut values[mid..])
  |     --------------------------^^^^^^--------
  |     |     |                   |
  |     |     |                   second mutable borrow occurs here
  |     |     first mutable borrow occurs here
  |     returning this value requires that `*values` is borrowed for `'1`

For more information about this error, try `rustc --explain E0499`.
error: could not compile `unsafe-example` due to previous error
```

---
## Unsafe Rust - Calling an Unsafe function

What we write instead:

```rust
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {             
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

---
## Unsafe Rust - Calling an Unsafe function

Why is this unsafe? The doc for `from_raw_parts_mut` will tell us! (or just `from_raw_parts` doc)

**_Unsafe functions often have the longest doc comments!!_**

---

### Unsafe Rust - from_raw_parts

#### The function itself

```rust
pub const unsafe fn from_raw_parts<'a, T>(data: *const T, len: usize) -> &'a [T] {
    debug_check_data_len(data, len);

    // SAFETY: the caller must uphold the safety contract for `from_raw_parts`.
    unsafe { &*ptr::slice_from_raw_parts(data, len) }
}
```

Dereferencing a raw ptr!!

---
### Unsafe Rust - from_raw_parts

#### What it does

```rust
/// Forms a slice from a pointer and a length.
///
/// The `len` argument is the number of **elements**, not the number of bytes.
```

#### Safety Conditions

```rust
/// # Safety
///
/// Behavior is undefined if any of the following conditions are violated:
```
---

Condition 1:

```rust
/// * `data` must be [valid] for reads for `len * mem::size_of::<T>()` many bytes,
///   and it must be properly aligned. This means in particular:
///
///     * The entire memory range of this slice must be contained within a single
///       allocated object!
///       Slices can never span across multiple allocated objects. 
///
///     * `data` must be non-null and aligned even for zero-length slices. One
///       reason for this is that enum layout optimizations may rely on references
///       (including slices of any length) being aligned and non-null to distinguish
///       them from other data. You can obtain a pointer that is usable as `data`
///       for zero-length slices using [`NonNull::dangling()`].

```

---
Condition 2

```rust
/// * `data` must point to `len` consecutive properly initialized values of type `T`.
```

Condition 3

```rust
/// * The memory referenced by the returned slice must not be mutated for the duration
///   of lifetime `'a`, except inside an `UnsafeCell`.
```

Condition 4

```rust
/// * The total size `len * mem::size_of::<T>()` of the slice must be no larger than `isize::MAX`.
///   See the safety documentation of [`pointer::offset`].
```

---

##### Caveat

```rust
/// # Caveat
///
/// The lifetime for the returned slice is inferred from its usage. To
/// prevent accidental misuse, it's suggested to tie the lifetime to whichever
/// source lifetime is safe in the context, such as by providing a helper
/// function taking the lifetime of a host value for the slice, or by explicit
/// annotation.
```

---
#### Incorrect usage

```rust
/// The following `join_slices` function is **unsound** ⚠️
use std::slice;

fn join_slices<'a, T>(fst: &'a [T], snd: &'a [T]) -> &'a [T] {
    let fst_end = fst.as_ptr().wrapping_add(fst.len());
    let snd_start = snd.as_ptr();
    assert_eq!(fst_end, snd_start, "Slices must be contiguous!");
    unsafe {
        // The assertion above ensures `fst` and `snd` are contiguous, but they might             
        // still be contained within _different allocated objects_, in which case
        // creating this slice is undefined behavior.
        slice::from_raw_parts(fst.as_ptr(), fst.len() + snd.len())
    }
}
fn main() {
    // `a` and `b` are different allocated objects...
    let a = 42;
    let b = 27;
    // ... which may nevertheless be laid out contiguously in memory: | a | b |
    let _ = join_slices(slice::from_ref(&a), slice::from_ref(&b)); // UB
}
```
---
<!-- _class: invert -->

### Unsafe Rust - Using `extern` Functions to Call External Code

- `extern` keyword allows you to define functions in one language and enable a different (foreign) programming language to call those functions.

```rust
extern "C" { // The ABI of the foreign function
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));                
    }
}
```

---
<!-- _class: gaia -->

### Python:

- PyO3 - rust binding for Python interpreter
    - https://github.com/PyO3/pyo3
- Inline Python (based on PyO3)
    - https://blog.m-ou.se/writing-python-inside-rust-1/
- Compile time python (based on inline python)
    - https://docs.rs/ct-python/latest/ct_python/

---

### Unsafe Rust - Accessing or Modifying a Mutable Static Variable

**_The dreaded globallllll_**

```rust
static HELLO_WORLD: &str = "Hello, world!"; // Implicitly using 'static lifetime

fn main() {
    println!("name is: {}", HELLO_WORLD);
}
```

By convention, named with `SCREAMING_SNAKE_CASE`. Clearly the designed have worked with with code using globals before...

---
##### Unsafe Rust - Accessing or Modifying a Mutable Static Variable

_Accessing and modifying mutable static variables is unsafe_

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);                                           
    }
}
```
---
##### Unsafe Rust - Accessing or Modifying a Mutable Static Variable

### Why is this unsafe?
Having multiple threads access COUNTER would likely result in data races.

Even without data races, globals can make it extremely difficult to reason about complex code. We'll all sleep better at night knowing they're wrapped in the unsafe guard

---
<!-- _class: invert -->

### Unsafe Rust - Implementing an Unsafe trait

By using unsafe impl, we’re promising that we’ll uphold the invariants that the compiler can’t verify.

e.g. `sync` and `send`

- `send` means that a type is safe to move from one thread to another. If the same type also implements Copy, this also means that it is safe to copy from one thread to another.

- `sync` means that a type is safe to _reference_ from multiple threads at the same time

---
<!-- _class: invert -->
### Unsafe Rust - Implementing an Unsafe trait

The compiler can only verify that our types are `sync` or `send` if they are made up of other `sync` and `send` types.

If our type contains non-sync or non-send types, but we know that our types satisfy the trait requirements, we must implement the unsafe trait ourselves

---

### Unsafe Rust - Accessing Fields of a Union

Unions:
- similar to a struct, but only one declared field is used in a particular instance at one time
- primarily used to interface with unions in C code.
- accessing union fields is unsafe because Rust can’t guarantee the type of the data currently being stored in the union instance.

---
### Unsafe Rust - Accessing Fields of a Union

```rust
#[repr(C)] // Do what C does for data layout/representation
union MyUnion {
    f1: u32,
    f2: f32,
}
```
Creating a union is safe, accessing a field is unsafe
```rust
let u = MyUnion { f1: 1 };
```

```rust
let f = unsafe { u.f1 };
```

---
#### Unsafe Rust - Accessing Fields of a Union

**Why is this unsafe??**

- Unions have no notion of an "active field"
- Every union access just interprets the storage at the type of the field used for the access
- Reading a union field reads the bits of the union at the field's type.
- Fields might have a non-zero offset (except when the C representation is used)
- It is the **programmer's responsibility** to make sure that the data is valid at the field's type.

---
#### Unsafe Rust - Accessing Fields of a Union

Access is essentially a reinterpret cast (`transmute` in rust speak)

Failing to do so correctly results in undefined behavior.

e.g. reading the value 3 through of a field of the boolean type is undefined behavior.

---
<!-- _class: invert -->
## Unsafe Rust - Conclusions

Unsafe code can be safe, but that safety is the programmers responsiblity.

Bugs may happen, but the unsafe keyword makes it easier to track them down!

---
<!-- _class: lead -->

# Advanced Traits

_Trait Secrets 🔒_

---

# Advanced Traits

### Placeholder Types in Trait Definitions

If you tried to implement your own iterator back in chapter 13 this code may look familiar:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

---
### Advanced Traits - Placeholder Types in Trait Definitions

```rust
pub trait Iterator {
    type Item; // Placeholder type

    fn next(&mut self) -> Option<Self::Item>; // Used in trait definition to
}                                             // define the return type of next
```

Implementors of the Iterator trait will specify the **concrete type** for `Item`, and the next method will return an `Option` containing a value of that concrete type.

---
### Advanced Traits - Placeholder Types in Trait Definitions

##### Why not use generics?

```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

Trait with generic parameters _could_ be implemented for a type multiple times with different generic parameter types

---
##### Advanced Traits - Placeholder Types in Trait Definitions

What it would look like with generics:

```rust
pub trait MyIterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

```rust
struct MyIterable{}

impl MyIterator<u32> for MyIterable {
    fn next(&mut self) -> Option<u32>{
        return Some(1);
    }
}
```

---
##### Advanced Traits - Placeholder Types in Trait Definitions

Fine, but opened up posibility of multiple definitions:

```rust
struct MyIterable{}

impl MyIterator<u32> for MyIterable {
    fn next(&mut self) -> Option<u32>{
        return Some(1);
    }
}

impl MyIterator<f32> for MyIterable {
    fn next(&mut self) -> Option<f32>{
        return Some(2.2);
    }
}
```

---

#### Advanced Traits - Placeholder Types in Trait Definitions

We want to just call next:

```rust
let mut val = MyIterable{};

println!("{:?}", val.next()); // ERROR if more than one next definition
```

But we now have to provide type annotations everytime we use next:

```rust
let mut val = MyIterable{};

println!("{:?}", <MyIterable as MyIterator<f32>>::next(&mut val));
println!("{:?}", <MyIterable as MyIterator<u32>>::next(&mut val));
```

---
### Advanced Traits - Placeholder Types in Trait Definitions

So instead of generics, we use a placeholder type. That way there can only ever be one implementation of the trait on our type

```rust
pub trait Iterator {
    type Item; // Placeholder type

    fn next(&mut self) -> Option<Self::Item>; // Used here
}
```

---
<!-- _class: invert -->
### Advanced Traits - Default Generic Type Parameters and Operator Overloading

```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

Add trait has a generic type parameter so we can add between two different types (e.g. u32 and f32), but defaults to self as the most common type

---
<!-- _class: invert -->
#### Advanced Traits - Default Generic Type Parameters and Operator Overloading

This simplifies our implementation blocks:

```rust
struct Point { x: i32, y: i32, }

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {                                         
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
```

---
<!-- _class: invert -->
#### Advanced Traits - Default Generic Type Parameters and Operator Overloading

Instead of needing an extra type annotation on the impl block:

```rust
impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
```

---
<!-- _class: invert -->
#### Advanced Traits - Default Generic Type Parameters and Operator Overloading

- Default type parameters like this are used in two main ways:
  - To extend a type without breaking existing code
  - To allow customization in specific cases most users won’t need

---

### Advanced Traits - Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

There are only so many good names to go around, and some trait methods may have the same name...

---

```rust
trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
```

---

#### Advanced Traits - Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

How does rust deal with this ambiguity?

```rust
fn main() {
    let person = Human;
    person.fly();
}
```

---

#### Advanced Traits - Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

How does rust deal with this ambiguity?

```rust
fn main() {
    let person = Human;
    person.fly(); // Defaults to method implemented directily on type
}
```

To use the other `fly` methods we need to use more explicit syntax

---
#### Advanced Traits - Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

To use the other `fly` methods we need to use more explicit syntax:

```rust
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
```

This syntax works because our fly method took the `&self` parameter.

---

```rust
trait BabyAnimal {
    fn name() -> String;
}

struct Dog;

impl Dog {
    fn name() -> String {
        String::from("Spot")
    }
}

impl BabyAnimal for Dog {
    fn name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::name());
}
```

---
#### Advanced Traits - Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

In this case, how do we call `Dog`'s `BabyAnimal` trait `name` method?? We can't use `BabyAnimal::name(&dog)` because name doesn't take a self paramter.

```rust
println!(Dog::name()) // Prints spot
```

```rust
println!(BabyAnimal::name()) // Could be any animal! No link to Dog!
```

---
#### Advanced Traits - Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

For these cases we need to use our fully qualified syntax:

```rust
println!("A baby dog is called a {}", <Dog as BabyAnimal>::name());
```

This is the same syntax we saw earlier when we tried to set up our iterator trait using a genereric type parameter instead of a trait placeholder type:

```rust
println!("{:?}", <MyIterable as MyIterator<u32>>::next(&mut val));
```

---
<!-- _class: invert -->

### Advanced Traits - Using Supertraits to Require One Trait’s Functionality Within Another Trait

We can use a dependent trait's method when defining a new trait by marking it a super trait of it:

```rust
use std::fmt;

trait HeartPrint: fmt::Display {
    fn heart_print(&self) {
        let output = self.to_string(); // to_string() is on fmt::Display trait
        println!("❤️{}❤️", output);
    }
}
```

---
<!-- _class: invert -->

```rust
use std::fmt;

trait Loveable{}

trait HeartPrint: fmt::Display + Loveable {
    fn heart_print(&self) {
        let output = self.to_string(); // to_string() is on fmt::Display trait
        println!("❤️{}❤️", output);
    }
}

struct Programming{}

impl fmt::Display for Programming {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Programming")
    }
}

impl Loveable for Programming {}

impl HeartPrint for Programming {}
```

---
### Advanced Traits - Using the Newtype Pattern to Implement External Traits on External Types

We can only implement a trait on a type if either the `type` or the `trait` are local to our crate.

This means we can't, for example, implement format on a vector of strings.

---
### Advanced Traits - Using the Newtype Pattern to Implement External Traits on External Types

We can instead implement the trait on a thin tuple struct wrapper type


```rust
use std::fmt;

struct MyWrapper(Vec<String>);

impl fmt::Display for MyWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
```

---

### Advanced Traits - Using the Newtype Pattern to Implement External Traits on External Types

```rust
struct MyWrapper(Vec<String>);                                                            
```

Note: we need to access the first element in our tuple struct to get to the inner type and any methods on it e.g. sort

```rust
let mut my_vec = MyWrapper(vec!["sort".to_string(),"vec".to_string(),"strings".to_string()]);
my_vec.0.sort();
```

If we really wanted the new type to have every method the inner type has, we could implement the `Deref` trait

---
<!-- _class: lead invert -->

# Advanced Types

---
<!-- _class: invert -->

### Advanced Types - Using the Newtype Pattern for Type Safety and Abstraction

By creating new `Millimeters` and `Meters` wrapper types instead of using `u32`s we can make common units bugs easier to spot and prevent

```rust
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, meters: Meters) -> Millimeters {
        Millimeters(self.0 + (meters.0 * 1000))
    }                                                                                         
}
```

---
### Advanced Types - Using the Newtype Pattern for Type Safety and Abstraction

Creating these wrappter types also allows us to expose a public API that is different from the API of the private inner type.

Example: Internally store `HashMap<i32, String>`, but provide simple API which assigns an ID internally:

```rust
let mut people = People::new();
people.add_person("Ferris");
```

---
#### Advanced Types - Using the Newtype Pattern for Type Safety and Abstraction

```rust
struct People(HashMap<i32, String>);

impl People {
    fn new() -> People {
        People(HashMap::new())
    }

    fn add_person(&mut self, name: &str) {
        self.0.insert(new_id(), name.to_string());
    }
}
```

Providing lightweight encapsulation

---
<!-- _class: invert -->

### Advanced Types - Creating Type Synonyms with Type Aliases

We may also see see type aliases used in place of simple types

```rust
type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y);
```

This doesn't give us the type safety which the newtype pattern we have just seen, as it is simply an alias

---
<!-- _class: invert -->
### Advanced Types - Creating Type Synonyms with Type Aliases

But it can still be useful for avoiding reqriting cumbersome verbose types, or to make our code clearer.

```rust
type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    // --snip--
}
fn returns_long_type() -> Thunk {
    // --snip--
}
```

---

# Advanced Types - The Never Type that Never Returns 🤔

What? Why? What on earth?

---
# Advanced Types - The Never Type that Never Returns 🤔

These are the return type of 'diverging functions' which never return, and are denoted with an exclamation mark `!`. It can be coerced into any type, since it never returns anyway!

```rust
#![feature(never_type)]

fn main() {
    let x: ! = panic!("This call never returns.");
    println!("You will never see this line!");
}
```

---

```rust
fn sum_odd_numbers(up_to: u32) -> u32 {
    let mut acc = 0;
    for i in 0..up_to {
        // Notice that the return type of this match expression must be u32
        let addition: u32 = match i%2 == 1 {
            // The "i" variable is of type u32, which is perfectly fine.
            true => i,
            // On the other hand, the "continue" expression does not return
            // u32, but it is still fine, because it never returns and therefore
            // does not violate the type requirements of the match expression.
            false => continue,
        };
        acc += addition;
    }
    acc
}
println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
```

---
## Advanced Types - The Never Type that Never Returns 🤔

`continue` is returning our 'never type', which is why the type checker isn't complaining that something other than a u32 is being assigned into our `addition` variable.

The 'never type' is also the return type of functions that loop forever (e.g. `loop {}`) like network servers or functions that terminate the process (e.g. `exit()` or `panic!`).


_Though this be madness, yet there is method in't_

---
<!-- _class: invert -->

## Advanced Types - Dynamically Sized Types and the Sized Trait

Where is this?

```rust
let hello = "hello";
```

That stack? The heap? Somewhere else??

---
<!-- _class: invert -->

### Advanced Types - Dynamically Sized Types and the Sized Trait

Well what type is it? If we annotate it with `str`:

```rust
let hello: str = "hello"; // ERROR
```
We get an error:
```
the size for values of type `str` cannot be known at compilation time
the trait `Sized` is not implemented for `str`
all local variables must have a statically known size
```

---
<!-- _class: invert -->

### Advanced Types - Dynamically Sized Types and the Sized Trait

We know the length of **our string** at compile time, but not the length of the **type `str`**

Because `str` is a dynamically sized type.

The correct type annotation is `&str`

```rust
let hello: &str = "hello";
```

---
<!-- _class: invert -->

### Advanced Types - Dynamically Sized Types and the Sized Trait

The string literal goes into your read-only data (ro.data) of your executable. It doens't allocate at run time, and its type has a known stack size at compile time.

The stack component is simply a pointer to its address, and its size.

---
<!-- _class: invert -->

### Advanced Types - Dynamically Sized Types and the Sized Trait

The error we got for type to type annotate with `str` mentioned the `Sized` trait not being implemented.

The `Sized` trait is automatically implemented for all types where the size is known at compile time.

---
<!-- _class: lead -->

# Advanced Functions and Closures

---

## Advanced Functions and Closures - Function Pointers

Function pointers allow us to pass functions to other functions (as opposed to closures, which are anonymous and might capture state where they're reated).

---
### Advanced Functions and Closures - Function Pointers

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}
```
---

### Advanced Functions and Closures - Function Pointers

Our type `fn(i32) -> i32` is a function pointer

```rust
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
```
When we covered closures in chapter 13 we were using generic type parameters with an `Fn` trait bound, but here we are using a concrete type of 'function pointer'

---
### Advanced Functions and Closures - Function Pointers

Function pointers can be used wherever closures can be used as they implement all three of the closure traits:

- `Fn`
- `FnMut`
- `FnOnce`

It is usually preferred to used the generic type parameters, to allow both closures and function pointers to be passed in

---
#### Advanced Functions and Closures - Function Pointers

Let's compare using a closure vs a function pointer when mapping `let list_of_numbers = vec![1, 2, 3];` to strings

```rust
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
```

```rust
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();
```

`|i| i.to_string()` vs `ToString::to_string`

---

#### Advanced Functions and Closures - Function Pointers

Similarly, we can map a list of values to an enum which wraps them, using either a closure, or a function pointer in the form of the enum type:
```rust
enum Status {
    Value(u32),
    Stop,
}

let use_func_ptr: Vec<Status> = (0u32..20).map(Status::Value).collect();

let use_closure: Vec<Status> = (0u32..20).map(|val| Status::Value(val)).collect();
```

---
#### Advanced Functions and Closures - Function Pointers

Function pointers can be returned from functions:

```rust
fn returns_function_pointer() -> fn() -> i32 {
    my_function
}
```
But closures cannot since Rust doesn’t know how much space it will need to store the closure.

```rust
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |i| i + 1
}
```

---
#### Advanced Functions and Closures - Function Pointers

The error message gives us a work around for simple cases:

```rust
fn returns_closure() -> impl Fn(i32) -> i32 {
    |i| i + 1
}
```

but for more general return values we may need to use the `Box` type

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

---
<!-- _class: lead invert -->

# Macros

---

<!-- _class: lead -->
# To be continued...

---

<!-- _class: invert -->

# What next?

- _Chapter 20: Final Project: Building a Multithreaded Web Server_
- And Beyond!!

---

