use ferris_says::say;

fn main() {
    let _setup = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer  = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}