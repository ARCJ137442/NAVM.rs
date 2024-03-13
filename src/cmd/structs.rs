//! 建立NAVM指令的数据结构

use enum_narsese::lexical::LexicalNarsese;

/// ! 暂时使用[`String`]作为Narsese对象的占位符
///   * 后续需要更结构化的数据，可能[`LexicalNarsese`]是首选
pub type Narsese = LexicalNarsese;

/// NAVM指令 数据结构
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cmd {
    /// `Save` | 在path保存target
    /// 指令：保存当前数据（记忆）到文件
    SAV { target: String, path: String },

    /// `Load` | 从path加载target
    /// 指令：从文件加载数据（记忆）
    LOA { target: String, path: String },

    /// `Reset` | 重置target
    /// 指令：清除CIN数据
    /// - 如：记忆区、缓冲区……
    RES { target: String },

    /// 指令：输入「CommonNarsese」语句
    /// - 不换行
    /// - 遵循CommonNarsese语法
    NSE(Narsese),

    /// `New` | 新建target
    /// 指令：创建新推理器
    NEW { target: String },

    /// `Delete` | 删除target
    /// 指令：删除(停止)推理器
    DEL { target: String },

    /// `Cycle`
    /// 指令：控制CIN步进
    CYC(usize),

    /// `Volume`
    /// 指令：控制CIN输出音量
    VOL(usize),

    /// `Register`
    /// 指令：向CIN注册操作（NAL-8）
    /// - 📌此处的「操作符名」不带尖号「^」，等价于「原子操作」
    REG { name: String },

    /// `Info`
    /// 指令：让CIN输出某类信息
    INF { target: String },

    /// `Help` | 帮助
    /// 指令：打印（CIN的）帮助文档
    HLP { name: String },

    /// `Remark`
    /// 指令：注释
    /// - 📌仅存储内容，后续通常翻译为空字串
    REM { comment: String },

    /// `Custom`
    /// 自定义指令
    ///
    /// * 🎯后续一切「VM特定指令」的后门类型
    /// * 🚩【2024-03-04 23:26:29】目前不使用「特征对象」的形式
    ///   * 🚩改为更通用、词法上更灵活的「字符串参数行」形式
    ///     * 【2024-03-05 01:09:27】**不默认使用空格分隔**
    ///   * 📌关键在于「内容完全限定」「后续容易『特殊VM特殊处理』」
    /// * 📌使用正常命名法，以区分其它作为「内置指令」的类型
    // Custom { cmd: Box<dyn NAVM指令Cmd> },
    Custom { head: String, args_line: String },
}

// /// 用于兼容枚举之外的自定义指令
// /// * 📌内涵有待增加
// pub trait NAVM指令Cmd {
//     // /// 从字符串解析
//     // /// * 💭【2024-02-29 00:26:48】似乎不应归特征管
//     // fn from_str(s: &str) -> Option<Self>;

//     /// 转换为字符串（NAVM指令形式）
//     fn to_str(&self) -> String;
// }
