use std::env;
#[macro_use]
extern crate log;

mod tcp_client;
mod tcp_server;
mod udp_client;
mod udp_server;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let pname: &str = &args[0];
    if args.len() != 4 {
        usage(pname);
    }

    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let addr: &str = &args[3];
    match protocol {
        "tcp" => match role {
            "server" => {
                tcp_server::serve(addr).unwrap_or_else(|e| error!("{}", e));
            }
            "client" => {
                tcp_client::connect(addr).unwrap_or_else(|e| error!("{}", e));
            }
            _ => {
                usage(pname);
            }
        },
        "udp" => match role  {
            "server" => {
                udp_server::serve(addr).unwrap_or_else(|e| error!("{}", e));
            }
            "client" => {
                udp_client::communicate(addr).unwrap_or_else(|e| error!("{}", e));
            }
            _ => {
                usage(pname);
            }
        },
        _ => {
            usage(pname);
        }
    }
}


fn usage(name: &str) {
    error!("Usage: {} [tcp|udp] [serer|client] [addr|port]", name);
    std::process::exit(1);
}
