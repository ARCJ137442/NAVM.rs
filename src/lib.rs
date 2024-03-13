//! 库的主类

// 实用宏
// * 🚩内外使用规则：
//   * 在自身`navm`内部使用`util`
//   * 在外部（无法使用`crate`引用`navm`）使用`nar_dev_utils`
pub extern crate nar_dev_utils as util;

// cmd
pub mod cmd;

// NAVM虚拟机
pub mod vm;
