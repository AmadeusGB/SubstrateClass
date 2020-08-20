//import thread package 
use std::thread;
//import net package
use std::net::{TcpListener, TcpStream, Shutdown};
//import io package
use std::io::{Read, Write};

//define handle_client function to handle tcp request
fn handle_client(mut stream: TcpStream) {
    // using 50 byte buffer
    let mut data = [0 as u8; 50]; 
    //read data to match
    while match stream.read(&mut data) {
        //match Ok(size)
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            //return true
            true
        },
        //match Err
        Err(_) => {
            //print error information
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            //shutdown thread
            stream.shutdown(Shutdown::Both).unwrap();
            //return false
            false
        }
    } {}
}

fn main() {
    //create listener,and bind localhost:3333
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    //accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    //read listener request context
    for stream in listener.incoming() {
        //to match type and information
        match stream {
            //read info to handle
            Ok(stream) => {
                //print connection info,include ip and port
                println!("New connection: {}", stream.peer_addr().unwrap());
                //spawing a new thread to handle this request
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            //read Err to expose
            Err(e) => {
                //print error information
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
