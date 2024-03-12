use std::io::Write;

fn main() {
    let mut input = String::new();
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
     {
        // 预置字符串
        input.push_str("[ECHO] ");
        // 读取一行
        stdin.read_line(&mut input).expect("无法读取输入");
        // 写入一行
        stdout
            .write_all(input.as_bytes())
            .unwrap_or_else(|_| panic!("无法输入字符串「{input}」"));
        stdout.flush().expect("无法刷新输出！");
        // 清空输入
        input.clear();
    }
}
