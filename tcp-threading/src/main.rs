use std::net::TcpListener;
use std::time::Duration;

fn main() {
    let t1 = std::thread::spawn(|| {
        println!("Starting Server .......");
        let listner = TcpListener::bind("0.0.0.0:9000").unwrap();
        println!("Starting Listner .......");
        let (stream, socket) = listner.accept().unwrap();
        println!("Recieved steram {:?}, socket {:?}", stream, socket);
        stream
    });

    std::thread::spawn(|| {
        let mut tick: u64 = 1;
        loop {
            println!("Server is working {} ...", tick);
            std::thread::sleep(Duration::from_secs(2));
            tick += 1;
        }
    });

    let stream = t1.join().expect("There was and error reading a stream");
}
