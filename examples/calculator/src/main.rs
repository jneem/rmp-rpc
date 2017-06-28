extern crate futures;
extern crate tokio_core;
extern crate rmp_rpc;

mod client;
mod server;

use client::Client;
use server::Calculator;
use tokio_core::reactor::Core;
use futures::Future;
use std::thread;
use std::time::Duration;
use std::net::SocketAddr;
use rmp_rpc::serve;

fn main() {

    let addr: SocketAddr = "127.0.0.1:54321".parse().unwrap();

    thread::spawn(move || serve(&addr, Calculator::new()));
    thread::sleep(Duration::from_millis(100));

    let mut reactor = Core::new().expect("Failed to start even loop");
    let client_future = Client::connect(&addr, &reactor.handle())
        .and_then(|mut client| {
            println!("connected");
            client
                .add(&[1, 2, 3])
                .and_then(|result| {
                    println!("{}", result);
                    Ok(client)
                })
                .or_else(|rpc_err| {
                    println!("add failed: {}", rpc_err);
                    Err(rpc_err)
                })
        })
        .and_then(|mut client| {
            client
                .sub(&[1])
                .and_then(|result| {
                    println!("{}", result);
                    Ok(client)
                })
                .or_else(|rpc_err| {
                    println!("sub failed: {}", rpc_err);
                    Err(rpc_err)
                })
        })
        .and_then(|mut client| {
            client
                .res()
                .and_then(|result| {
                    println!("{}", result);
                    Ok(client)
                })
                .or_else(|rpc_err| {
                    println!("res failed: {}", rpc_err);
                    Err(rpc_err)
                })
        })
        .and_then(|mut client| {
            client
                .clear()
                .and_then(|result| {
                    println!("{}", result);
                    Ok(client)
                })
                .or_else(|rpc_err| {
                    println!("clear failed: {}", rpc_err);
                    Err(rpc_err)
                })
        });
    let _ = reactor.run(client_future).unwrap();
}
