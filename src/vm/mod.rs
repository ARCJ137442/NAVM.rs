//! 负责对整个「NAVM 非公理虚拟机」的抽象
//! * ✨核心idea：将所有CIN抽象成一个「接收统一格式指令，输出统一格式数据」的虚拟机

// 特征
mod traits;
pub use traits::*;

// 输出
mod output;
pub use output::*;
