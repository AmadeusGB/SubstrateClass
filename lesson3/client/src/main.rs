//import net package
use std::net::{TcpStream};
//import io package
use std::io::{Read, Write};
//import str package
use std::str::from_utf8;

fn main() {
    //to match localhost:3333 request
    match TcpStream::connect("localhost:3333") {
        //read stream to handle
        Ok(mut stream) => {
            //pinrt success information
            println!("Successfully connected to server in port 3333");
            //define msg = "Hello!"
            let msg = b"Hello!";
            //write msg to stream
            stream.write(msg).unwrap();
            //print prompt information
            println!("Sent Hello, awaiting reply...");
            //define data,using 6 byte buffer
            let mut data = [0 as u8; 6]; 
            //match data 
            match stream.read_exact(&mut data) {
                //Ok,to handle it
                Ok(_) => {
                    //judge msg 
                    if &data == msg {
                        //print Reply
                        println!("Reply is ok!");
                    } else {
                        //convert utf8 to text
                        let text = from_utf8(&data).unwrap();
                        //print text
                        println!("Unexpected reply: {}", text);
                    }
                },
                //Error,to handle it
                Err(e) => {
                    //print data match Error information
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        //Error to handle it
        Err(e) => {
            //print stream Error information
            println!("Failed to connect: {}", e);
        }
    }
    //end connection,print prompt information
    println!("Terminated.");
}

