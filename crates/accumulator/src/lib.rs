//! A simple accumulator application.
//!
//! Each accumulator node maintains a set of strings. Upon receiving a string
//! from a client, the node adds the string to its state, and broadcast the
//! new state to other nodes in the network. All nodes eventually converge to
//! the same state, by merging received states into their own states.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::net::UdpSocket;

#[derive(Serialize, Deserialize, Debug)]
enum Message {
    FromClient(ClientMessage),
    FromServer(ServerMessage),
    Terminate,
}

/// Network configuration. Contains a list of server addresses.
#[derive(Debug, Clone)]
struct Configuration {
    server_addrs: Vec<String>,
}

/// Client message type for the accumulator application. Each message contains
/// a string.
#[derive(Serialize, Deserialize, Debug)]
struct ClientMessage {
    item: String,
}

/// the current node state, which is a set of strings.
#[derive(Serialize, Deserialize, Debug)]
struct ServerMessage {
    state: HashSet<String>,
}

/// A client node for the accumulator application.
struct Client {
    socket: UdpSocket,
    config: Configuration,
}

impl Client {
    /// Create a new client
    fn new(addr: &str, config: &Configuration) -> Self {
        let s = UdpSocket::bind(addr).unwrap();
        Self {
            socket: s,
            config: config.clone(),
        }
    }

    /// Disseminate a string to the accumulator network.
    fn disseminate(&mut self, item: &str) {
        let msg = Message::FromClient(ClientMessage {
            item: String::from(item),
        });
        self.socket
            .send_to(
                serde_json::to_string(&msg).unwrap().as_bytes(),
                &self.config.server_addrs[0],
            )
            .unwrap();
    }

    /// Terminate a running accumulator server.
    fn terminate(&mut self, addr: &str) {
        let msg = Message::Terminate;
        self.socket
            .send_to(serde_json::to_string(&msg).unwrap().as_bytes(), addr)
            .unwrap();
    }
}

/// An accumulator server node. Each node maintains a UDP socket, and a set of
/// strings as its internal state.
struct Server {
    config: Configuration,
    addr: String,
    socket: UdpSocket,
    state: HashSet<String>,
    running: bool,
}

impl Server {
    /// Create a new node.
    fn new(addr: &str, config: &Configuration) -> Self {
        let s = UdpSocket::bind(addr).unwrap();
        Self {
            config: config.clone(),
            addr: String::from(addr),
            socket: s,
            state: HashSet::new(),
            running: false,
        }
    }

    /// Handle a message
    fn handle_msg(&mut self, msg: Message) {
        match msg {
            Message::FromClient(msg) => {
                let s = HashSet::from_iter(vec![msg.item]);
                self.merge(s);
            }
            Message::FromServer(msg) => {
                self.merge(msg.state);
            }
            Message::Terminate => {
                self.running = false;
            }
        }
    }

    /// Merge a state into the current state. If the state changes, broadcast
    /// the new state.
    fn merge(&mut self, state: HashSet<String>) {
        let old_size = self.state.len();
        self.state.extend(state);
        if self.state.len() > old_size {
            self.broadcast(Message::FromServer(ServerMessage {
                state: self.state.clone(),
            }));
        }
    }

    /// Broadcast message to all other nodes in the network.
    fn broadcast(&mut self, msg: Message) {
        for addr in &self.config.server_addrs {
            if self.addr.ne(addr) {
                self.socket
                    .send_to(serde_json::to_string(&msg).unwrap().as_bytes(), addr)
                    .unwrap();
            }
        }
    }

    /// Main event loop.
    fn run(&mut self) {
        self.running = true;
        while self.running {
            let mut buf = [0; 1500];
            let (n, _) = self.socket.recv_from(&mut buf).unwrap();
            let msg: Message = serde_json::from_str(&String::from_utf8_lossy(&buf[..n])).unwrap();
            self.handle_msg(msg);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time;

    use super::*;

    fn setup(n_server: usize, n_client: usize) -> (Configuration, Vec<String>) {
        let mut server_addrs = Vec::new();
        let mut client_addrs = Vec::new();
        for i in 0..n_server {
            server_addrs.push(format!("127.0.0.1:{}", 5000 + i));
        }
        for i in 0..n_client {
            client_addrs.push(format!("127.0.0.1:{}", 5000 + n_server + i));
        }
        let config = Configuration {
            server_addrs: server_addrs,
        };
        (config, client_addrs)
    }

    fn terminate(config: &Configuration) {
        let mut client = Client::new("127.0.0.1:8000", &config);
        for addr in config.server_addrs.iter() {
            client.terminate(&addr);
        }
    }

    #[test]
    fn single_server() {
        let (config, client_addrs) = setup(1, 1);
        // Start server
        let c = config.clone();
        let handle = std::thread::spawn(move || {
            let mut server = Server::new(&c.server_addrs[0], &c);
            server.run();
            server.state
        });
        // Run client
        let mut client = Client::new(&client_addrs[0], &config);
        client.disseminate("hello");
        // End test
        thread::sleep(time::Duration::from_millis(100));
        terminate(&config);
        let state = handle.join().unwrap();
        assert!(state.contains("hello"));
    }

    #[test]
    fn multi_servers() {
        let (config, client_addrs) = setup(3, 1);
        // Start servers
        let mut handles = Vec::new();
        for i in 0..3 {
            let c = config.clone();
            handles.push(std::thread::spawn(move || {
                let mut server = Server::new(&c.server_addrs[i], &c);
                server.run();
                server.state
            }));
        }
        // Run client
        let mut client = Client::new(&client_addrs[0], &config);
        client.disseminate("hello");
        // End test
        thread::sleep(time::Duration::from_millis(100));
        terminate(&config);
        let states = handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .collect::<Vec<_>>();
        assert!(states.iter().all(|s| s.contains("hello")));
    }
}
