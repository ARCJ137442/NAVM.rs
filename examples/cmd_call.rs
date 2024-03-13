//! 示例：打开一个NARS程序
//! * 🔗参考：https://rustwiki.org/zh-CN/rust-by-example/std_misc/process.html
#![allow(unused)]

use std::{
    borrow::Borrow,
    io::{stdin, Read, Stdin, Write},
    process::{self, Child, Command, Stdio},
    thread,
};

use nar_dev_utils::show;

const EXE_PATH_ONA: &str = r"H:\A137442\Develop\AGI\NARS\NARS-executables\NAR.exe";
const EXE_PATH_REPL: &str = r"H:\A137442\Develop\Julia\语言学小工Ju\繁简转换\dist\repl_简化.exe";
const EXE_PATH_ECHO: &str =
    r"H:\A137442\Develop\AGI\NARS\_Project\NAVM.rs\target\debug\examples\echo_exe.exe";
const EXE_WC: &str = r"wc";
const EXE_REV: &str = r"rev";

/// 只读取一次的call
#[test]
fn test_read_once() {
    let output = Command::new(EXE_PATH_ONA)
        // .arg("pong")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("failed and stderr was:\n{}", s);
    }
}

/// 经典例子@管道
#[test]
fn test_wc() {
    // 常量
    const TEXT: &str = "the quick brown fox jumped over the lazy dog\n";

    // 启动 `wc` 命令
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn wc: {:?}", why),
        Ok(process) => process,
    };

    // 将字符串写入 `wc` 的 `stdin`。
    //
    // `stdin` 拥有 `Option<ChildStdin>` 类型，不过我们已经知道这个实例不为空值，
    // 因而可以直接 `unwrap 它。
    match process.stdin.unwrap().write_all(TEXT.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {:?}", why),
        Ok(_) => println!("sent pangram to wc"),
    }

    // 因为 `stdin` 在上面调用后就不再存活，所以它被 `drop` 了，管道也被关闭。
    //
    // 这点非常重要，因为否则 `wc` 就不会开始处理我们刚刚发送的输入。

    // `stdout` 字段也拥有 `Option<ChildStdout>` 类型，所以必需解包。
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {:?}", why),
        Ok(_) => print!("wc responded with:\n{}", s),
    }
}
/// 读写REPL的call
fn _test_repl(c: Child, input: &str) {
    // 展示线程ID
    show!(c.id());

    // 获取标准输入、输出、错误
    // let mut c_in = c.stdin.expect("无法获取标准输入");
    // let mut c_out = c.stdout.expect("无法获取标准输出");
    // let mut c_err = c.stderr.expect("无法获取标准错误");

    // 展示线程信息
    // show!(&c_in, &c_out, &c_err;);

    // 写入输入
    {
        let mut c_in = c.stdin.unwrap();
        match c_in.write_all(input.as_bytes()) {
            Err(why) => panic!("couldn't write to wc stdin: {:?}", why),
            Ok(_) => println!("sent message"),
        }
    }

    // for _ in 0..10 {
    //     c_in.write_all("这是一个字符串\n\n".as_bytes())
    //         .expect("无法写入输入");
    //     c_in.flush().expect("无法刷新输入");
    //     c_in.flush();
    // }

    // 读取输出 | 【2024-03-11 20:18:28】不知为啥，这里卡住了
    let mut buffer = String::new();
    c.stdout
        .unwrap()
        .read_to_string(&mut buffer)
        .expect("无法读取输出");
    show!(buffer);
}

/// 读写REPL的call 2
/// * 🚩尝试使用「子线程」读写进程信息
fn _test_repl_2(mut c: Child, input: &str) {
    // 展示线程ID
    show!(c.id());

    // 写入输入
    let inputs = input.to_string(); // 需要把所有权拿到，才能安全搞进线程中
    let t1 = thread::spawn(move || {
        let mut c_in = c.stdin.unwrap();
        match c_in.write_all(inputs.as_bytes()) {
            Err(why) => panic!("couldn't write to wc stdin: {:?}", why),
            Ok(_) => println!("sent message"),
        }
        thread::sleep(std::time::Duration::from_secs_f32(1.0));
        // c.kill();
    });

    // 读取输出 | 【2024-03-11 20:18:28】不知为啥，这里卡住了
    let t2 = thread::spawn(move || {
        let mut buffer = String::new();
        let mut out = c.stdout.unwrap();
        // 开始尝试读取输出
        println!("正在读取输出。。。");
        out.read_to_string(&mut buffer).expect("无法读取输出");
        show!(buffer);
    });

    // 等待线程执行结束
    t1.join().unwrap();
    t2.join().unwrap();
}

#[test]
fn test_repl() {
    // 配置并启动子进程（外部exe，WC）
    let mut c = Command::new(EXE_WC)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("无法打开子进程");

    // 测试线程
    // _test_repl(c, "这是一个字符串，它是用来测试的\n");
    _test_repl_2(c, "这是一个字符串，它是用来测试的\n");

    // 配置并启动子进程（外部exe，REV）
    let mut c = Command::new(EXE_REV)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("无法打开子进程");

    // 测试线程
    // _test_repl(c, "这是一个字符串，它是用来测试的\n");
    _test_repl_2(c, "这是一个字符串，它是用来测试的\n");

    // 配置并启动子进程（外部exe，ECHO）
    let mut c = Command::new(EXE_PATH_ECHO)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("无法打开子进程");

    // 测试线程
    // _test_repl(c, "这是一个字符串，它是用来测试的\n");
    _test_repl_2(c, "这是一个字符串，它是用来测试的\n");

    // // 配置并启动子进程（本地闭包）
    // !!! 通过本地闭包产生的是线程，通过`Command::new`产生的是进程！
    // let mut c = thread::spawn(|| {
    //     println!("这是一条消息");
    // })
    // .spawn()
    // .expect("无法打开子进程");

    // // 测试线程
    // _test_repl(c, "这是一个字符串，它是用来测试的\n");

    // 配置并启动子进程（外部exe，REPL）
    let mut c = Command::new(EXE_PATH_REPL)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("无法打开子进程");

    // 测试线程
    _test_repl(c, "这是一个字符串，它是用来测试的\n");
}

fn main() {}
