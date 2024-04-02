//! 定义与「非公理虚拟机」有关的抽象结构
//! * 🚩【2024-04-02 21:18:11】最初用于定义「虚拟机状态」
//!   * 🎯使用一个枚举指示虚拟机的状态
use anyhow::Result;

/// 虚拟机状态
/// * 📌指示虚拟机「是否正在运行」「是否终止」「是否报错」等状态
/// * 📄被[`crate::vm::VmRuntime::status`]返回
/// * 🎯可根据「是否终止」采取「自动重启」等操作
///   * ⚠️与[`crate::output::Output::TERMINATED`]不同：后者仅作为一条「通知」而不保证「实际状态就是那样」
///   * 📍一切状态以[`crate::vm::VmRuntime::status`]的返回值为准
/// * 📌一般在[`crate::vm::VmRuntime::terminate`]调用后转为[`Self::Terminated`]状态
///   * 可以是正常终止的`Terminated(Ok(..))`
///   * ❗亦可能在虚拟机报错退出时转为[`Self::Terminated`]状态（此时附带[`Err`]）
///
/// ! ⚠️不要在此使用泛型：虚拟机整体需要能被作为特征对象使用
pub enum VmStatus {
    /// 正在运行
    /// * ✅允许接收指令
    /// * ✅可拉取有效输出
    Running,

    /// 已终止
    /// * ℹ️包含作为[`Result`]的终止结果
    /// * ❌不允许接收指令
    /// * ❌不可拉取有效输出
    Terminated(Result<()>),
}
