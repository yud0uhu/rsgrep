use std::fs::read_to_string;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "rsgrep")]
struct GrepArgs {
    #[structopt(name = "PATTERN")]
    pattern: String,
    #[structopt(name = "PATTERN")]
    path: Vec<String>,
}

fn grep(content: String, pattern: &String) {
    for line in content.lines() {
        // as_strで&str型に変換している
        // line == pattern.st_str()などと比較してもいいが
        // 等価性を図る型の場合はその方がいい
        if line.contains(pattern.as_str()) {
            println!("{}", line);
        }
    }
}

fn run(state: GrepArgs) {
    for path in state.path.iter() {
        match read_to_string(path) {
            // なぜここでコンパイルエラーが起こるのか？
            // RustにはGCがなく、リソースの開放は自動で行われる
            // forループでmoveが起こり、所有権がなくなっていた
            Ok(content) => grep(content, &state.pattern),
            Err(reason) => println!("{}", reason),
        }
    }
}

fn main() {
    // // pattern, pathはタプルと呼ばれる文法
    // let pattern = std::env::args().nth(1);
    // let path = std::env::args().nth(2);
    // match (pattern, path) {
    //     // Someの中のpatternを取り出す
    //     (Some(pattern), Some(path)) => run(GrepArgs::new(path, pattern)),
    //     _ => println!("No path is specified!"),
    // }
    run(GrepArgs::from_args());
}
