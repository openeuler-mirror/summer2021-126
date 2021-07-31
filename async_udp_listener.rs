/** UDP listener **/
use smol::{Async, io};
use async_net::UdpSocket;
use std::net::SocketAddr;

async fn listen(socket: UdpSocket, mut buffer: &mut [u8]) 
    -> SocketAddr {
    
    let (n_bytes, src_addr) = socket.recv_from(&mut buffer)
                                    .expect("no data received").await?;
    // recv_from() receives a single datagram message, number of bytes and the origin
    // this method must be called with a valid byte slice of sufficient size to hold the message
    println!("Received {:?} bytes from {:?}", n_bytes, src_addr);
    src_addr
}

async fn send(socket: UdpSocket, receiver: SocketAddr, msg: &Vec<u8>) 
    -> io::Result<usize> {

    println!("sending data");
    let result = socket.send_to(msg, receiver)
                       .expect("failed to send message").await?;
    Ok(result)
}

fn main() -> io::Result<()> {
    smol::block_on(async {
        let mut buffer: Vec<u8> = Vec::with_capacity(100);
        let socket = Async::<UdpSocket>::bind("127.0.0.1:0")?; // requested the port from the os
        println!("the socket is bound to {}", socket.local_addr()?);
        
        let message = String::from("ACK received");
        let msg_bytes = message.into_bytes();
            
        loop {
            let src_addr = listen(&socket, &mut buffer);
            smol::spawn(send(&socket, src_addr, &msg_bytes)).detach();
        }
    })
}