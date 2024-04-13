//! 示例用「继承推理机」虚拟机实现
//! * 📄全程：Inheritance Reasoning System
//! * 📄理论来源：《NAL》中的「继承逻辑-1」**IL-1**
//! * 🎯展示：如何基于NAVM构建符合IO格式的最小CIN
//! * 🎯展示：NAVM的「原生」字符串IO
//!   * 🔗对应BabelNAR.rs的「原生」转译器
//! * ⚠️需要用到[`narsese`]库中的「[枚举Narsese](`narsese::enum_narsese`)」特性

use navm::{
    cmd::Cmd,
    vm::{VmLauncher, VmRuntime},
};
use std::io::stdin;

nar_dev_utils::mods! {
    // 虚拟机部分
    use pub vm;
}

/// 入口
fn main() {
    // 启动虚拟机
    let vm = VmDed.launch().unwrap();
    // 开始REPL
    repl(vm)
}

fn repl(mut vm: impl VmRuntime) {
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
        repl_line(&mut vm, line);

        // 清空缓冲区
        buf.clear();
    }
}

/// REPL执行一行
fn repl_line(vm: &mut impl VmRuntime, line: &str) {
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
