use turbo_json_checker::JsonChecker;
use std::io;

fn fmain() -> io::Result<()> {
    let stdin = io::stdin();
    let mut checker = JsonChecker::new(stdin.lock());
    io::copy(&mut checker, &mut io::sink())?;
    let outer_type = checker.finish()?;
    println!("{:?}", outer_type);
    Ok(())
}

fn main() {
    if let Err(e) = fmain() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
