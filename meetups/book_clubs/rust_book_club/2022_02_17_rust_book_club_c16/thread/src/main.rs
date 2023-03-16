use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};

fn main() {
    // _thread_func();
    // _channel_func();
    // _channel_func2();
    // _channel_func3();
    // _mutex_func_scoped();
    // _mutex_func();
    // _mutex_threads_func();
    _chunk_func();
    _chunk_atomic_func();
}

fn _thread_func() {
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

    handle.join().unwrap();

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn _channel_func() {
    let (tx, rx) = mpsc::channel(); // multiple producer, single consumer

    let handle = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();

        let txc = tx.clone();
        let val = String::from("hi");
        txc.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    
    handle.join().unwrap();
}

fn _channel_func2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        //println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn _channel_func3() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

}

fn _mutex_func_scoped() {
    let m = Mutex::new(5); // Creates a new mutex guarded resource of type Mutex<i32>

    {
        let mut num = m.lock().unwrap(); // Mutex lock must be aquired before we
        *num = 6;                                      // can write to the &i32
    }

    println!("m = {:?}", m);
}

fn _mutex_func_unscoped() {
    let m = Mutex::new(5); // Creates a new mutex guarded resource of type Mutex<i32>

    let mut num = m.lock().unwrap(); // Mutex lock must be aquired before we
    *num = 6;                                      // can write to the &i32

    println!("m = {:?}", m);
}

fn _mutex_threads_func() {
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

    println!("Result: {}", *counter.lock().unwrap());
}

fn _chunk_func() {
    // This is our data to process.
    // We will calculate the sum of all digits via a threaded  map-reduce algorithm.
    // Each whitespace separated chunk will be handled in a different thread.
    //
    // TODO: see what happens to the output if you insert spaces!
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    // Make a vector to hold the child-threads which we will spawn.
    let mut children = vec![];

    /*************************************************************************
     * "Map" phase
     *
     * Divide our data into segments, and apply initial processing
     ************************************************************************/

    // split our data into segments for individual calculation
    // each chunk will be a reference (&str) into the actual data
    let chunked_data = data.split_whitespace();

    // Iterate over the data segments.
    // .enumerate() adds the current loop index to whatever is iterated
    // the resulting tuple "(index, element)" is then immediately
    // "destructured" into two variables, "i" and "data_segment" with a
    // "destructuring assignment"
    for (_, data_segment) in chunked_data.enumerate() {

        // Process each data segment in a separate thread
        //
        // spawn() returns a handle to the new thread,
        // which we MUST keep to access the returned value
        //
        // 'move || -> u32' is syntax for a closure that:
        // * takes no arguments ('||')
        // * takes ownership of its captured variables ('move') and
        // * returns an unsigned 32-bit integer ('-> u32')
        //
        // Rust is smart enough to infer the '-> u32' from
        // the closure itself so we could have left that out.
        //
        // TODO: try removing the 'move' and see what happens
        children.push(thread::spawn(move || -> u32 {
            // Calculate the intermediate sum of this segment:
            let result = data_segment
                        // iterate over the characters of our segment..
                        .chars()
                        // .. convert text-characters to their number value..
                        .map(|c| c.to_digit(10).expect("should be a digit"))
                        // .. and sum the resulting iterator of numbers
                        .sum();

            result

        }));
    }

    /*************************************************************************
     * "Reduce" phase
     *
     * Collect our intermediate results, and combine them into a final result
     ************************************************************************/

    // combine each thread's intermediate results into a single final sum.
    //
    // we use the "turbofish" ::<> to provide sum() with a type hint.
    //
    // TODO: try without the turbofish, by instead explicitly
    // specifying the type of final_result
    let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();

    println!("Final sum result: {}", final_result);
}


fn _chunk_atomic_func() {
        // This is our data to process.
    // We will calculate the sum of all digits via a threaded  map-reduce algorithm.
    // Each whitespace separated chunk will be handled in a different thread.
    //
    // TODO: see what happens to the output if you insert spaces!
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    // Make a vector to hold the child-threads which we will spawn.
    let mut children = vec![];

    let chunked_data = data.split_whitespace();

    let total = Arc::new(Mutex::new(0));

    for (_, data_segment) in chunked_data.enumerate() {
        let total = Arc::clone(&total);

        children.push(thread::spawn(move || {
            // Calculate the intermediate sum of this segment:
            let result:u32 = data_segment
                        // iterate over the characters of our segment..
                        .chars()
                        // .. convert text-characters to their number value..
                        .map(|c| c.to_digit(10).expect("should be a digit"))
                        // .. and sum the resulting iterator of numbers
                        .sum();

            loop {
                if let Ok(mut mutex_guard) = total.lock() {
                    *mutex_guard += result;
                    break;
                }

                thread::sleep(Duration::from_millis(50));
            }
        }));
    }

    for handle in children.into_iter() {
        handle.join().unwrap();
    }

    println!("Final sum result2: {}", *total.lock().unwrap());

}