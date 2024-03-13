//! 负责「NAVM输出」与其它类型的转换
//! * 🎯自定义指令API
//! * 📌主要是JSON等外部可交换的格式
//!
//! TODO: 完善具体内容
#![allow(unused)]

// use std::collections::HashMap;

// * 💫不知这些用来干什么的
// /// 词法意义上的NAVM指令指令
// /// * 🎯用于捕获与拆分字符串到「指令头 参数...」的形式
// ///   * 📌不同指令头有不同的处理方法（使用闭包实现）
// ///   * 📌同时，具有默认的「拆分出头→全部后续内容塞到一行中」的默认方式（使用「指令头→Box(闭包)」映射）
// pub(crate) struct LexicalCmd<'a> {
//     /// 指令头
//     head: &'a str,
//     /// 指令参数集
//     params: Vec<&'a str>,
// }

// /// 指令参数转译函数
// pub(crate) type LexicalParamsParseFn = Box<dyn Fn(&str) -> Vec<&str>>;

// /// 指令参数转译字典
// pub(crate) struct LexicalCmdParseMap {
//     /// 头⇒参数解析函数 映射表
//     map: HashMap<String, LexicalParamsParseFn>,
//     /// 默认参数解析函数
//     default_parse_fn: LexicalParamsParseFn,
// }

// impl LexicalCmdParseMap {

//     /// 构造函数
//     /// * ⚠️闭包的生命周期必须得是静态的
//     pub(crate) fn new(map: HashMap<String, LexicalParamsParseFn>, default_parse_fn: impl Fn(&str) -> Vec<&str> + 'static) -> Self {
//         Self { map, default_parse_fn: Box::new(default_parse_fn) }
//     }
// }
