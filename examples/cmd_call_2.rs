#![allow(unused)]
use std::io::{stdin, BufRead, BufReader, BufWriter, Error, ErrorKind, Write};
use std::process::{ChildStdin, Command, Stdio};

use nar_dev_utils::show;

const PROGRAM: &str = r"H:\A137442\Develop\AGI\NARS\NARS-executables\NAR.exe";

fn main() -> Result<(), Error> {
    let mut child = Command::new(PROGRAM)
        .arg("shell")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?;

    // 写入输入
    for _ in 0..1 {
        let take = child
            .stdin.take();
        // if take.is_none() {
        //     child.stdin.insert(ChildStdin::from(Stdio::piped()));
        // }
        let stdin = take
            .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard input."))?;
        let mut writer = BufWriter::new(stdin);
        let mut write = |s: &str| {
            writer
                .write_all(s.as_bytes())
                .expect("无法写入输入");

        };
        write("<A --> B>.\n");
        write("<B --> C>.\n");
        write("<A --> C>?\n");
    }

    // show!(child.stdin);
    let stdout = child
        .stdout.take()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    // 读取输出
    let reader = BufReader::new(stdout);
    reader
        // 获取输出的行
        .lines()
        .map_while(Result::ok)
        // .filter(|line| line.contains("NAR"))
        .for_each(|line| println!("我打印了一行！行 = {line:?}"));

    Ok(())
}
