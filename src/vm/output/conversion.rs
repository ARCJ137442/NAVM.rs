//! 负责「NAVM输出」与其它类型的转换
//! * 主要是JSON等外部可交换的格式
//!
//! TODO: 完善具体内容

use std::collections::HashMap;

/// 词法意义上的NAIR指令
/// * 🎯用于捕获与拆分字符串到「指令头 参数...」的形式
///   * 📌不同指令头有不同的处理方法（使用闭包实现）
///   * 📌同时，具有默认的「拆分出头→全部后续内容塞到一行中」的默认方式（使用「指令头→Box(闭包)」映射）
pub(crate) struct LexicalCmd<'a> {
    /// 指令头
    head: &'a str,
    /// 指令参数集
    params: Vec<&'a str>,
}

/// 指令参数转译函数
pub(crate) type LexicalParamsParseFn = Box<dyn Fn(&str) -> Vec<&str>>;

/// 指令参数转译字典
pub(crate) struct LexicalCmdParseMap {
    /// 头⇒参数解析函数 映射表
    map: HashMap<String, LexicalParamsParseFn>,
    /// 默认参数解析函数
    default_parse_fn: LexicalParamsParseFn,
}

impl LexicalCmdParseMap {
    pub(crate) fn new<'a>(map: HashMap<String, LexicalParamsParseFn>, default_parse_fn: impl Fn(&str) -> Vec<&str>) -> Self {
        Self { map, default_parse_fn: Box::new(default_parse_fn) }
    }
}
