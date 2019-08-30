use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // .bind() returns a `Result<T, E>` indicating that the binding
    // could fail. Note: binding wouldn't work if we ran two instances 
    // of our program and so had two programs listening to the same port.

    for stream in listener.incoming() {

        let stream = stream.unwrap();
        // use .unwrap() to to stop the program if errors happen

        println!("Connection est.");
    }
}

