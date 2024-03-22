//! 定义NAVM的输出类型
//! * 🚩【2024-03-21 11:34:10】目前使用枚举实现
//!
//! # Reference
//!
//! 参考**BabelNAR**中的如下Julia代码：
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

use narsese::lexical::Narsese as LexicalNarsese;

/// NAVM输出类型
/// * 🎯使用枚举，统一对「输出类别」分派
/// * 📌除其中的[`String`]类型字段，通用于所有具体实现
///   * 📄与具体NAVM实现无关
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum Output {
    /// 表示「已输入信息」的recall
    /// * 📌该「信息」一般是Narsese字符串
    ///   * 如各类CIN对Narsese输入的回显
    /// * 📄样例 @ ONA: `Input: <A --> B>. Priority=1.000000 Truth: frequency=1.000000, confidence=0.900000\n`
    /// * ⚠️部分CIN可能不会输出
    IN { content: String },

    /// 表示「的一般输出信息」的recall
    /// * 🎯一般「推理导出结论」等不太重要的信息
    /// * 📄样例 @ ONA: `Derived: <A --> C>. Priority=0.407250 Truth: frequency=1.000000, confidence=0.810000\n`
    ///
    /// ! ⚠️【2024-03-22 18:28:12】现在将「是否需要在所有『CIN输出』中提取统一的Narsese」**交给各大运行时**
    OUT {
        /// 原始内容
        content_raw: String,
        /// （可能有的）Narsese内容（词法Narsese）
        /// * ⚠️具体实现交给各大运行时
        narsese: Option<LexicalNarsese>,
    },

    /// 表示「内部错误」的信息
    /// * 🎯一般传递「内部发生了一个错误，可能需要处理」
    /// * 📄例如OpenNARS中的`[ERR]: NullPointer Exception...`
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
        /// * ⚠️具体实现交给各大运行时
        narsese: Option<LexicalNarsese>,
    },

    /// 表示「输出一个『完成』」
    /// * 🎯一般各CIN对「目标」语句的「完成」
    /// * 🚩内部一般是相应的Narsese文本
    /// * 📄最初见于PyNARS（🔗[原PR](https://github.com/bowen-xu/PyNARS/pull/30)）
    ///
    /// ! ⚠️【2024-03-22 18:28:12】现在将「是否需要在所有『CIN输出』中提取统一的Narsese」**交给各大运行时**
    ACHIEVED {
        /// 原始内容
        content_raw: String,

        /// （可能有的）Narsese内容（词法Narsese）
        /// * ⚠️具体实现交给各大运行时
        narsese: Option<LexicalNarsese>,
    },

    /// 表示「输出一个操作」
    /// * 🎯一般表示各CIN「需要调用外部 代码/程序」的信号
    /// * 🚩内部封装专有数据结构
    ///   * 📌之所以不内联，是因为改数据结构后续还要进行使用
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
    ///  * 📄如：（OpenNARS）`[l]: attaching Shell to Nar...`
    ///  * 📄如：（PyNARS）``
    INFO { message: String },

    /// 表示「输出一条注释」
    /// * 🎯一般表示（比OUT、INFO）更无关紧要的输出
    /// * 📄最初见于PyNARS
    COMMENT { content: String },

    /// 表示「『预期』某个事件发生」
    /// * 🎯一般表示CIN（NAL 7~9）的高阶行为
    /// * 📄最初见于OpenNARS
    ///
    /// ! ⚠️【2024-03-22 18:28:12】现在将「是否需要在所有『CIN输出』中提取统一的Narsese」**交给各大运行时**
    ANTICIPATE {
        /// 原始内容
        content_raw: String,

        /// （可能有的）Narsese内容（词法Narsese）
        /// * ⚠️具体实现交给各大运行时
        narsese: Option<LexicalNarsese>,
    },

    /// 表示其它CIN输出
    /// * 🎯用于表示「可以识别到类型，但不在此枚举中」的NAVM输出
    ///   * 📌针对一些特定CIN的方言使用
    ///   * 📌针对后续「使用模式匹配识别出的类型」使用
    UNCLASSIFIED { r#type: String, content: String },

    /// 表示其它CIN输出
    /// * 🎯一般表示「暂无法格式化识别」的其它CIN输出
    ///   * 📌大多数时候无关紧要
    ///   * 🎯一般对应一行输出
    /// * 📄样例 @ OpenNARS: `Got relative path for loading the config: ./config/defaultConfig.xml`
    /// * 📄样例 @ OpenNARS: `executed based on [...]`
    ///   * 📝操作执行的证据基础，用于验证「系统是否习得知识」
    OTHER { content: String },
}

impl Output {
    /// 判断「NAVM输出」的类型
    /// * 📌【2024-03-21 11:36:49】使用[`str`]静态返回
    /// * 🚩直接`match`并返回**全大写**英文
    #[inline]
    pub fn type_name(&self) -> &str {
        match self {
            Output::IN { .. } => "IN",
            Output::OUT { .. } => "OUT",
            Output::ERROR { .. } => "ERROR",
            Output::ANSWER { .. } => "ANSWER",
            Output::ACHIEVED { .. } => "ACHIEVED",
            Output::EXE { .. } => "EXE",
            Output::INFO { .. } => "INFO",
            Output::COMMENT { .. } => "COMMENT",
            Output::ANTICIPATE { .. } => "ANTICIPATE",
            // ! 特别的「未分类」情形：使用其中预置的「类名」
            Output::UNCLASSIFIED { r#type, .. } => r#type.as_str(),
            Output::OTHER { .. } => "OTHER",
        }
    }
}

/// 表征一个「NARS操作」
/// * 直接对应各CIN中形如「操作(参数1, 参数2, ...)」
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Operation {
    // 操作头名
    // * 📄如：`left` `right` `deactivate`
    //
    // ! ⚠️不带尖号
    pub head: String,

    // 操作的参数
    pub params: Vec<String>,
}

impl Operation {
    /// 构造函数
    pub fn new(operator_name: &str, params: impl Iterator<Item = String>) -> Self {
        Self {
            head: operator_name.into(),
            params: params.collect(),
        }
    }

    /// 转换为JSON字符串
    /// * 🚩使用不带空白符的「最密版本」
    pub fn to_json_string(&self) -> String {
        format!("[{},{}]", &self.head, self.params.join(","))
    }
}

/// 转换为纯字符串数组
impl From<Operation> for Vec<String> {
    fn from(value: Operation) -> Self {
        // 首先提取其元素
        let Operation {
            head,
            // 将「参数」换成可变的「返回值」
            params: mut result,
        } = value;
        // 然后将头添加进返回值中
        result.insert(0, head);
        // 返回「参数」
        result
    }
}

/// 快捷构造宏
#[macro_export]
macro_rules! operation {
    ($operator_name:expr => $($param:expr)*) => {
        Operation{ head: $operator_name.into(), params: vec![$($param.into()),*] }
    };
}

/// 单元测试
/// * 🎯需要产生测试集给其它地方用
/// * 🚩【2024-03-21 12:44:23】此处模块必须使用不一样的名称
///   * 📌原因：`output`模块直接被`vm`重导出
#[cfg(test)]
pub mod tests {
    use super::*;
    use Output::*;

    /// 产生测试样本集
    pub fn test_samples() -> Vec<Output> {
        vec![
            IN {
                content: "in".into(),
            },
            OUT {
                content_raw: "out".into(),
                narsese: None,
            },
            ERROR {
                description: "err".into(),
            },
            ANSWER {
                narsese: None,
                content_raw: "answer".into(),
            },
            ACHIEVED {
                content_raw: "achieved".into(),
                narsese: None,
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
            ANTICIPATE {
                content_raw: "anticipate".into(),
                narsese: None,
            },
            OTHER {
                content: "other".into(),
            },
        ]
    }
}
