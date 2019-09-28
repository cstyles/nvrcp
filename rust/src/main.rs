use neovim_lib::{Neovim, NeovimApi, Session, Value};
use std::io::{self, Read};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut nvim = connect_to_nvim();

    if args.len() < 2 {
        getreg(&mut nvim);
    } else {
        match args[1].to_ascii_lowercase().as_str() {
            "s" | "set" | "setreg"     => setreg(&mut nvim),
            "g" | "get" | "getreg" | _ => getreg(&mut nvim),
        }
    }
}

fn getreg(nvim: &mut Neovim) {
    let args = vec![Value::String("\"".into())];
    let output = nvim.call_function("getreg", args).unwrap().to_string();

    // Remove surrounding double quotes
    let trimmed_output = output.get(1..output.len() - 1).unwrap();
    print!("{}", trimmed_output);
}

fn setreg(nvim: &mut Neovim) {
    let args = vec![
        Value::String("\"".into()),
        Value::String(read_from_stdin().into()),
    ];

    nvim.call_function("setreg", args).unwrap();
}

fn connect_to_nvim() -> Neovim {
    let listen_address = env::var("NVIM_LISTEN_ADDRESS").unwrap();
    let mut session = Session::new_unix_socket(listen_address).unwrap();
    session.start_event_loop();
    Neovim::new(session)
}

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handler = stdin.lock();
    handler.read_to_string(&mut buffer).unwrap();
    buffer
}
