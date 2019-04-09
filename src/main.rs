use std::env;

fn run<'a>() -> Result<(), &'a str> {
    if env::args().len() != 2 {
        Err("引数の個数が正しくありません")
    } else {
        let n_parsed: i64;
        let res = env::args().nth(1).unwrap().parse::<i64>();
        if res.is_err() {
            Err("ソースコードをパースできません")
        } else {
            n_parsed = res.unwrap();

            println!(".intel_syntax noprefix");
            println!(".global main");
            println!("main:");
            println!("  mov rax, {}", n_parsed);
            println!("  ret");

            Ok(())
        }
    }
}

fn main() {
    ::std::process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("Error: {:?}", err);
            1
        }
    });
}
