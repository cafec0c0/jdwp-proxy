mod constants;

use crate::constants::header_to_string;
use colored::Colorize;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener, TcpStream};

struct JdwpProxy {
    debuggee_port: u16,
    debugger_port: u16,
}

impl JdwpProxy {
    pub fn new(debuggee_port: u16, debugger_port: u16) -> Self {
        JdwpProxy {
            debuggee_port,
            debugger_port,
        }
    }

    pub async fn start_proxy(&self) {
        let mut debuggee_socket = Self::connect_to_debuggee(self.debuggee_port).await;
        Self::debuggee_handshake(&mut debuggee_socket).await;

        let mut debugger_socket = Self::accept_debugger_connection(self.debugger_port).await;
        Self::debugger_handshake(&mut debugger_socket).await;

        // These need to use into_split() as they will be shared across different threads
        let (debuggee_read, debuggee_write) = debuggee_socket.into_split();
        let (debugger_read, debugger_write) = debugger_socket.into_split();

        let h1 = tokio::spawn(async {
            Self::intercept_debuggee_messages(debuggee_read, debugger_write).await;
        });

        let h2 = tokio::spawn(async {
            Self::intercept_debugger_messages(debugger_read, debuggee_write).await;
        });

        h1.await.unwrap();
        h2.await.unwrap();
        println!("here");
    }

    pub async fn connect_to_debuggee(port: u16) -> TcpStream {
        println!("Connecting to debuggee");
        TcpStream::connect(format!("127.0.0.1:{}", port))
            .await
            .unwrap()
    }

    pub async fn accept_debugger_connection(port: u16) -> TcpStream {
        println!("Waiting for debugger to connect");
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
            .await
            .unwrap();
        listener.accept().await.unwrap().0
    }

    pub async fn debuggee_handshake(debuggee_socket: &mut TcpStream) {
        println!("Exchanging debuggee handshake");
        let (mut debuggee_read, mut debuggee_write) = debuggee_socket.split();
        let handshake = b"JDWP-Handshake";
        debuggee_write.write(handshake).await.unwrap();
        let mut handshake_buf = [0u8; 14];
        debuggee_read.read(&mut handshake_buf).await.unwrap();
    }

    pub async fn debugger_handshake(debugger_socket: &mut TcpStream) {
        println!("Exchanging debugger handshake");
        let (mut debugger_read, mut debugger_write) = debugger_socket.split();
        let mut handshake_buf = [0u8; 14];
        debugger_read.read(&mut handshake_buf).await.unwrap();
        let handshake = b"JDWP-Handshake";
        debugger_write.write(handshake).await.unwrap();
    }

    pub async fn intercept_debuggee_messages(
        mut debuggee_read: OwnedReadHalf,
        mut debugger_write: OwnedWriteHalf,
    ) {
        println!("Starting debuggee interceptor");
        loop {
            let mut header_buf = [0u8; 11];
            debuggee_read.read_exact(&mut header_buf).await.unwrap();
            debugger_write.write(header_buf.as_slice()).await.unwrap();
            println!(
                "{}: {}",
                "Debugger <- Debuggee".green(),
                header_to_string(&header_buf)
            );

            let len = i32::from_be_bytes(header_buf[..4].try_into().unwrap());
            let remaining_length = (len - 11) as usize;

            if remaining_length > 0 {
                let mut remaining_buf = vec![0; remaining_length];
                debuggee_read.read_exact(&mut remaining_buf).await.unwrap();
                debugger_write
                    .write(remaining_buf.as_slice())
                    .await
                    .unwrap();
            }
        }
    }

    pub async fn intercept_debugger_messages(
        mut debugger_read: OwnedReadHalf,
        mut debuggee_write: OwnedWriteHalf,
    ) {
        println!("Starting debugger interceptor");
        loop {
            let mut header_buf = [0u8; 11];
            debugger_read.read_exact(&mut header_buf).await.unwrap();
            debuggee_write.write(header_buf.as_slice()).await.unwrap();
            println!(
                "{}: {}",
                "Debugger -> Debuggee".blue(),
                header_to_string(&header_buf)
            );

            let len = i32::from_be_bytes(header_buf[..4].try_into().unwrap());
            let remaining_length = (len - 11) as usize;

            if remaining_length > 0 {
                let mut remaining_buf = vec![0; remaining_length];
                debugger_read.read_exact(&mut remaining_buf).await.unwrap();
                debuggee_write
                    .write(remaining_buf.as_slice())
                    .await
                    .unwrap();
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let proxy = JdwpProxy::new(8000, 8001);
    proxy.start_proxy().await;
}
