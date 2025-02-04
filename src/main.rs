use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use std::{net::{IpAddr, SocketAddr}, time::Instant};
use rand::seq::SliceRandom;
use rand::thread_rng;
use chrono::Local;

extern crate chrono;

fn print_banner() {
    let date = Local::now();//loading local time with chrono
    println!("Port scanner made with rust");
    println!("Author by Martin :>");
    println!("Porteiest starting on: {}", date.format("%Y-%m-%d / %H:%M:%S"));//showing excuted date
}


async fn send_request(ip: IpAddr, port: u16) -> bool {//returning bool
    let addr = SocketAddr::new(ip, port);//ctreating addr for send a tcp request
    match TcpStream::connect(&addr).await {
        Ok(_) => true,//if ok then return true
        Err(_) => false,//if Err then return false
    }
}

fn get_result(ip: IpAddr) {
    let rt = Runtime::new().expect("Unexpected Error");
    let mut numbers: Vec<u16> = (0..6500).collect();//using I've tried it up to 65536 but size didn't matched and I can't fix it

    let mut rng = thread_rng();

    numbers.shuffle(&mut rng);//shuffling the numbers (0 to 6500)

    for port in numbers { //sending tcp request to every ports in numbers array
        rt.block_on(async {
            if send_request(ip, port).await {
                println!("Port {} is open", port);
            }
        });
    }
}

fn main() {
    let start = Instant::now();//counting excution time

    print_banner();
    let ip: IpAddr = "127.0.0.1".parse().expect("Invalid IP Address"); 
    get_result(ip);

    let finish = start.elapsed();//end checking
    println!("Finished at : {:?}", finish);//showing the excution time
}
