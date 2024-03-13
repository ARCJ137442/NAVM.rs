//! 引入「非公理虚拟机」的特征

use crate::cmd;
use crate::vm;

/// 所有「非公理虚拟机」遵循的特征
pub trait VM {
    // 指令相关 //

    /// 【抽象】向虚拟机输入NAVM指令指令
    /// * 📌几乎是一切NAVM的核心函数
    fn input_cmd(&mut self, cmd: cmd::Cmd);

    // 输出相关 //

    /// 【抽象】向虚拟机存入一个输出
    /// * 🎯用于「向实际的『输出缓存列表』中存储输出」
    fn store_output(&mut self, output: vm::Output);

    /// 【抽象】从虚拟机中获取一个输出
    /// * 📌一般使用「输出缓冲区」实现
    /// * 📌一般**从旧到新**输出
    /// * 🚩可能没有：此时说明虚拟机没有输出
    fn fetch_output(&mut self) -> Option<vm::Output>;

    /// 【抽象】向虚拟机添加一个「输出侦听器」
    /// * 📌功能上：可添加多个，并且被链式调用
    ///   * 🚩添加时基本是「先来后到」原则
    ///   * 🚩所传入的函数一般被放进[`Box`]中
    /// * 🚩链式调用机理：输出消耗链——通过[`Option`]控制「输出是否被处理（被消耗）」
    ///   * 返回`Some(输出)`：输出未被消耗，侦听可以继续（实现「一个输出，多方处理」的效果）
    ///   * 返回`None`：输出已被消耗，侦听链中断
    fn add_output_listener<Listener>(&mut self, listener: Listener)
    where
        Listener: FnMut(vm::Output) -> Option<vm::Output>;

    /// 【抽象】在虚拟机上遍历「输出侦听器」
    /// * 🚩返回一个「输出所有『输出侦听器』的可变引用」的迭代器
    /// * 🚩输出全部装箱，以便后续作为特征对象
    /// * 📝此处需要统一返回类型的生命周期，避免「自身比返回的迭代器提早销毁」的悬垂引用
    fn iter_output_listeners<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a mut dyn FnMut(vm::Output) -> Option<vm::Output>> + 'a>;

    /// 事件：当（封装的）CIN存储一个输出时
    /// * 🚩遍历所有「输出侦听器」，若均未被捕获，则【存入】输出（缓冲区）
    fn on_output(&mut self, output: vm::Output) {
        // 装入容器
        let mut output = Some(output);
        // 在所有侦听器上传递
        for listener in self.iter_output_listeners() {
            match output {
                // 若未消耗⇒继续传递
                Some(inner) => output = listener(inner),
                // 若被消耗⇒结束传递
                None => break,
            }
        }
        // 传递后还有⇒存入（缓冲区）
        if let Some(inner) = output {
            self.store_output(inner)
        }
    }
}
