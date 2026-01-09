use std::io::Write;
use std::{env, io};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;

const MAX: u16 = 65535;

struct Arguments{
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str>{
        if args.len() < 2 {
            return Err("not enough args use -h/--help for help");
        } else if args.len() > 4 {
            return Err("too much args");
        }
        
        let first_arg = &args[1];
        let flag = args[1].clone();

        if args.len() == 2 {
            if let Ok(ipaddr) = IpAddr::from_str(first_arg){
                return Ok(Arguments {ipaddr, threads: 4});
            }
            if matches!(flag.as_str(), "-h" | "--help") {
                println!("Usage: -j/--threads to set amount of threads to use\n -h/-help to show this help msg");
                return Err("help");
            }
            return Err("wrong usage use -h/--help for help")
        }

        if matches!(flag.as_str(), "-j" | "--threads"){
            if args.len() != 4 {
                return Err("invalid syntax: use -j <threads> <ipaddr>");
            }
            
            let threads = args[2].parse::<u16>()
                .map_err(|_| "failed to parse thread number")?;

            if threads == 0 {
                return Err("threads must be greater than 0");
            }
            
            let ipaddr = IpAddr::from_str(&args[3])
                .map_err(|_| "not a valid IPADDR: must be ipv4 or v6")?;
            
            return Ok(Arguments { 
                ipaddr, 
                threads 
            });
        }
        Err("invalid syntax. Use -h for help")
    }
    
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16){
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)){
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err == "help" {
                process::exit(0);
            } else {
                eprintln!("{} problem parsing args: {}", program, err);
                process::exit(1);
            }
        }
    );
    
    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();
    
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, addr, num_threads); 
        });
    }
    
    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }
    
    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}

