use rayon::prelude::*;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

fn main() {
    // Start simple
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("In child thread `{}`", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("In Parent thread `{}`", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();

    // passing data to threads
    let v = vec![1, 2, 3, 4];
    let handle = thread::spawn(move || {
        println!("vector: {:?}", v);
    });
    handle.join().unwrap();

    // quiz code (n is copied not moved)
    let mut n = 1;
    let t = thread::spawn(move || {
        n = n + 1;
        thread::spawn(move || {
            n = n + 1;
        })
    });
    n = n + 1;
    t.join().unwrap().join().unwrap();
    println!("{n}");

    // -----------------------------------------------------------
    // Channels: MPSC (Multiple Producers Single Consumer)
    // use std::sync::mpsc;
    // * Channels may have multiple senders and a single receiver.
    // -----------------------------------------------------------
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // recv is blocking it will block the code untial a value is recieved (forever)
    let recieved = rx.recv().unwrap();
    println!("Received: `{}`", recieved);
    handle.join().unwrap();

    // using the non-blocking verision try_recv
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let val = String::from("hi2");
        println!("Sending: `{}`", val);
        tx.send(val).unwrap();
    });

    // recv is blocking it will block the code untial a value is recieved (forever)
    let mut step = 0;
    let data = loop {
        match rx.try_recv() {
            Err(e) => {
                step += 1;
                println!("We are waiting for recieving data ({}) with: {:?}", step, e);
            }
            Ok(d) => break d,
        }
    };
    println!("Recieved data: `{}`", data);
    handle.join().unwrap();

    // ------------------------------------------------------------------------------
    // Sending multiple messages from multiple producers
    // ------------------------------------------------------------------------------
    let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

    // cloning the channel but we have to send the same datatype
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

        // tx.send(3).unwrap(); // BUG: mismatch type as we are sending String
    });

    for received in rx {
        println!("Got: {received}");
    }

    // --------------------------------------------------------------------
    // Mutex (Mutual Exclusion)
    // --------------------------------------------------------------------
    let x = Mutex::new(3);
    {
        let mut num = x.lock().unwrap();
        *num += 4;
    }
    println!("x={:?}", x);

    // --------------------------------------------------------------------------
    // Rc + Mutex but for thread safe: Arc + Mutex multible mutable access to the same data
    // --------------------------------------------------------------------------
    let counter = Arc::new(Mutex::new(0));
    let mut handle = Vec::new();
    for _ in 0..10 {
        let ptr = Arc::clone(&counter);
        let h = thread::spawn(move || {
            let mut ptr = ptr.lock().unwrap();
            *ptr += 1;
        });
        handle.push(h);
    }
    // waiting for threads to finish
    for h in handle {
        h.join().unwrap();
    }
    println!("n={}", *counter.lock().unwrap());

    fn multiply_vecs(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
        let v_len = v1.len();
        let v1_ptr = Arc::new(Mutex::new(v1));
        let v2_ptr = Arc::new(Mutex::new(v2));
        let res_ptr = Arc::new(Mutex::new(vec![0; v_len]));
        let mut handle = Vec::new();
        for idx in 0..v_len {
            let ptr1 = Arc::clone(&v1_ptr);
            let ptr2 = Arc::clone(&v2_ptr);
            let r_ptr = Arc::clone(&res_ptr);
            let h = thread::spawn(move || {
                let x1 = &ptr1.lock().unwrap()[idx];
                let x2 = &ptr2.lock().unwrap()[idx];
                r_ptr.lock().unwrap()[idx] = *x1 * *x2;
            });
            handle.push(h);
        }
        for h in handle {
            h.join().unwrap();
        }
        res_ptr.lock().unwrap().clone()
    }

    fn multiply_vecs_rayon(v1: &[i32], v2: &[i32]) -> Vec<i32> {
        v1.par_iter().zip(v2).map(|(a, b)| a * b).collect()
    }
    fn multiply_vecs_scope(v1: &[i32], v2: &[i32]) -> Vec<i32> {
        assert_eq!(v1.len(), v2.len(), "vectors must be same length");
        let n = v1.len();
        let mut res = vec![0; n];
        if n == 0 {
            return res;
        }
        // one chunk per core (never more chunks than elements)
        let thread_count = thread::available_parallelism()
            .map(|c| c.get())
            .unwrap_or(1)
            .min(n);
        let chunk_size = (n + thread_count - 1) / thread_count;
        thread::scope(|s| {
            for (r, (a, b)) in res
                .chunks_mut(chunk_size)
                .zip(v1.chunks(chunk_size).zip(v2.chunks(chunk_size)))
            {
                // each thread owns a disjoint slice -> no Mutex, no Arc, no clone
                s.spawn(move || {
                    for (slot, (x, y)) in r.iter_mut().zip(a.iter().zip(b)) {
                        *slot = *x * *y;
                    }
                });
            }
        }); // scope automatically joins all threads here
        res
    }
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![1, 2, 3, 4, 5];
    let v_out = multiply_vecs_rayon(&v1, &v2);
    println!("v_out: {:?}", v_out);
    let v_out = multiply_vecs_scope(&v1, &v2);
    println!("v_out: {:?}", v_out);

    // ------------------------------------------------------------
    // thread::scope
    // Unlike non-scoped threads, scoped threads can borrow non-'static data, as the
    // scope guarantees all threads will be joined at the end of the scope.
    // ------------------------------------------------------------
    //
    let mut a = vec![1, 2, 3];
    let mut x = 0;
    thread::scope(|s| {
        s.spawn(|| {
            println!("hello from the first scoped thread");
            // We can borrow `a` here.
            dbg!(&a);
        });
        s.spawn(|| {
            println!("hello from the second scoped thread");
            // We can even mutably borrow `x` here,
            // because no other threads are using it.
            x += a[0] + a[2];
        });
        println!("hello from the main thread");
    });
    // After the scope, we can modify and access our variables again:
    a.push(4);
    assert_eq!(x, a.len());

    // another example
    let v1 = vec![1, 2, 3, 4];
    let v2 = vec![1, 2, 3, 4];
    let mut res = vec![0; v1.len()];
    thread::scope(|s| {
        // v1.iter() creating a ref over the variables so we are moving the ref itself not the
        // entier element
        for (r, (a, b)) in res.iter_mut().zip(v1.iter().zip(v2.iter())) {
            s.spawn(move || {
                *r = a * b;
            });
        }
    });
    println!("res={:?}", res);
}
