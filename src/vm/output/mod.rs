//! 定义NAVM的输出类型
//! * 📄最初该类型定义在**BabelNAR** [^1] 中
//! * 🚩现在通过枚举统一定义
//!
//! [^1]: <https://github.com/ARCJ137442/BabelNAR.jl>

// 数据结构
mod structs;
pub use structs::*;

// 转换
mod conversion;
pub use conversion::*;
