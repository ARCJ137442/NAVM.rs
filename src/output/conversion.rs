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
//!     /** 输出的类别（全大写） */
//!     type: string
//!     /** 输出的（原始）内容，可能会截去类别信息 */
//!     content: string
//!     /** 若输出包含被识别出的Narsese，则为相应的Narsese字符串 */
//!     narsese?: string
//!     /** 若输出包含被识别出的NARS操作，则为`[无尖号操作名, ...操作参数]`字符串数组 */
//!     operation?: [string, ...string[]]
//! }
//! ```
//!
//! 另请参考其所对接的结构[`OutputJSON`]
#![allow(unused)]

use super::{Operation, Output};
use anyhow::{anyhow, Result};
use narsese::conversion::string::impl_lexical::format_instances::FORMAT_ASCII;
use serde::{Deserialize, Serialize};
use util::{list, manipulate, pipe};

/// 用于统一存储「JSON化的NAVM输出」的结构
/// * 🎯对包含各种不同字段的枚举[`Output`]进行信息压缩
///   * 多种不同字段⇒数个相同字段
/// * 🚩【2024-04-09 10:39:33】现在接入[`serde`]与[`serde_json`]
///   * 📝当[`Option`]为[`None`]时忽略：使用`#[serde(skip_serializing_if = "Option::is_none")]`与`#[serde(default)]`
///     * 前者在序列化时条件忽略[`None`]字段，后者在反序列化时条件设置默认值[`None`]
///   * 🔗参考：<https://stackoverflow.com/questions/53900612/how-do-i-avoid-generating-json-when-serializing-a-value-that-is-null-or-a-defaul>
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OutputJSON {
    /// 输出的类别
    /// * 📝使用`r#`前缀以避开关键字
    ///   * 实际使用时还是以`type`作为字段
    pub r#type: String,

    /// 共有：输出的内容
    pub content: String,

    /// 专有：输出的Narsese
    /// * 📌格式：ASCII CommonNarsese
    /// * 🚩【2024-03-22 18:37:37】目前暂无将「词法Narsese」无缝转译到JSON的方案
    ///   * 📌统一其中格式足矣
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub narsese: Option<String>,

    /// 专有：输出的操作信息（可能没有）
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub operation: Option<Vec<String>>,
}

/// 将「JSON化的NAVM输出」转换为字符串
/// * ✅【2024-04-09 10:31:23】现在接入[`serde_json`]以实现序列化
///   * ✨可选择性禁用
///   * ⚠️理论上不会失败（字符串/字符串数组）
#[cfg(feature = "serde_json")]
impl ToString for OutputJSON {
    fn to_string(&self) -> String {
        // *
        serde_json::to_string(self).expect("数据序列化失败")
    }
}

impl Output {
    // * 序列化 * //

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
    #[cfg(feature = "serde_json")]
    pub fn to_json_string(&self) -> String {
        self.to_json_struct().to_string()
    }

    /// 将NAVM输出数组转换为JSON数组
    #[cfg(feature = "serde_json")]
    pub fn vec_to_json_string(v: &[Self]) -> String {
        serde_json::to_string(v).expect("不会转换失败：内部JSON结构总是转换成功")
    }

    /// 将NAVM输出引用数组转换为JSON数组
    /// * ⚠️与[`vec_to_json_string`]的核心区别就在`&[&Self]`与`&[Self]`
    #[cfg(feature = "serde_json")]
    pub fn vec_ref_to_json_string(v: &[&Self]) -> String {
        serde_json::to_string(&v).expect("不会转换失败：内部JSON结构总是转换成功")
    }

    // * 反序列化 * //

    /// 尝试从中间「JSON结构体」折叠为自身
    /// * 🚩先获取各个字段，再根据「输出类型」进行对应折叠
    pub fn try_from_json_struct(json: OutputJSON) -> Result<Self> {
        // 类型
        let r#type = json.r#type;

        // 内容
        let content = json.content;

        // 操作
        let operation = match json.operation {
            Some(operation) => match operation.len() {
                0 => return Err(anyhow!("NARS输出中，操作缺乏操作符")),
                _ => Some(Operation::try_from_strings(
                    &operation[0],
                    // * 🚩使用不可变手段解析
                    // 跳过第一个值
                    operation[1..].iter(),
                )?),
            },
            None => None,
        };

        // Narsese | 🚩逐个解析内部字符串（其中引入错误）
        let narsese = match json.narsese {
            Some(narsese) => Some(
                // ! 此中需要使用`?`上抛错误，因此不能使用[`Option::map`]
                FORMAT_ASCII.parse(&narsese)?,
            ),
            None => None,
        };

        // 四者组合 | 🚩复用常量池中的常量
        use super::structs::type_names::*;
        let out = match r#type.as_str() {
            IN => Output::IN { content, narsese },
            OUT => Output::OUT {
                content_raw: content,
                narsese,
            },
            ERROR => Output::ERROR {
                description: content,
            },
            ANSWER => Output::ANSWER {
                content_raw: content,
                narsese,
            },
            ACHIEVED => Output::ACHIEVED {
                narsese,
                content_raw: content,
            },
            EXE => Output::EXE {
                content_raw: content,
                operation: operation.ok_or(anyhow!("在解析NARS操作中缺乏操作"))?,
            },
            INFO => Output::INFO { message: content },
            COMMENT => Output::COMMENT { content },
            TERMINATED => Output::TERMINATED {
                description: content,
            },
            OTHER => Output::OTHER { content },
            _ => Output::UNCLASSIFIED {
                r#type,
                content,
                narsese,
            },
        };

        // 返回
        Ok(out)
    }

    /// 尝试从 JSON 字符串中解析出输出
    /// * 🚩先解析出中间JSON结构体，再将其折叠为输出类型
    #[cfg(feature = "serde_json")]
    pub fn try_from_json_string(s: &str) -> Result<Self> {
        pipe! {
            s
            => OutputJSON::try_from
            => {?}#
            => Self::try_from_json_struct
        }
    }

    /// 将JSON字符串转换为「输出类型数组」
    /// * 🚩现在直接使用[`serde_json::from_str`]方法
    ///   * ✅【2024-04-09 13:26:42】已通过[`serde`]对[`Output`]进行默认的序列化、反序列化实现
    /// * 🔗参考[`serde`]对[`Vec`]的默认反序列化实现：<https://docs.rs/serde/latest/serde/trait.Deserialize.html#impl-Deserialize%3C'de%3E-for-Vec%3CT%3E>
    #[cfg(feature = "serde_json")]
    pub fn vec_try_from_json_string(s: &str) -> Result<Vec<Self>> {
        Ok(serde_json::from_str(s)?)
    }
}

/// 对输出直接实现序列化
impl Serialize for Output {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // 先转换为JSON对象
        let json_s = self.to_json_struct();
        // 再以JSON对象进行序列化
        json_s.serialize(serializer)
    }
}

/// 对输出直接实现反序列化
impl<'de> Deserialize<'de> for Output {
    fn deserialize<D>(deserializer: D) -> std::prelude::v1::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        // 先反序列化成JSON对象
        let json_s = OutputJSON::deserialize(deserializer)?;
        // 再从JSON对象解析，并转换其中的错误类型
        // * 📝归并到「通用错误转换函数」使用[`D::Error::custom`]
        // * 🔗参考：<https://serde.rs/impl-deserializer.html>
        Self::try_from_json_struct(json_s).map_err(D::Error::custom)
    }
}

#[cfg(feature = "serde_json")]
impl TryFrom<&str> for OutputJSON {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self> {
        OutputJSON::try_from_json_string(s)
    }
}

impl TryFrom<OutputJSON> for Output {
    type Error = anyhow::Error;

    fn try_from(json: OutputJSON) -> Result<Self> {
        Output::try_from_json_struct(json)
    }
}

#[cfg(feature = "serde_json")]
impl OutputJSON {
    pub fn try_from_json_string(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}

impl From<Output> for OutputJSON {
    fn from(output: Output) -> Self {
        output.to_json_struct()
    }
}

/// 单元测试
#[cfg(test)]
mod tests {
    use crate::output::{tests::test_samples, Output};

    /// 测试/与JSON结构互转
    /// * 🎯能与JSON结构无损互转
    #[test]
    fn test_json_struct() {
        let samples = test_samples();
        // 各个样本的测试
        for output in &samples {
            let json = output.to_json_struct();
            println!("{json:?}");
            let re_converted = super::Output::try_from_json_struct(json).expect("JSON结构解析失败");
            // println!("<= {re_converted:?}");
            assert_eq!(*output, re_converted);
        }
    }

    /// 测试/与JSON字串互转
    /// * 🎯能与JSON字符串无损互转
    #[test]
    #[cfg(feature = "serde_json")]
    fn test_json_str() {
        use util::asserts;

        let samples = test_samples();
        // 各个样本的测试
        for output in &samples {
            let json_str = output.to_json_string();
            println!("{json_str}");
            let re_converted =
                super::Output::try_from_json_string(&json_str).expect("JSON字串解析失败");
            // println!("<= {re_converted:?}");
            assert_eq!(*output, re_converted);
        }
        // 样本集总体的测试
        let sample_refs = samples.iter().collect::<Vec<_>>();
        let json_str = Output::vec_to_json_string(&samples);
        let json_str_ref = Output::vec_ref_to_json_string(&sample_refs);
        println!("{json_str}");
        let re_converted = Output::vec_try_from_json_string(&json_str).expect("JSON转换失败");
        let re_converted_ref = Output::vec_try_from_json_string(&json_str).expect("JSON转换失败");
        asserts! {
            samples => re_converted,
            samples => re_converted_ref,
            re_converted => re_converted_ref,
        }
    }
}
