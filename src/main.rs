use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use std::{net::{IpAddr, SocketAddr}, time::Instant};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::env;
use chrono::Local;



extern crate chrono;

fn usage(){
    println!("Usage:");
    println!("     -scan <IP> to scan up to 65500ports");
    println!("     -ip <IP> <start_port> <end_port>");
}

fn print_banner() {
    let date = Local::now();//loading local time with chrono
    println!("Port scanner made with rust");
    println!("Author by Martin :>");
    println!("Porteiest starting on: {}", date.format("%Y-%m-%d / %H:%M:%S"));//showing excuted date
}


async fn send_request(ip: IpAddr, port:u16 ) -> bool {//returning bool
    let addr = SocketAddr::new(ip, port);//ctreating addr for send a tcp request
    match TcpStream::connect(&addr).await {
        Ok(_) => true,//if ok then return true
        Err(_) => false,//if Err then return false
    }
}

fn get_result(ip: IpAddr) {
    let rt = Runtime::new().expect("Unexpected Error");
    let mut numbers: Vec<u16> = (0..65500).collect();//using I've tried it up to 65536 but size didn't matched and I can't fix it

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



fn custom_get_result(ip: IpAddr, start_prt: u16 , end_port: u16) {
    let rt = Runtime::new().expect("Unexpected Error");
    let mut numbers: Vec<u16> = (start_prt..end_port).collect();//using I've tried it up to 65536 but size didn't matched and I can't fix it

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
    let args: Vec<String> = env::args().collect(); 
    let start = Instant::now();//counting excution time
    if args.len() > 1 && args[1] == "-scan" {
        let ipadd: &String = &args[2];
        let ip: IpAddr = ipadd.parse().expect("Invalid IP Address"); 
        get_result(ip);
    }
    else if args.len() > 1 && args[1] == "-ip" {
        let ipadd: &String = &args[2];
        let ip: IpAddr = ipadd.parse().expect("Invalid IP Address"); 
        let start_port: &String = &args[3];
        let end_prt: &String = &args[4];
        let start_prt: u16 = start_port.parse().unwrap();
        let end_port: u16 = end_prt.parse().unwrap();
        custom_get_result(ip, start_prt, end_port);
    }
//     else if args.len() > 1 && args[1] == "-v" {
//         let ipadd: &String = &args[2];
//         let ip: IpAddr = ipadd.parse().expect("Invalid IP Address"); 
//         let prt: &String = &args[3];
//         let port: u16 =  prt.parse().expect("Unexpected Error");
//         let rn = Runtime::new().expect("");
//         rn.block_on(grab_information(ip, port));
    else {
        usage();
    }

    print_banner();

    let finish = start.elapsed();//end checking
    println!("Finished at : {:?}", finish);//showing the excution time
}
