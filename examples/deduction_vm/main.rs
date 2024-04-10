//! 示例用「原生IL-1」虚拟机实现
//! * 🎯展示：如何基于NAVM构建符合IO格式的最小CIN
//! * ⚠️需要用到[`narsese`]库中的「[枚举Narsese](`narsese::enum_narsese`)」特性

use navm::{
    cmd::Cmd,
    vm::{VmLauncher, VmRuntime},
};
use std::io::stdin;

nar_dev_utils::mods! {
    use pub vm;
}

/// REPL
fn main() {
    let mut vm = VmDed.launch().unwrap();
    let mut buf = String::new();
    loop {
        // 读取缓冲区内容
        if let Err(e) = stdin().read_line(&mut buf) {
            println!("读取行错误：{e}");
        }
        let line = buf.trim();
        if line.is_empty() {
            continue;
        }
        repl(&mut vm, line);

        // 清空缓冲区
        buf.clear();
    }
}

/// REPL执行一行
fn repl(vm: &mut VmRuntimeDed, line: &str) {
    // 解析&输入
    match Cmd::parse(line) {
        Ok(cmd) => {
            if let Err(e) = vm.input_cmd(cmd) {
                println!("指令输入时发生错误：{e}");
            }
        }
        Err(e) => println!("指令解析错误: {}", e),
    }

    // 拉取输出
    while let Ok(Some(output)) = vm.try_fetch_output() {
        // 输出/JSON
        println!("{}", output.to_json_string());
    }
}
