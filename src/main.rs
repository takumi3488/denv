use regex::Regex;
use std::{
    env,
    fs::File,
    io::Read,
    process::{Command, Stdio},
};

fn main() {
    // コマンドを取得
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("command must be inputted");
        return;
    }
    let command = &args[1];
    let command_args = &args[2..].to_vec();

    // 環境変数を取得
    let re = Regex::new("([A-Z_]+)[\"\']?=[\"\']?([^\"\']*)[\"\']?").unwrap();
    let mut f = File::open(".env").expect("`.env` not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the .env file");
    let envs: Vec<(String, String)> = contents
        .lines()
        .map(|line| re.captures(line))
        .filter(|line| line.is_some())
        .map(|line| {
            let caps = line.unwrap();
            (caps[1].to_string(), caps[2].to_string())
        })
        .collect();

    // コマンドを実行
    Command::new(command)
        .args(command_args)
        .envs(envs)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed");
}
