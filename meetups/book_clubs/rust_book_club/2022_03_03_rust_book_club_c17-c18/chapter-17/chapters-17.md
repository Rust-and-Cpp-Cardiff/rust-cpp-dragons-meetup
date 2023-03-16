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
## discord: Rust and C++ Cardiff

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

- **Week 1-5**: Chapters 1-10
- **Week 6**: Writing Automated Tests & Building a Command Line Program
- **Week 7**: Functional Language Features & More on Cargo and Crates.io
- **Week 8**: Smart Pointers
- **Week 9**: Fearless Concurrency

---

<!-- _class: lead-->

# **Rust Book**

Chapter 17

_Object Orientation_

---

## 17. Object Orientation

- 17.1 Characteristics of Object Oriented Languages
- 17.2 Using Trait Objects That allow for Values of Different Types
- 17.3 Implementing an Object Oriented Design Pattern

---
<!-- _class: invert-->


## History of Object Orientation

- _Object Orientation_ is a way of modelling programs

- The concept of objects first used in the Simula language in the 1960s

- This influenced Alan Kay's programming architecture in which objects pass messages to each other

- He coined the term _Object Orientated Programming_ in 1967 to describe this architecture

---

## Rust Book definition of OO

- No consensus in the programming community about what features a language must have to be considered object oriented. 

- Many OO languages share certain characteristics. 

- This portion of the book runs through those characteristics and discusses Rust's level of support for them.
---

## Rust Book definition of OO

_Objects Contain Data and Behaviour_

- "Object-oriented programs are made up of objects. An object packages both data and the procedures that operate on that data. The procedures are typically called methods or operations."
- Rust supports this form of program modelling. `Structs` and `Enums` can hold data and their `Impl` blocks provide methods to operate on that data.

---

## Rust Book definition of OO

_Encapsulation that Hides implementation details_

- Implementation details of an object aren’t accessible to code using that object. The only way to interact with an object is via its public API.

- Code using the object shouldn’t be able to reach change the object’s internal data or behavior directly. 

---

## Rust Book definition of OO

_Encapsulation that Hides implementation details_

- This enables the programmer to change and refactor an object’s internals without needing to change the code that uses the object.

<!-- It's an abstraction technique to  prevent data/information leakage to other areas of the system, keeping them as simple as possible, and in theory it forces us to write more descriptive method names.

For example if I wrote a game/simulation about feeding a dog, rather than reducing its hunger value, you would call a method like dog.feed(10);-->

- Encapsulation in Rust is controlled using the `pub` keyword

- `pub` determines which modules, types, functions, methods and members are made public, and everything else is private by default. 

---

## Rust Book definition of OO

_Inheritance_

- Inheritance is a mechanism whereby an object can inherit from another object’s definition, thus gaining the parent object’s data and behavior without you having to define them again.

- If a language must have inheritance to be an object-oriented language, then Rust is not one. 

- There is no way to define a `struct` that inherits the parent `struct’s` fields and method implementations. 

<!-- However, if you’re used to having inheritance in your programming toolbox, you can use other solutions in Rust, depending on your reason for reaching for inheritance in the first place. The book argues that programmers reach for inheritance for one of two reasons. -->

---

## Rust Book definition of OO

_Inheritance as a means of sharing code_

- Inheritance in OO languages allows for the reuse of code: you can implement particular behavior for one type, and inheritance enables you to reuse that implementation for a different type. 

- You can share Rust code using default trait method implementations instead

---

## Rust Book definition of OO

_Inheritance as a means of sharing code_
<!--
    We saw this in Chapter 10 when we added a default implementation of the summarize method on the Summary trait.
    Any type implementing the Summary trait would have the summarize method available on it without any further code.
-->

```
// Summarize defined with a default implementation
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// Summarize doesn't require a definition when implementing the Summary trait.
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

---

## Rust Book definition of OO

_Inheritance as a type system_

- Inheritance allows for a child type to be used in place of a parent type. 

- This is called polymorphism 
<!-- which means that you can substitute multiple objects for each other at runtime if they share certain characteristics. -->

<!-- 
    To many people, polymorphism is synonymous with inheritance. But it’s actually a more general concept that refers to code that can work with data of multiple types. For inheritance, those types are generally subclasses.
-->
---

## Rust Book definition of OO

_Inheritance as a type system_

- Rust uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide. 

- This is sometimes called bounded parametric polymorphism. 
<!--
    Again, we saw this in chapter 10, in the code snippet below we can see a function accepting a generic (or template) type T, that must implement the Summary trait from the previous example 
    This strikes me as being very similar to C++20's template concepts, though I haven't got much experience using them so if anyone knows of any reason why they aren't equivalent I'd love to know.
-->

```
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```
---
## Rust Book definition of OO

_Inheritance: Why no Inheritance in Rust?_

Inheritance has "recently" fallen out of favor:
-  Often risks sharing more code than necessary. 
<!-- Subclasses shouldn’t always share all characteristics of their parent class but will do so with inheritance. This can make a program’s design less flexible.-->
- Possible to call methods on subclasses that don’t make sense or that cause errors because the methods don’t apply to the subclass. 
- Some languages only allow a subclass to inherit from one class, restricting the flexibility of a program’s design.
<!-- I actually think restricting inheritance is a good thing for OO languages. My personal mental model for inheritance is pure interface > abstract class > concrete type. I've found in personal experience if it goes beyond that, its usually a bad thing. And even then, having that shared code in the abstract class inbetween is more of an edge case thing.-->

<!--
    The book claims that this loss of favour is recent, and maybe I'm just too young and everything they consider recent encapsulates my entire career, but the phrase 'favour composition over inheritance' has been ingrained into my psyche since university. Presumably that rule/mantra gained popularity for precisely these reasons.
-->

<!--For these reasons, Rust takes a different approach, using trait objects instead of inheritance.-->

---
## Using Trait Objects
_Ex: How trait objects enable polymorphism in Rust_

<!--
    In chapter 8 (Common Collections) we looked at an example situation in which we wanted to use a vector to store elements of varying types, despite the fact that vectors can only store elements of one type. In chapter 8, we got around that by defining an enum that had variants to hold ints, floats, text etc..
    This is a perfectly good solution when our possible types are known when our code is compiled.

    The situation we're interested in now is when those types are not known at compile time.

    The example the book uses is that:
    
        We want to develop a GUI library that defines a number of items that all have a method called `draw`. We want the library to allow for users to extent the library with their own types that can implement draw. 
    
        We want the library to be able to go through all types defined by itself and defined by users (which we can't know about when writing the library) and be able to call draw on all of them.

        With an OO language we might do this by defining an abstract or interface class named something like `Component` that has a method named `draw` on it, and have other classes inherit from `Component` and thus inherit the `draw` method. Rust doesn't have inheritance so we need another way to structure the gui library.
-->

- Libray defines items that all have a method `draw`

- Users can add their own types that implement `draw`

- Library can iterate through a collection of types including those defined by itself and library implementers, and call `draw` on all of them

---

## Using Trait Objects
_Defining a trait for common behaviour_
<!-- 
    To implement the behavior we want gui to have, we’ll define a trait named Draw that will have one method named draw. 
    Then we can define a vector that takes a trait object. A trait object points to both an instance of a type implementing our specified trait as well as a table used to look up trait methods on that type at runtime. 
    
    We create a trait object by specifying some sort of pointer, such as a & reference or a Box<T> smart pointer, then the dyn keyword, and then specifying the relevant trait. In chapter 19 we'll cover why a trait object must use a pointer.

    Trait objects can be used instead of a generic or concrete type. Wherever we use a trait object, Rust’s type system will ensure at compile time that any value used in that context will implement the trait object’s trait. Consequently, we don’t need to know all the possible types at compile time.

-->
- Define a trait (as seen in chapter 10)
```
pub trait Draw {
    fn draw(&self);
}
```

- Define a vector with a trait object as its type
```
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```

---

## Using Trait Objects
_Defining a trait for common behaviour_

- Iterator through the vector and all draw on each item.
```
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

---

## Using Trait Objects
_Different to generic type and trait bounds_

<!-- 
    Important to note that this is different from defining a struct that uses a generic type parameter with trait bounds. 

    The book provides the example on screen to remind us that a generic type parameter can only be substituted with one concrete type at a time, 
    unlike trait objects which allow for multiple concrete types to fill in for the trait object at runtime.

    If you know that you'll only ever have collections of a single type, using generics and trait bounds is preferable to trait objects because
    the definitions will be monomorphized at compile time to use concrete types.

    Monomorphization is a compile-time process in which a polymorphic function is replaced by is underlying concrete function. 
    This means that the code generated at compile time for the call will be performing static-dispatch instead of dynamic-dispatch. 
    (But I'm getting ahead of myself ;) )
-->

```
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```
---
## Using Trait Objects
_Implementing the trait_

<!-- 
    The book's example continues to define a toy example type which is provided by the library called Button which implements the draw trait.
    This should look familiar as again, it was covered in chapter 10.
-->

```
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
```

---
## Using Trait Objects
_Implementing the trait_

```
use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

```
---
## Using Trait Objects
_Implementing the trait_
<!-- 
    And again the book continues the scenario: someone using the library decides to create their own type, SelectBox, which implements Draw
-->

```
use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
```

---
## Using Trait Objects
<!-- 
    Our library's user can now write a main function that creates an instance of the screen type - which owns a vector of Draw Trait Objects - 
    and assign both the library defined Box type, and the user's own SelectBox type, and call their draw functions.
-->
```
use gui::{Button, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```
---
## Using Trait Objects
_Defining a trait for common behaviour_

<!-- 
    So just to reiterate, when we wrote this library in our example. We didn't know that someone would come along and create the SelectBox type,
    but Screen was able to operate on the new type and draw it because SelectBox implements the Draw trait, meaning it implements the Draw method.

    The concept of being concerned only with the messages a value responds to rather than the value's concrete type, is similar to the idea of
    duck typing from languages with dynamic typing. If it walks like a duck and quacks like a duck, it's a duck!

    The implementation of run didn't need to know what concrete types it was holding because it knows it will only be asked to call methods defined on the draw trait.
    This is the power of combining trait objects and the Rust type system. 
-->

- We didn't know the user would create `SelectBox`
- This is similar to Duck Typing 
- Implementation of `Screen` doesn't need to know the underlying type, only that it implements the Draw trait.
- Rust can check for this at Runtime

---
## Using Trait Objects
_Defining a trait for common behaviour_
<!-- 
    The book gives us this example, which won't compile because String doesn't implement Draw. 
-->
```
use gui::Screen;

fn main() {
    let screen = Screen {
        components: vec![Box::new(String::from("Hi"))],
    };

    screen.run();
}
```
---
## Using Trait Objects
_Defining a trait for common behaviour_

```
$ cargo run
   Compiling gui v0.1.0 (file:///projects/gui)
error[E0277]: the trait bound `String: Draw` is not satisfied
 --> src/main.rs:5:26
  |
5 |         components: vec![Box::new(String::from("Hi"))],
  |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Draw` is not implemented for `String`
  |
  = note: required for the cast to the object type `dyn Draw`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `gui` due to previous error
```
---
## Using Trait Objects
_Trait objects perform dynamic dispatch_

Alex is going to waffle about dynamic dispatch

<!-- 
    If you'll remember, I mentioned monomorphization earlier. It was also covered in chapter 10 when discussing trait bound generics.

    The compiler generates nongeneric implementations of functions and methods for each concrete type that we use in place of a generic type parameter. 
    The code that results from this is doing static dispatch, which is when the compiler knows what method you’re calling at compile time.

    This is opposed to dynamic dispatch, which is when the compiler can’t tell at compile time which method you’re calling. In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.
    When we use trait objects, Rust must use dynamic dispatch because it doesn't know all the possible types that could implement the trait, and therefore cannot tell at compile time which method is being called.

    Instead it relies on pointers stored inside the trait object, which are used at runtime to know which method to call. There is a runtime cost for performing this lookup, but we've traded this slight performance hit for an increase in code flexibility. 
-->

---

## Using Trait Objects
_Object safety is required for Trait Objects_

<!-- 
    Trait objects must be what we call Object-Safe traits. This is enforced because once you’ve used a trait object, Rust no longer knows the concrete type that’s implementing that trait.

    The book alludes to complex rules that govern what makes a trait object-safe, but says that practically it comes down to these two rules.

    The Self keyword is an alias for the type we’re implementing the traits or methods on. But as we've established we don't know what the concrete type is, we can't resolve Self.

    This is the same story for the generic types, we can't resolve the concrete type, so we can't fill the generic type in with a concrete type.
-->

- The return type isn't `Self`
- There are no generic type parameters

---

## Using Trait Objects
_Object safety is required for Trait Objects_

<!--
    And the book gives us the Clone trait as an example. Clone returns Self, and when we try to use it as a trait object, the compiler tells us that we can't do that because it's not an object-safe trait.
-->

```
pub trait Clone {
    fn clone(&self) -> Self;
}

pub struct Screen {
    pub components: Vec<Box<dyn Clone>>,
}
```
```
error[E0038]: the trait `Clone` cannot be made into an object
 --> src/lib.rs:2:29
  |
2 |     pub components: Vec<Box<dyn Clone>>,
  |                             ^^^^^^^^^ `Clone` cannot be made into an object
  |
  = note: the trait cannot be made into an object because it requires `Self: Sized`
  = note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
```
---

## Implementing an Object-Oriented Design Pattern

<!-- 
    Final section of this chapter is a guided example of implementing a program utilising the State Pattern. I think most of the benefit of this chapter comes from following it yourself but I will quickly run through it. 

    The crux of the pattern is that an object has some internal state, which is represented by a set of state-objects. The object's behaviour changes based on its internal state. 

    The key idea being that when business rules change, we'll only need to update the code inside one of these state objects, or possibly add/remove some.

    The example details a blog post program, with the end aim being the following workflow.
-->

- A blog post starts as an empty draft.
- When the draft is done, a review of the post is requested.
- When the post is approved, it gets published.
- Only published blog posts return content to print, so unapproved posts can’t accidentally be published.

<!-- 
    Any other changes attempted on a post should have no effect.
    For example, if we try to approve a draft blog post before we’ve requested a review, the post should still be an unpublished draft afterwards. 
-->
---
## Implementing an Object-Oriented Design Pattern

<!-- 
    Here's the example usage of the API for our Post object, you can imagine its internally utilising state-objects on each method call.
    Refering back to the workflow on the previous slide, 
        we want to allow the user to create a new draft blog post with Post::new

        we want to be able to add text, maybe we want to add multiple strings, maybe its collecting input from multiple users, or at different time frames. 
        we'll want to be allowed to keep adding text with mutliple method calls, so don't change state when adding text

        we want to enable a request for review, signaling we're waiting for someone to verify the text collected so far.

        finally, the post can be approved. It's at this point that the post's content will actually be queryable, as it's now in the appropriate state.

    Even though we've discussed the transitioning of states, and state objects, notice that we haven't interacted with any state objects in this program. They've all been encapsulated within the Post object. The Post object manages the transition between the 3 internal states (draft, waiting for review, approved), preventing the user of the object from making a mistake.
-->

```
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
```
---
## Implementing an Object-Oriented Design Pattern
_Defining Post and Creating a New Instance in the Draft State_

<!-- 
    To Start, we create a Post object, which owns it's content string and its state object. 
    State is a trait object because it could be one of three states during runtime. 

    We make the State trait private so that it isn't exposed to users of the library, remember this is an internal trait-object.
    Note that state is Optional, I'll explain why in a minute.

    When we create the Post Object, we set the state value to a Some that holds a box pointing to a Draft struct.
    This ensures that whenever a new Post object is created, it's initial state is Draft as there's no way to create a Post Object.
-->

```
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

trait State {}

struct Draft {}

impl State for Draft {}
```

---
## Implementing an Object-Oriented Design Pattern
_Storing the text of the Post Content_

<!--    
    This is pretty straightforward. It appends text and doesn't alter the internal state at all.
    It also doesn't depend on internal state either, so no checks need to take place.
 -->

```
impl Post {
    // --snip--
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

---
## Implementing an Object-Oriented Design Pattern
_Requesting a Review of the Post Changes Its State_

<!-- 
    Next, we want to add functionality to request a review of a post, which should change its state from Draft to PendingReview.

    We've added a public method called request_review to Post
    We've added a public method called request_review to the state trait.
        Which only allows us to call it if we're calling it from a Boxed type.

    We've implemented the function for the Draft struct
    We've also created a new struct called PendingReview, which impelments the state trait.

    For the two states, the request_review method differs in behaviour.
        For Draft, it returns a new Box pointing to a PendingReview struct
        For PendingReview, it returns self.


    When Post::request_review is called, it calls the state's request review, assigning the result to its internal state member.

    To consume the old state, the request_review method needs to take ownership of the state value. 
    
    This is why the state is an Optional. We call the take method to take the Some value out of the state field and leave a None in its place, because Rust doesn’t let us have unpopulated fields in structs. 
    This lets us move the state value out of Post rather than borrowing it. Then we’ll set the post’s state value to the result of this operation.
-->

```
impl Post {
    // --snip--
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
```

---
## Implementing an Object-Oriented Design Pattern
_Adding the approve Method_


<!-- 
    The approve method is similar to request review in that it calls a method on the internal state, and uses it to update its internal state.
    This time, we'll call a new method on the State trait, approve. We'll also add a new satte called Published 
    Approve will do nothing if called on a Draft or a Published struct. For a PendingReview, it will return a new Box pointing to a Published.

-->

```
impl Post {
    // --snip--
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    // --snip--
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // --snip--
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
```

---
## Implementing an Object-Oriented Design Pattern
_Returning content_

<!-- 
    Now, we only want to be able to return the content string from the Post object if the state is of type Published.
    As the goal is to keep all the rules inside the structs that implement state, we're going to require a new method for the state object.
    It will pass in a Post object, as you can see with self, in order to obtain the content string.
-->

```
impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    // --snip--
}
```

---
## Implementing an Object-Oriented Design Pattern
_Returning content_
<!-- 
    So we add the content method to State, which accepts a Post, but it only returns content if the State is of type Published. 
    This is how we enforce the rule that Post only returns its content when its in the Published state.
    And we've handled this with a default implementation for content, which is only overriden by Published.

    My only source of confusion with this example is that we can access the private member of Post from within another type. 
    If anyone can clarify that I'd greatly appreciate it.

    But as far as the OO side of things go, this code works and the state pattern has been implemented.
-->

```
trait State {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// --snip--
struct Published {}

impl State for Published {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
```

---
## Implementing an Object-Oriented Design Pattern
_Trade offs of the State pattern_

<!-- 
    The book's shown us that Rust is capable of implementing OO design patterns. (Or at least, the state pattern).
    If we were to create an alternative implementation without this pattern, we might've used match expressions in each of Post's methods, but this would mean we'd have to look in several places to understand the full implications of a post being in a particular state. This would get worse as the number of states increased.

    With that in mind, the state pattern brings us the following benefits
-->

- Implementation is easy
- Straight forward to extend

---
## Implementing an Object-Oriented Design Pattern
_Trade offs of the State pattern_

<!-- 
    However it does have downsides....
-->

- Some of the states are coupled together, we rely on explicit transitioning from one to another
- We've duplicated some logic, request_review and approve cannot have default implementations because they return Self
- This would violate the object-safety requirement of trait objects.

<!-- 
    In short, following this pattern exactly as defined for OO languages means we don't take full advantage of Rust's strengths.
-->

---
## Implementing an Object-Oriented Design Pattern
_Making it Rustic_
<!-- 
    The book proposes two key ways in which we can Rustify/Corrode this solution, or make this example Rustic.
    We're shown how to rethink the problem, so that rather than abstract away the state and transitions, we enforce them using Rust's type system.
-->

- Encoding States and Behavior as Types
- Implementing Transitions as Transformations into Different Types

---
## Implementing an Object-Oriented Design Pattern
_Encoding States and Behavior as Types_
<!-- 
    We still enable the creation of new Posts with the new function, but now it returns a DraftPost type. This is the only type that has a method available allowing us to add text, but the DraftPost type cannot return the content at all. It's now impossible for a DraftPost to be accidentally queried for content because the compiler will not allow it. There's no content method.
-->

```
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```
---
## Implementing an Object-Oriented Design Pattern
_Implementing Transitions as Transformations into Different Types_

<!-- 
    We want to enforce the rule that a draft post has to be reviewed and approved before it is published.
    DraftPost can have its request_review method return a type PendingReviewPost.
    This new type only has the ability to be approved. The approve method will return a type Post, whose contents can be viewed as we saw in the previous slide.

    These types take ownership of self and consume them, thus enforcing the transition aspect. With no lingering references to the previous states, we've ensured that the state has truely transformed.  
-->

```
impl DraftPost {
    // --snip--
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
```
---
## Implementing an Object-Oriented Design Pattern

<!-- 
    This does require some changes to Main.
    We can't assert the content at each stage anymore (nor do we want to). And we must also capture each returned Post from the method calls.
    We can utilise shadowing assignments to prevent having to write out multiple variations of the same variable name though.


    These changes mean that we don't really follow the state pattern anymore: the transformations between the states are no longer encapsulated entirely within the Post implementation. 
    However, invalid states are now impossible because we've utilised the type system to enforce our state rules at compile time! This prevents logic bugs from being entered into production.
-->
```
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
```
---
## Summary

<!-- 
    We’ve seen that:
        - Rust can implement OO Design Patterns, but those patterns have trade offs
        - Rethinking the problem to take advantage of Rust’s features can provide benefits, such as preventing some bugs at compile time. 


        - We know that we can use trait objects to get some object-oriented features in Rust. 
        - Dynamic dispatch can give your code some flexibility in exchange for a bit of runtime performance. 
        - You can use this to implement OO patterns that can make your code easier to maintain. 
        
        -An object-oriented pattern won’t always be the best solution to a problem with Rust, because it won't necessarilty take advantage of Rust’s strengths.
-->

- Rust can implement OO Design Patterns
- Beneficial to rethink these problems Rustically
- Trait Objects are a means to get OO features in Rust
- Dynamic dispatch can give Rust code some flexibility 
- OO patterns won't take advantage of Rust's strengths
---

<!-- _class: lead -->
# End of chapter 17