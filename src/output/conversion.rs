//! 负责「NAVM输出」与其它类型的转换
//! * 🎯自定义指令API
//! * 📌主要是JSON等外部可交换的格式
//! * 🚩【2024-03-23 18:05:22】约定：其中所有Narsese均遵循CommonNarsese格式规范
//!
//! # Reference
//!
//! 📄JSON格式参考如下TypeScript定义：
//! ```typescript
//! export type NARSOutput = {
//!     /** 输出的类别 */
//!     type: string
//!     /** 输出的（原始）内容，可能会截去类别信息 */
//!     content: string
//!     /** 若输出包含被识别出的Narsese，则为相应的Narsese字符串 */
//!     narsese?: string
//!     /** 若输出包含被识别出的NARS操作，则为`[无尖号操作名, ...操作参数]`字符串数组 */
//!     operation?: string[]
//! }
//! ```
//!
//! 另请参考其所对接的结构[`OutputJSON`]
#![allow(unused)]

use super::Output;
use narsese::conversion::string::impl_lexical::format_instances::FORMAT_ASCII;

/// 用于统一存储「JSON化的NAVM输出」的结构
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OutputJSON {
    /// 输出的类型
    /// * 📝使用`r#`前缀以避开关键字
    ///   * 实际使用时还是以`type`作为字段
    pub r#type: String,

    /// 共有：输出的内容
    pub content: String,

    /// 专有：输出的Narsese
    /// * 📌格式：ASCII CommonNarsese
    /// * 🚩【2024-03-22 18:37:37】目前暂无将「词法Narsese」无缝转译到JSON的方案
    ///   * 📌统一其中格式足矣
    pub narsese: Option<String>,

    /// 专有：输出的操作信息（可能没有）
    pub operation: Option<Vec<String>>,
}

/// 将「JSON化的NAVM输出」转换为字符串
/// * 🚩【2024-03-21 12:19:55】目前通过[`format!`]实现，只需传入字符串引用
/// * 💭此处仍然是非常专用的JSON生成代码，其基于JSON格式的稳定性
///   * ⚠️【2024-03-22 17:31:43】注意：内部字符串的转义，可能有隐患（目前通过Rust的`format!`实现）
impl ToString for OutputJSON {
    // TODO: 添加有关`serde_json`的特化方案
    fn to_string(&self) -> String {
        // 共有参数：类型
        let type_ = &self.r#type;
        // 共有参数：内容（原始字符串）
        let content = &self.content;
        // 特有参数：操作（字符串数组）
        let operation_suffix = match &self.operation {
            // 仅在内部有值时进行处理
            Some(v) => format!(
                // ! 这是JSON的一部分
                ",\"operation\":[{}]",
                v.iter()
                    // 统一转义
                    .map(|s| format!("{s:?}"))
                    .collect::<Vec<_>>()
                    // 使用逗号分隔
                    .join(",")
            ),
            // 没有⇒空字串
            None => "".to_string(),
        };
        // 特有参数：内部Narsese（实现为ASCII CommonNarsese）
        let narsese_suffix = match &self.narsese {
            // 存在⇒以Debug形式添加（自动转义）
            Some(narsese_str) => format!(",\"narsese\":{narsese_str:?}"),
            // 没有⇒空字串
            None => "".to_string(),
        };
        // 最终拼接
        format!(
            "{}\"type\":{type_:?},\"content\":{content:?}{}{}{}",
            "{",
            // 尝试转换，有⇒添加，无⇒置空
            operation_suffix,
            narsese_suffix,
            "}"
        )
    }
}

impl Output {
    /// 将NAVM输出转换为JSON结构
    pub fn to_json_struct(&self) -> OutputJSON {
        // 共有：输出类型
        let r#type = self.type_name().to_owned();
        // 共有：内容 | 原始内容
        let content = self.raw_content().into();
        // 专有：操作
        let operation: Option<Vec<String>> = match self {
            // * 🚩只有`EXE`才会附带操作信息
            Output::EXE { operation, .. } => Some(operation.clone().into()),
            _ => None,
        };
        // 专有：Narsese（词法Narsese）
        let narsese = self
            .get_narsese()
            // * 🚩将内部可能有的Narsese值转换为ASCII CommonNarsese字符串
            .map(|narsese| FORMAT_ASCII.format(narsese));
        // 输出
        OutputJSON {
            content,
            narsese,
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
    use crate::output::tests::test_samples;

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
