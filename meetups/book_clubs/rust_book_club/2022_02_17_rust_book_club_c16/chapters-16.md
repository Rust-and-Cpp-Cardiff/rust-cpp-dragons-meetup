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
<!-- _class: invert -->

#### Recap

- **Chapters 1- 6:** Installation, Essential Concepts, Ownership, Structs, Enums and Pattern Matching
- **Chapters 7 - 12:** Packages, Crates and Modules, Vectors, Strings and Hashmaps, Error Handling, Generic Types, Traits and Lifetimes, Writing Automated Tests, and Building a Command Line Program
- **Chapters 13 - 16:** Functional language features: iterators & closures, more about Cargo and Crates.io, Smart pointers

---

<!-- _class: lead-->

# **Rust Book**

Chapter 16

_Fearless Concurrency_

---

## 16. Fearless Concurrency

- 16.1 Using Threads to Run Code Simultaneously
- 16.2 Using Message Passing to Transfer Data Between Threads
- 16.3 Shared-State Concurrency
- 16.4 Extensible Concurrency with the Sync and Send Traits

---
<!-- _class: invert-->


## Why Concurrency?

_SPEED_

Some terminology:

- Concurrent programming: where different parts of a program execute independently
- Parallel programming: where different parts of a program execute at the same time

---

![bg fit](images/ConcurrentVsParallel_coffee.jpg)

---

## Concurrent

![width:1000](images/concurrent.png)


$_{Jakob\space Jenkov: \space Concurrency \space vs. \space Parallelism}$

---

### Parallel

![width:900](images/parallel.png)

$_{Jakob\space Jenkov: \space Concurrency \space vs. \space Parallelism}$

---

### Parallel Concurrent Execution

![width:900](images/concurrent_and_parallel.png)

---

## Concurrency vs Parallelism

- Concurrency: 'Dealing with a lot of things at once' **Structure**

- Parallelism: 'Doing a lot of things at once' **Execution**

Parallelism is not the goal of concurrency. The goal of concurrency is to create a good structure. BUT concurrency can allow things to be better parallelized.

---

## Concurrency vs Parallelism

If there is only one processor for example, you need things like a mouse, keyboard etc to be concurrent even though you have no way to parallelize them.

#### Concurrency

- Structure a program into independent pieces
- Coordinate those pieces via some kind of communication

---

- Video by Rob Pike "Concurrency is not parallelism" https://www.youtube.com/watch?v=oV9rvDllKEg
- Paper by Tony Hoare "Communicating Sequential Processes" https://www.cs.cmu.edu/~crary/819-f09/Hoare78.pdf

> In order to use [a multiprocessor machine] effectively on a single
task, the component processors must be able to communicate and to synchronize with each other. Many methods of achieving this have been proposed.

---
<!-- _class: invert-->

## Fearless Concurrency

### As if we needed another reason to love the borrow checker

> By leveraging ownership and type checking, many concurrency errors are compile-time errors in Rust rather than runtime errors.

---

### 16.1 Using Threads to Run Code Simultaneously


- An executed program’s code is run in a process
- A typical operating system manages multiple processes at once
- Within your program, you can also have **independent parts** that run **simultaneously**
- The features that run these independent parts are called **threads**

---
<!-- _class: invert-->

### Threads

- Perks
  - Speed
  - Can keep your applications interactive
  - program does multiple tasks at the same time
- Drawbacks
  - Complexity
  - no inherent guarantee about the order in which parts of your code on different threads will run

---
<!-- _class: invert-->

### Threads

DANGER

- **Race conditions**, where threads are accessing data or resources in an inconsistent order
- **Deadlocks**, where two threads are waiting for each other to finish using a resource the other thread has, preventing both threads from continuing
- **Bugs** that happen only in certain situations and are hard to reproduce and fix reliably

---

### Threads

Rust provides 1:1 threads

One operating system thread per one language thread.

---
<!-- _class: invert-->

#### Creating a new thread

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

---
<!-- _class: invert-->

```txt
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 2 from the main thread!
hi number 3 from the spawned thread!
hi number 3 from the main thread!
hi number 4 from the spawned thread!
hi number 4 from the main thread!
hi number 5 from the spawned thread!
```

The application terminates before the spawned thread completes its work. To wait on it we can join the `handle<()>` returned from `thread::spawn`

---
#### Blocking on a thread

```rs
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

handle.join().unwrap();
```

---
#### Blocking on a thread

Why `unwrap` after `join`?

If the associated thread panics, `[Err]` is returned with the parameter given to panic.

---

### Thread data

The borrow checked in Rust makes it clear that a thread must own the data it uses.

```rust
let v = vec![1, 2, 3];

let handle = thread::spawn(|| { // ERROR
    println!("Here's a vector: {:?}", v);
});
// On main thread, we could have dropped v or changed, invalidating reference
handle.join().unwrap();
```

> closure may outlive the current function, but it borrows `v`, which is owned by the current function

---

### Thread data

We need to move the data into the closure

```rust
let v = vec![1, 2, 3];

let handle = thread::spawn(move || { // Fine
    println!("Here's a vector: {:?}", v);
});

// v not available here any more - already moved into closure

handle.join().unwrap();
```

---
<!-- _class: invert -->

## 16.2 Using Message Passing to Transfer Data Between Threads

> Do not communicate by sharing memory; instead, share memory by communicating.

---

### Channels

In Rust, we use a `channel` to accomplish this

There are two parts of a channel - a transmitter or type `Sender<>` and a receiver of type `Receiver<>`

One part of your code calls methods on the transmitter/sender with the data you want to send, and another part checks the receiving end for arriving messages.

A channel is said to be closed if either the transmitter or receiver half is dropped.

---

### Channels

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel(); // multiple producer, single consumer
                                    // Sender<T> and Receiver<T> types inferred from use

    thread::spawn(move || {         // tx is moved inside the closure
        let val = String::from("hi");
        tx.send(val).unwrap();      // tx is of type Sender<String>
    });

    let received = rx.recv().unwrap(); // rx is of type Receiver<String>
    println!("Got: {}", received);
}
```

---
### Channels - the sender

```rust
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // Send returns an error if Receiver was
                               // already dropped
    });
```

The send method returns a `Result<T, E>` type, so if the receiving end has already been dropped and there’s nowhere to send a value, the send operation will return an error

---
### Channels - the receiver

```rust
    let received = rx.recv().unwrap(); // rx is of type Receiver<String>
```

`recv` (short for receive) will block the main thread’s execution and wait until a value is sent down the channel.

Once a value is sent, recv will return it in a `Result<T, E>`.

When the sending end of the channel closes, recv will return an error to signal that no more values will be coming.

---

### Channel - the receiver

`try_recv` method doesn’t block, but will instead return a `Result<T, E>` immediately: an `Ok` value holding a message if one is available and an `Err` value if there aren’t any messages this time.

```rust
loop {
    if let Ok(received_value) = rx.try_recv() {
        // Do something very important with received_value
    }
    else {
        // Do some other very important work
    }
}
```

---
<!-- _class: invert -->

### Channel - sending multiple values

```rust
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let vals = vec![String::from("hi"),
                    String::from("from"),
                    String::from("thread"),];
    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx { // When the channel is closed, iteration will end.               
    println!("Got: {}", received);
}
```

---


### Channel - sending multiple values from multiple threads

###### `mpsc::channel` Multiple producer, singler consumer

An `mpsc` channel sender can be cloned to allow messages from multiple threads to be received by the same receiver.

---

```rs
let (tx, rx) = mpsc::channel();
let tx1 = tx.clone();
thread::spawn(move || {
    let vals = vec![String::from("hi"), String::from("from"), String::from("thread1"),];
    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});
thread::spawn(move || {
    let vals = vec![String::from("hi"), String::from("from"), String::from("thread2"),];
    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});
for received in rx {
    println!("Got: {}", received);
}
```

---
<!-- _class: invert -->

## 16.3 Shared State Concurrency

Multiple threads able to access the same memory location at the same time.

To achieve this we need something akin to shared pointers, but for for multithreaded processes

---

## 16.3 Shared State Concurrency

How do we stop two threads trying to write to the same data at the same time if the threads are running in parallel?

How do we prevent trying to read for data which is in the process of being written to?

---
<!-- _class: invert -->

#### Shared State Concurrency: The humble mutex

Mutex (_mutual exclusion_) allows only one thread to access some data at any given time
  
To access the data in a mutex, a thread must acquire the mutex’s lock. The lock keeps track of who currently has exclusive access to the data.

A mutex **locks** a resource

---

### Mutexes

- You must attempt to acquire the lock before using the data.
- When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.

Rust's `Mutex<T>` takes care of these conditions for us, ensuring we don't accidentally leave a resource locked.

---
#### Mutex<T>

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5); // Creates a new mutex guarded resource of type Mutex<i32>

    {
        let mut num = m.lock().unwrap(); // Mutex lock must be aquired before we
        *num = 6;                        // can write to the &i32
    }

    println!("m = {:?}", m);
}
```

---

### Mutex<T>

```rust
let num_lock_result: LockResult<MutexGuard<i32>> = m.lock();
let num_mutex_guard: MutexGuard<i32> = num_lock_result.unwrap();
let mut num: i32 = *num_mutex_guard;
```

The call to lock returns a smart pointer called `MutexGuard`, wrapped in a `LockResult` that we handle with the call to `unwrap`

The `MutexGuard` smart pointer implements `Deref` to point at our inner data; it also has a `Drop` implementation that releases the lock when a `MutexGuard` goes **out of scope**

---

### Mutex scope

The lock is live as long as it is in scope (whether or not it is used)

- With brackets for scope

```txt
m = Mutex { data: 6, poisoned: false, .. }
```

- Without brackets

```txt
m = Mutex { data: <locked>, poisoned: false, .. }
```

---
<!-- _class: invert -->

### Mutexes and Threading

`Mutex` does not implement the `Copy` trait, since it implements `Drop`

Usual reference rules apply, so as with `String` and `Vector` mutexes must be moved _into_ any threads to avoid violating lifetime rules

```rust
let counter = Mutex::new(0);
let handle = thread::spawn(move || { // Move Mutex<i32> num into thread
    let mut num = counter.lock().unwrap();

    *num += 1;
});
```

---

### Mutexes and Threading

The means a mutex can only be used by one thread, so on its own a mutex locked resource cannot be used for shared state concurrency

---

### Mutexes and Threading

Simply wrapping the `Mutex` in an `Rc` (from last week - reference counted shared smart pointer) will get us slightly further

```rust
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }
```

---
#### Mutexes and Threading

```rs
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || { // ERROR
        ...
```

but still fails, this time with error:

```txt
`Rc<Mutex<i32>>` cannot be sent between threads safely
```

---

#### We can't use `Rc<T>` to share our data

- _`Rc<Mutex<i32>>` cannot be sent between threads safely_
- _the trait `Send` is not implemented for `Rc<Mutex<i32>>`_

###### Why?

When `Rc<T>` manages the reference count, it doesn't guard the count to make sure that changes to the count can’t be interrupted by another thread

---

#### We **can** use `Arc<T>` to share our data

The **a** stands for atomic, meaning it’s an **atomically reference counted type**

`Arc<T>` has the same API as `Rc` so can be straightforwardly swapped in its place

We don't use `Arc` in single threaded code because the added thread safety comes with a performance penalty

---

#### Mutexes and Threading

```rust
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap()); // Prints 10                      
```

---

#### Some Notes on Mutability

- counter is immutable **but** we could get a mutable reference to the value inside it
- `Mutex<T>` provides **interior mutability**

_see section 15.5 on `RefCell<T>` for a recap in the interior mutability pattern_

---

<!-- _class: gaia -->

##### 16.3 Shared State Concurrency

Rust's tools do not protect us against:

- **deadlocks** - processes block each other due to resource acquisition and none of the processes makes any progress as they wait for the resource held by the other process
- **livelocks** - similar to a deadlock where a process can't access a recource, but might still change state e.g. two resources both trying to avoid a deadlock by swapping resources
- **resource starvation** -  process is unable to gain regular access to the shared resources it requires to complete a task and so, unable to make any progress

---

##### 16.3 Shared State Concurrency - an example

Process some data in chunks on separate threads

```rust
let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668"; // Data

let mut children = vec![]; // Vector to hold on to thread handles             
let chunked_data = data.split_whitespace(); // Chunk data
let total = Arc::new(Mutex::new(0)); // Atomic to hold resulting total
```

---
###### 16.3 Shared State Concurrency - an example

```rust
    for (_, data_segment) in chunked_data.enumerate() {
        let total = Arc::clone(&total);

        children.push(thread::spawn(move || {
            let result:u32 = data_segment // Calculate the intermediate sum of this segment:
                        .chars()
                        .map(|c| c.to_digit(10).expect("should be a digit"))
                        .sum();

            loop { // Try to get lock - could add timeout if needed
                if let Ok(mut mutex_guard) = total.lock() {
                    *mutex_guard += result;
                    break;
                }

                thread::sleep(Duration::from_millis(50));
            }
        }));
    }
```

---

###### 16.3 Shared State Concurrency - an example

Print the final result after waiting for all the threads to finish:

```rust
for handle in children.into_iter() {
    handle.join().unwrap();
}

println!("Final sum result2: {}", *total.lock().unwrap());
```

---

### 16.4 **Sync** and **Send**

- A type is **Send** if it is safe to _send_ it to another thread.
- A type is **Sync** if it is safe to _share_ between threads (T is **Sync** if and only if `&T` is **Send**).

Not everything obeys inherited mutability. Some types allow you to have multiple aliases of a location in memory while mutating it. Unless these types use synchronization to manage this access, they are **not thread-safe**. Rust captures this through the **Send** and **Sync** traits.

---

<!-- _class: invert -->

### 16.4 Allowing Transference of Ownership Between Threads with **Send**

`Send` marker trait

- indicates that ownership of values of the type implementing `Send` can be transferred between threads
- Almost every Rust type is `Send`

---

<!-- _class: invert -->

#### 16.4 Allowing Transference of Ownership Between Threads with **Send**

- `Rc<T>` this **cannot** be `Send` because if you cloned an `Rc<T>` value and tried to transfer ownership of the clone to another thread, both threads might update the reference count at the same time
- Types composed of other `Send` types area automatically marked as `Send` as well
- Almost all primitive types are `Send`, aside from raw pointers (see Chapter 19)

---

#### 16.4 Allowing Access from Multiple Threads with **Sync**

`Sync` marker trait

- indicates that it is safe for the type implementing **Sync** to be referenced from multiple threads
- i.e. any type T is **Sync** if `&T` is **Send**, meaning the reference can be sent safely to another thread
- Primitive types are **Sync**, and types composed of types that are Sync are also Sync.

---

#### 16.4 Allowing Access from Multiple Threads with **Sync**

- `Rc<T>` is **not Sync** for the same reasons that it’s not Send
- The `RefCell<T>` type and the family of related `Cell<T>` types are **not Sync**
  - The implementation of borrow checking that `RefCell<T>` does at runtime is not thread-safe
- `Mutex<T>` **is Sync** and can be used to share access with multiple threads as you saw in the 'Sharing a `Mutex<T>` Between Multiple Threads' section.

---

#### 16.4 Allowing Access from Multiple Threads with **Sync**

`MutexGuard` is **Sync** but NOT **Send**

- The implementation of `MutexGuard` requires you to ensure you don't try to free a lock that you acquired in a different thread
- If you were able to **Send** a MutexGuard to another thread the destructor would eventgually run in the thread you sent it to, violating the requirement
- `MutexGuard` can still be Sync because all you can send to another thread is an `&MutexGuard` and dropping a reference does nothing

---

<!-- _class: lead -->
# End of chapter 16

---

## More threading!


---
<!-- _class: invert -->

# What next?

- _Chapter 17: Object Oriented Programming Features of Rust_
- _Chapter 18: Patterns and Matching_

---

<!-- _class: lead -->

### Dates coming up

23rd February - Richard Shepherd

10th February - Speaker style meetup

17th February - Fearless Concurency
