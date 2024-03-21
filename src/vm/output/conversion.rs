//! 负责「NAVM输出」与其它类型的转换
//! * 🎯自定义指令API
//! * 📌主要是JSON等外部可交换的格式
//!
//! # Reference
//!
//! 📄JSON格式参考如下TypeScript定义：
//! ```typescript
//! export type NARSOutput = {
//!     type?: string
//!     content?: string
//!     operation?: string[]
//! }
//! ```
//!
//! 另请参考其所对接的结构[`OutputJSON`]
#![allow(unused)]

use super::Output;

/// 用于统一存储「JSON化的NAVM输出」的结构
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OutputJSON {
    /// 输出的类型
    /// * 📝使用`r#`前缀以避开关键字
    ///   * 实际使用时还是以`type`作为字段
    pub r#type: String,

    /// 输出的内容
    pub content: String,

    /// 输出的操作信息（可能没有）
    pub operation: Option<Vec<String>>,
}

/// 将「JSON化的NAVM输出」转换为字符串
/// * 🚩【2024-03-21 12:19:55】目前通过[`format!`]实现，只需传入字符串引用
impl ToString for OutputJSON {
    // TODO: 添加使用`serde_json`的特化方案
    fn to_string(&self) -> String {
        let type_ = &self.r#type;
        let content = &self.content;
        let operation_suffix = match &self.operation {
            // 仅在内部有值时进行处理
            Some(v) => format!(
                // ! 这是JSON的一部分
                ",operation:[{}]",
                v.iter()
                    // 统一转义
                    .map(|s| format!("{s:?}"))
                    .collect::<Vec<_>>()
                    // 使用逗号分隔
                    .join(",")
            ),
            None => "".to_string(),
        };
        format!(
            "{}type:{type_:?},content:{content:?}{}{}",
            "{",
            // 尝试转换，有⇒添加，无⇒置空
            operation_suffix,
            "}"
        )
    }
}

impl Output {
    /// 将NAVM输出转换为JSON结构
    pub fn to_json_struct(&self) -> OutputJSON {
        let r#type = self.type_name().to_owned();
        let operation: Option<Vec<String>> = match self {
            // * 🚩只有`EXE`才会附带操作信息
            Output::EXE { operation, .. } => Some(operation.clone().into()),
            _ => None,
        };
        // 内容
        let content = match self {
            // 字段`content`
            Output::IN { content }
            | Output::OUT { content }
            | Output::COMMENT { content }
            | Output::ANTICIPATE { content }
            | Output::UNCLASSIFIED { content, .. }
            | Output::OTHER { content } => content.clone(),
            // 字段`description`
            Output::ERROR { description } => description.clone(),
            // 字段`narsese`
            Output::ANSWER { narsese } | Output::ACHIEVED { narsese } => narsese.clone(),
            // 字段`source`
            Output::EXE { source, .. } => source.clone(),
            // 字段`message`
            Output::INFO { message } => message.clone(),
        };
        // 输出
        OutputJSON {
            content,
            r#type,
            operation,
        }
    }
    /// 将NAVM输出转换为JSON字符串
    /// * 🚩先转换为JSON结构，再将其转换为字符串
    pub fn to_json_string(&self) -> String {
        self.to_json_struct().to_string()
    }
}

/// 单元测试
#[cfg(test)]
mod tests {
    use crate::vm::tests_output::test_samples;

    /// 测试/转换为JSON字符串
    #[test]
    fn test_json_str() {
        let samples = test_samples();
        for output in samples {
            let s = dbg!(output.to_json_string());
            println!("{s}");
        }
    }
}
