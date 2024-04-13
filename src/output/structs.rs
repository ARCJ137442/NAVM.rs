//! 定义NAVM的输出类型
//! * 🚩【2024-03-21 11:34:10】目前使用枚举实现
//!
//! ! 📌【2024-03-27 19:29:44】现在移除输出类型`ANTICIPATE`，降格为`UNCLASSIFIED`
//! * 原因：仅在特定CIN中出现，并不普遍适用于各CIN（并且在OpenNARS其中也只是插件）
//!
//! # Reference
//!
//! **BabelNAR**中的如下Julia代码（旧）：
//! ```julia
//! NARSOutputType = (;
//!     IN = "IN",
//!     OUT = "OUT",
//!     ERROR = "ERROR",
//!     ANSWER = "ANSWER",
//!     ACHIEVED = "ACHIEVED",
//!     EXE = "EXE",
//!     INFO = "INFO",
//!     COMMENT = "COMMENT",
//!     ANTICIPATE = "ANTICIPATE",
//!     OTHER = "OTHER"
//!     # *【2024-01-25 15:27:03】`OTHER`类型用于表示「暂无法格式化识别」的其它信息
//!     #     * @example 如OpenNARS的`executed based on`（操作执行的证据基础，用于验证「系统是否习得知识」）
//!     #     * 🎯用于在后续实验中提取「推理器特异」的实用信息
//! )
//! ```
//! 🔗[GitHub链接](https://github.com/ARCJ137442/BabelNAR.jl/blob/main/src/CIN/struct/NARSOutputType.jl)

use anyhow::Result;
use narsese::{
    conversion::string::impl_lexical::format_instances::FORMAT_ASCII,
    lexical::{Narsese as LexicalNarsese, Term as LexicalTerm},
};
use std::fmt::Display;
use util::{AsStrRef, JoinTo};

/// NAVM输出类型
/// * 🎯使用枚举，统一对「输出类别」分派
/// * 📌除其中的[`String`]类型字段，通用于所有具体实现
///   * 📄与具体NAVM实现无关
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Output {
    /// 表示「已输入信息」的recall
    /// * 📌该「信息」一般是Narsese字符串
    ///   * 如各类CIN对Narsese输入的回显
    /// * 📄样例 @ ONA: `Input: <A --> B>. Priority=1.000000 Truth: frequency=1.000000, confidence=0.900000\n`
    /// * ⚠️部分CIN可能不会输出
    /// * 🚩【2024-03-29 22:41:33】需要支持`narsese`属性，以便在测试时支持「回显检测」
    ///   * 📄如各类CIN对Narsese输入的回显
    IN {
        /// 原始内容
        content: String,
        /// （可能有的）Narsese内容（词法Narsese）
        /// * ⚠️因CIN而异
        narsese: Option<LexicalNarsese>,
    },

    /// 表示「导出信息」的recall
    /// * 🎯一般「推理导出结论」等不太重要的信息
    /// * 📄样例 @ ONA: `Derived: <A --> C>. Priority=0.407250 Truth: frequency=1.000000, confidence=0.810000\n`
    ///
    /// ! ⚠️【2024-03-22 18:28:12】现在将「是否需要在所有『CIN输出』中提取统一的Narsese」**交给各大运行时**
    OUT {
        /// 原始内容
        content_raw: String,
        /// （可能有的）Narsese内容（词法Narsese）
        /// * ⚠️因CIN而异
        narsese: Option<LexicalNarsese>,
    },

    /// 表示「内部错误」的信息
    /// * 🎯一般传递「内部发生了一个错误，可能需要处理」
    /// * 📄样例 @ OpenNARS: `[ERR]: NullPointer Exception...`
    ERROR { description: String },

    /// 表示「输出一个『回答』」
    /// * 🎯一般各CIN对「问题」语句的「回答」
    /// * 🚩内部一般是相应的Narsese文本
    ///
    /// ! ⚠️【2024-03-22 18:28:12】现在将「是否需要在所有『CIN输出』中提取统一的Narsese」**交给各大运行时**
    ANSWER {
        /// 原始内容
        content_raw: String,

        /// （可能有的）Narsese内容（词法Narsese）
        /// * ⚠️因CIN而异
        narsese: Option<LexicalNarsese>,
    },

    /// 表示「输出一个『完成』」
    /// * 🎯一般各CIN对「目标」语句的「完成」
    /// * 🚩内部一般是相应的Narsese文本
    /// * 📄最初见于PyNARS（🔗[原PR](https://github.com/bowen-xu/PyNARS/pull/30)）
    /// * 📄样例 @ PyNARS: `ACHIEVED: A. :|: %1.000:0.900%`
    ///
    /// ! ⚠️【2024-03-22 18:28:12】现在将「是否需要在所有『CIN输出』中提取统一的Narsese」**交给各大运行时**
    ACHIEVED {
        /// 原始内容
        content_raw: String,

        /// （可能有的）Narsese内容（词法Narsese）
        /// * ⚠️因CIN而异
        narsese: Option<LexicalNarsese>,
    },

    /// 表示「输出一个操作」
    /// * 🎯一般表示各CIN「需要调用外部 代码/程序」的信号
    /// * 🚩内部封装专有数据结构
    ///   * 📌不内联的原因：数据结构[`Operation`]后续常常要**独立使用**
    /// * 📄样例 @ OpenNARS: `EXE: $0.45;0.90;0.95$ ^left([{SELF}, (*,P1,P2)])=null`
    EXE {
        /// 「截取出的操作」的上下文
        /// * 📌一般是操作所出现的行
        content_raw: String,

        /// 截取出的操作信息
        /// * 🚩使用专有数据结构，以便规整化交互
        operation: Operation,
    },

    /// 表示「输出一条信息」
    /// * 🎯一般是CIN输出的各种（无关紧要的）提示信息
    ///  * 📄样例 @ PyNARS: `INFO  : Loading RuleMap <LUT.pkl>...`
    INFO { message: String },

    /// 表示「输出一条注释」
    /// * 🎯一般表示（比OUT、INFO）更无关紧要的输出
    /// * 📄最初见于PyNARS
    COMMENT { content: String },

    /// 表示「CIN终止运行」
    /// * 🎯用于表征并告知「CIN终止」的情况
    ///   * 📌往往是NAVM运行时发出的最后一条消息
    /// * 📄ONA中「Narsese解析失败」「Narsese输入不合法」等，都会导致CIN停止运行
    ///   * 如：`Parsing error: Punctuation has to be belief . goal ! or question ?\n` `Test failed.`
    TERMINATED {
        /// 「终止」的描述
        description: String,
    },

    /// 表示其它CIN输出
    /// * 🎯用于表示「可以识别到类型，但不在此枚举中」的NAVM输出
    ///   * 📌针对一些特定CIN的「方言」使用
    ///   * 📌针对后续「使用模式匹配识别出的类型」使用
    UNCLASSIFIED {
        r#type: String,
        content: String,
        narsese: Option<LexicalNarsese>,
    },

    /// 表示其它CIN输出
    /// * 🎯一般表示「暂无法格式化识别」的其它CIN输出
    ///   * 📌大多数时候无关紧要
    ///   * 🎯一般对应一行输出
    /// * 📄样例 @ OpenNARS: `Got relative path for loading the config: ./config/defaultConfig.xml`
    /// * 📄样例 @ OpenNARS: `executed based on [...]`
    ///   * 📝操作执行的证据基础，用于验证「系统是否习得知识」
    OTHER { content: String },
}

/// 有关「输出类型名称」的常量池
pub mod type_names {
    /// 输出类型名称 @ IN
    pub const IN: &str = "IN";
    /// 输出类型名称 @ OUT
    pub const OUT: &str = "OUT";
    /// 输出类型名称 @ ERROR
    pub const ERROR: &str = "ERROR";
    /// 输出类型名称 @ ANSWER
    pub const ANSWER: &str = "ANSWER";
    /// 输出类型名称 @ ACHIEVED
    pub const ACHIEVED: &str = "ACHIEVED";
    /// 输出类型名称 @ EXE
    pub const EXE: &str = "EXE";
    /// 输出类型名称 @ INFO
    pub const INFO: &str = "INFO";
    /// 输出类型名称 @ COMMENT
    pub const COMMENT: &str = "COMMENT";
    /// 输出类型名称 @ TERMINATED
    pub const TERMINATED: &str = "TERMINATED";
    /// 输出类型名称 @ OTHER
    pub const OTHER: &str = "OTHER";

    /// 输出类型名称 @ ANTICIPATE
    /// * 🚩【2024-04-11 22:58:00】仅出现在OpenNARS、ONA中的「半正式类型」
    /// * 📝【2024-04-11 22:58:45】禁用の考量：所涉及NAL层级（NAL-9）过高、不稳定，且输出往往无用
    pub const ANTICIPATE: &str = "ANTICIPATE";
}

impl Output {
    /// 获取「NAVM输出」的类型
    /// * 📌【2024-03-21 11:36:49】使用[`str`]静态返回
    /// * 🚩直接`match`并返回**全大写**英文
    #[inline]
    #[doc(alias = "type")]
    #[doc(alias = "get_type")]
    pub fn type_name(&self) -> &str {
        use type_names::*;
        match self {
            Output::IN { .. } => IN,
            Output::OUT { .. } => OUT,
            Output::ERROR { .. } => ERROR,
            Output::ANSWER { .. } => ANSWER,
            Output::ACHIEVED { .. } => ACHIEVED,
            Output::EXE { .. } => EXE,
            Output::INFO { .. } => INFO,
            Output::COMMENT { .. } => COMMENT,
            Output::TERMINATED { .. } => TERMINATED,
            // ! 特别的「未分类」情形：使用其中预置的「类名」
            Output::UNCLASSIFIED { r#type, .. } => r#type.as_str(),
            Output::OTHER { .. } => OTHER,
        }
    }

    /// 获取「NAVM输出」的类型
    /// * 🔗[`Self::type_name`]的别名
    /// * 🎯便于调用者以`get`统一检索
    #[inline(always)]
    pub fn get_type(&self) -> &str {
        self.type_name()
    }

    /// 获取「NAVM输出」的原始内容
    /// * 🚩【2024-03-24 18:27:50】提取其中的「主要内容」「原始内容」
    ///   * 📌主要包含各CIN输出的行
    pub fn raw_content(&self) -> &str {
        match self {
            Output::IN { content, .. }
            | Output::OUT {
                content_raw: content,
                ..
            }
            | Output::COMMENT { content }
            | Output::UNCLASSIFIED { content, .. }
            | Output::OTHER { content }
            | Output::ERROR {
                description: content,
            }
            | Output::ANSWER {
                content_raw: content,
                ..
            }
            | Output::ACHIEVED {
                content_raw: content,
                ..
            }
            | Output::EXE {
                content_raw: content,
                ..
            }
            | Output::INFO { message: content }
            | Output::TERMINATED {
                description: content,
            } => content,
        }
    }

    /// 获取「NAVM输出」的类型
    /// * 🔗[`Self::raw_content`]的别名
    /// * 🎯便于调用者以`get`统一检索
    #[inline(always)]
    pub fn get_content(&self) -> &str {
        self.raw_content()
    }

    /// 判断其「类型/头部」是否为指定的字串
    /// * ⚠️参数需要使用全大写的字符串，如"ANSWER"
    pub fn is_type(&self, type_name: &str) -> bool {
        self.type_name() == type_name
    }

    /// 获取Narsese（词法Narsese）
    /// * 🎯封装`match`逻辑，提取输出中可能的Narsese
    /// * 📌可能有，也可能没有
    /// * 🚩【2024-03-28 15:01:57】目前不区分「类型本身就没有」与「类型支持，但未存储」
    pub fn get_narsese(&self) -> Option<&LexicalNarsese> {
        match self {
            Output::IN { narsese, .. }
            | Output::OUT { narsese, .. }
            | Output::ANSWER { narsese, .. }
            | Output::ACHIEVED { narsese, .. }
            // * 📝从`&Option<T>`变成`Option<&T>`的方法，直接使用[`Option::as_ref`]
            | Output::UNCLASSIFIED { narsese, .. } => narsese.as_ref(),
            // ! 使用通配符可能意味着后续「在别的类型中添加了Narsese字段，但不会被处理」的情况
            _ => None,
        }
    }

    /// 获取Narsese操作（专有的[`Operation`]）
    /// * 🎯封装`match`逻辑，提取输出中可能的Narsese操作
    /// * 🎯与后续「输出预期」中的「Narsese操作」对齐：允许`.nal`语法中统一解析操作
    /// * 📌可能有，也可能没有
    ///   * 🚩【2024-03-31 17:07:23】目前只有输出类型[`Output::EXE`]会有
    pub fn get_operation(&self) -> Option<&Operation> {
        match self {
            Output::EXE { operation, .. } => Some(operation),
            // ! 使用通配符可能意味着后续「在别的类型中添加了Narsese字段，但不会被处理」的情况
            _ => None,
        }
    }
}

/// 表征一个「NARS操作」
/// * 直接对应各CIN中形如「操作(参数1, 参数2, ...)」
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Operation {
    // 操作符名
    // * 📄如：`left` `right` `deactivate`
    //
    // ! ⚠️不带尖号
    pub operator_name: String,

    /// 操作的参数（词项数组）
    pub params: Vec<LexicalTerm>,
}

impl Operation {
    /// 构造函数
    /// * ℹ️若需从[`String`]与[`Vec`]直接构造，请直接使用结构体字面量语法
    ///   * 📄参见[`Operation`]
    pub fn new(operator_name: &str, params: impl Iterator<Item = LexicalTerm>) -> Self {
        Self {
            operator_name: operator_name.into(),
            params: params.collect(),
        }
    }

    /// 构造函数/从字符串迭代器构造
    /// * ℹ️若需从[`String`]与[`Vec`]直接构造，请直接使用结构体字面量语法
    ///   * 📄参见[`Operation`]
    pub fn try_from_strings(
        operator_name: &str,
        params_str: impl Iterator<Item = impl AsStrRef>,
    ) -> Result<Self> {
        // 先解析参数
        let mut params = vec![];
        for param in params_str {
            let parsed = FORMAT_ASCII.parse(param.as_str_ref())?.try_into_term()?;
            params.push(parsed);
        }
        // 构造自身并返回
        Ok(Self {
            operator_name: operator_name.into(),
            params,
        })
    }

    /// 转换为JSON字符串
    /// * 🚩转换为JSON字符串数组
    /// * 🚩使用不带空白符的「最密版本」
    /// * 🚩【2024-04-09 11:05:01】目前暂不使用[`serde_json`]
    pub fn to_json_string(&self) -> String {
        format!(
            "[{:?},{:?}]",
            &self.operator_name,
            self.params
                .iter()
                .map(|t| FORMAT_ASCII.format(t))
                .join_to_new(",")
        )
    }

    /// 判断是否没参数
    /// * 🎯在「预期匹配」中作为「通配符」使用
    /// * 📄「无参操作」如`^left`仅在ONA中出现过
    /// * 🚩直接调用[`Vec::is_empty`]，少用一个逻辑取反
    #[inline]
    pub fn no_params(&self) -> bool {
        self.params.is_empty()
    }

    /// 判断是否有参数
    /// * 📄相对[`Self::no_params`]而言
    /// * 🚩多一个逻辑取反
    #[inline]
    pub fn has_params(&self) -> bool {
        !self.no_params()
    }
}

/// 呈现
/// * 🎯格式化成一个CommonNarsese词项（陈述）
impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 操作符
        write!(f, "<(*")?;
        for param in self.params.iter() {
            write!(f, ", {}", FORMAT_ASCII.format(param))?;
        }
        write!(f, ") --> ^{}>", self.operator_name)
    }
}

/// 转换为纯字符串数组
impl From<Operation> for Vec<String> {
    fn from(value: Operation) -> Self {
        // 首先提取其元素
        let Operation {
            operator_name,
            // 将「参数」换成可变的「返回值」
            params,
        } = value;

        // 创建返回值，自动包含头
        let mut result = vec![operator_name];

        // 然后逐个添加内部词项的字符串形式
        for param in params {
            result.push(FORMAT_ASCII.format(&param));
        }

        // 返回
        result
    }
}

/// 快捷构造宏
#[macro_export]
macro_rules! operation {
    ($operator_name:expr => $($param:expr)*) => {
        Operation{
            operator_name: $operator_name.into(),
            params: vec![$(
                FORMAT_ASCII.parse($param.as_str_ref()).unwrap().try_into_term().unwrap()
            ),*]
        }
    };
}

/// 单元测试
/// * 🎯需要产生测试集给其它地方用
/// * 🚩【2024-03-21 12:44:23】此处模块必须使用不一样的名称
///   * 📌原因：`output`模块直接被`vm`重导出
#[cfg(test)]
pub mod tests {
    use super::*;
    use narsese::lexical_nse;
    use Output::*;

    /// 产生测试样本集
    pub fn test_samples() -> Vec<Output> {
        vec![
            IN {
                content: "in".into(),
                narsese: Some(lexical_nse!("<in --> out>")),
            },
            OUT {
                content_raw: "out".into(),
                narsese: Some(lexical_nse!(<A --> C>.)),
            },
            ERROR {
                description: "err".into(),
            },
            ANSWER {
                narsese: Some(lexical_nse!(<A --> B>.)),
                content_raw: "answer".into(),
            },
            ACHIEVED {
                content_raw: "achieved".into(),
                narsese: Some(lexical_nse!(G.)),
            },
            EXE {
                content_raw: "EXE: ^left({SELF})".into(),
                operation: operation!("left" => "{SELF}"),
            },
            INFO {
                message: "info".into(),
            },
            COMMENT {
                content: "comment".into(),
            },
            TERMINATED {
                description: "terminated".into(),
            },
            UNCLASSIFIED {
                r#type: "unclassified".to_uppercase(),
                content: "unclassified".into(),
                narsese: Some(lexical_nse!(<A --> B>.)),
            },
            OTHER {
                content: "other".into(),
            },
        ]
    }
}
