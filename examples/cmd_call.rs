//! 示例：打开一个NARS程序
//! * 🔗参考：https://rustwiki.org/zh-CN/rust-by-example/std_misc/process.html
#![allow(unused)]

use std::process::Command;

const EXE_PATH_ONA: &str = r"H:\A137442\Develop\AGI\NARS\NARS-executables\NAR.exe";
const EXE_PATH_REPL: &str = r"H:\A137442\Develop\Julia\语言学小工Ju\繁简转换\dist\repl_简化.exe";
const EXE_PATH: &str = EXE_PATH_REPL;

fn main() {
    let output = Command::new(EXE_PATH)
        // .arg("pong")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
    });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("failed and stderr was:\n{}", s);
    }
}
