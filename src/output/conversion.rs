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
///   * ⚠️理论上不会失败（字符串/字符串数组）
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
    pub fn to_json_string(&self) -> String {
        self.to_json_struct().to_string()
    }

    /// 将NAVM输出数组转换为JSON数组
    /// * 📌[`serde`]并未对Vec<Self>`自动实现[`Serialize`]特征
    /// * 🚩此处采用手动序列化的方式
    pub fn vec_to_json_string(v: &[Self]) -> String {
        // 先转换为JSON结构
        let vec = list![
            (output.to_json_struct())
            for output in (v)
        ];
        // 再对结构数组进行序列化
        serde_json::to_string(&vec).expect("不会转换失败：内部JSON结构总是转换成功")
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
    pub fn try_from_json_string(s: &str) -> Result<Self> {
        pipe! {
            s
            => OutputJSON::try_from
            => {?}#
            => Self::try_from_json_struct
        }
    }

    /// 将JSON字符串转换为「输出类型数组」
    /// * 🚩先利用派生的`Vec<OutputJSON>`实现，转换为「中间JSON结构体」
    ///   * 🚩再将其逐一转换为「输出数组」
    /// * 🔗参考[`serde`]对[`Vec`]的默认反序列化实现：<https://docs.rs/serde/latest/serde/trait.Deserialize.html#impl-Deserialize%3C'de%3E-for-Vec%3CT%3E>
    ///   * ⚠️并不对[`Output`]直接实现[`Deserialize`]
    pub fn vec_try_from_json_string(s: &str) -> Result<Vec<Self>> {
        // 先转换为JSON结构数组
        let v: Vec<OutputJSON> = serde_json::from_str(s)?;
        // 再逐一折叠
        Ok(list![
            (Self::try_from_json_struct(json)?)
            for json in (v)
        ])
    }
}

// TODO: impl Serialize for Output
// TODO: impl Deserialize for Output

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

    /// 测试/与JSON字串互转
    /// * 🎯能与JSON字符串无损互转
    #[test]
    fn test_json_str() {
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
        let json_str = Output::vec_to_json_string(&samples);
        println!("{json_str}");
        let re_converted = Output::vec_try_from_json_string(&json_str).expect("JSON转换失败");
        assert_eq!(samples, re_converted);
    }
}
