//! 引入「非公理虚拟机」的特征

use crate::{cmd::Cmd, output::Output};
use util::ResultS;

/// 虚拟机运行时
/// * 🎯所有**已启动**的「非公理虚拟机」遵循的特征
/// * 🚩【2024-03-21 21:48:22】目前方案：通过一个启动器进行启动
///
/// ## 基本理论模型
///
/// 「非公理虚拟机」的运行时，简单而言只做两件事：
/// * 输入：从某处接收「指令」[`crate::cmd::Cmd`]
/// * 输出：产生一系列「输出」[`crate::output::Output`]
///
/// 在这两件事之间，虚拟机和外界是**并行**运作的
/// * 📌输入、输出都是**异步**的
///   * 输入虚拟机的指令，不会在函数层面有任何返回值
///   * 从虚拟机发出的输出，不直接与「输入」相绑定
pub trait VmRuntime {
    // 输入 //

    /// 【抽象】向虚拟机输入NAVM指令
    /// * 📌几乎是一切NAVM的核心函数
    /// * 📌是一个**异步**的方法
    ///   * 输入之后不会有「回传」（即此处返回值）
    ///   * 回传需要在「输出侦听器」进行捕获
    /// * 🚩输入时有可能产生错误：此时返回一个错误信息
    ///   * 📄指令转译错误
    fn input_cmd(&mut self, cmd: Cmd) -> ResultS<()>;

    // 输出 //

    /// 【抽象】从虚拟机中获取一个输出
    /// * ⚠️若暂时没输出，则会阻塞调用者
    /// * 📌输出顺序**从旧到新**：先产生的输出先被拉取出来
    fn fetch_output(&mut self) -> ResultS<Output>;

    /// 【抽象】尝试从虚拟机中获取一个输出
    /// * 🚩逻辑类似[`VmRuntime::fetch_output`]，但是非阻塞版本
    /// * 🚩可能没有：此时说明虚拟机没有输出
    fn try_fetch_output(&mut self) -> ResultS<Option<Output>>;
}

/// 虚拟机启动器
/// * 📌使用Rust的「Builder模式」
///   * 🚩整体使用流程：构造、链式调用加配置、最后转换成运行时
/// * 🎯用于「新建配置、构建参数、启动」的初始化流程
///   * 📄如：`VmCommand::new()`
///
/// ! 📝不能在带有`self`的方法中使用默认实现：`Self`内存大小未知
/// @template `Runtime` 要构建到的运行时
pub trait VmLauncher<Runtime: VmRuntime> {
    /// 从builder构建并启动运行时
    /// * ⚠️消耗自身
    /// * 📌类型指定为特定的运行时
    /// * 📌兼「构建」和「启动」：此时的运行时应该**即刻启动**
    /// * 📌理念：构建好后的运行时，要么是「待启动的配置状态」，要么是「在运行的『运行时状态』」
    ///   * 📄如「命令行运行时」会立即启动子进程（及其辅助线程）
    /// * ⚠️【2024-03-24 02:08:43】目前约定「启动」必须成功
    fn launch(self) -> Runtime;
}
