/// NAVM输出类型
/// * 🎯使用枚举，统一对「输出类别」分派
///
/// # Reference
///
/// 参考**BabelNAR**中的如下Julia代码：
/// ```julia
/// NARSOutputType = (;
///     IN = "IN",
///     OUT = "OUT",
///     ERROR = "ERROR",
///     ANSWER = "ANSWER",
///     ACHIEVED = "ACHIEVED",
///     EXE = "EXE",
///     INFO = "INFO",
///     COMMENT = "COMMENT",
///     ANTICIPATE = "ANTICIPATE",
///     OTHER = "OTHER"
///     # *【2024-01-25 15:27:03】`OTHER`类型用于存储「暂无法格式化识别」的其它信息
///     #     * @example 如OpenNARS的`executed based on`（操作执行的证据基础，用于验证「系统是否习得知识」）
///     #     * 🎯用于在后续实验中提取「推理器特异」的实用信息
/// )
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Output {
    /// 表示「CIN已输入信息」的recall
    /// * 📌该「信息」一般是Narsese字符串
    /// * ⚠️部分CIN可能不会输出
    IN { content: String },

    /// 表示「CIN的一般输出信息」的recall
    /// * 📌一般用于「推理导出结论」等不太重要的信息
    OUT { content: String },

    /// 表示「CIN内部错误」的信息
    /// * 📌一般用于传递「内部发生了一个错误，可能需要处理」
    /// * 📄例如OpenNARS中的`[ERR]: NullPointer Exception...`
    ERROR { description: String },

    /// 表示「CIN输出一个『回答』」
    /// * 📌一般用于各CIN对「问题」语句的「回答」
    /// * 🚩内部一般是相应的Narsese文本
    ///
    /// TODO: 后续可能需要统一成CommonNarsese？目前尚未对此进行利用
    ANSWER { narsese: String },

    /// 表示「CIN输出一个『完成』」
    /// * 📌一般用于各CIN对「目标」语句的「完成」
    /// * 🚩内部一般是相应的Narsese文本
    /// * 📄最初见于PyNARS
    ///
    /// TODO: 后续可能需要统一成CommonNarsese？目前尚未对此进行利用
    ACHIEVED { narsese: String },

    /// 表示「CIN输出一个操作」
    /// * 📌一般用于表示各CIN「需要调用外部 代码/程序」的信号
    /// * 🚩直接封装专有数据结构
    ///   * 📌之所以不内联，是因为改数据结构后续还要进行使用
    EXE(Operation),

    /// 表示「CIN输出一条信息」
    /// * 📌一般是CIN输出的各种（无关紧要的）提示信息
    ///  * 📄如：（OpenNARS）`[l]: attaching Shell to Nar...`
    ///  * 📄如：（PyNARS）``
    INFO { message: String },

    /// 表示「CIN输出一条注释」
    /// * 📌一般用于表示（比OUT、INFO）更无关紧要的输出
    /// * 📄最初见于PyNARS
    ///
    /// ? 💭似乎已经不知道是哪儿来的了
    ///
    COMMENT { content: String },

    /// 表示「CIN『预期』某个事件发生」
    /// * 📌一般用于表示CIN（NAL 7~9）的高阶行为
    /// * 📄最初见于OpenNARS
    ///
    /// TODO: 后续实际上需要进一步细化？比如提取其中的Narsese内容
    ANTICIPATE { content: String },

    /// 表示其它CIN输出
    /// * 📌一般用于存储「暂无法格式化识别」的其它CIN输出
    ///   * 📌大多数时候无关紧要
    ///   * 📌一般对应一行输出
    /// * 📄如OpenNARS`Got relative path for loading the config: ./config/defaultConfig.xml`
    /// * 📄如OpenNARS`executed based on`（操作执行的证据基础，用于验证「系统是否习得知识」）
    ///
    /// TODO: 后续实际上需要进一步细化？比如提取其中的Narsese内容
    OTHER { content: String },
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
    pub fn new(operator_name: &str, params: impl Iterator<Item = String>) -> Self {
        Self {
            head: operator_name.into(),
            params: params.collect(),
        }
    }
}

#[macro_export]
macro_rules! operation {
    ($operator_name:expr => $($param:expr)*) => {
        Operation{ head: $operator_name.into(), params: vec![$($param.into()),*] }
    };
}
