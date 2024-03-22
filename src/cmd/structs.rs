//! 建立NAVM指令的数据结构
//! * ✨现在对指令[`Cmd::NSE`]引入的是「词法Narsese」，保证所输入Narsese的词法正确性
//!   * 【2024-03-22 17:34:48】⚠️也有可能是一种限制

use narsese::lexical::Task as LexicalTask;

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
    /// * 如：记忆区、缓冲区……
    RES { target: String },

    /// 指令：输入「CommonNarsese」语句
    /// * 🚩使用「词法Narsese」（[`narsese::lexical`]）作为数据结构
    /// * 📌【2024-03-22 17:40:15】此处只使用其中的「任务」结构
    ///   * 📄目前OpenNARS、ONA、NARS-Python、PyNARS、OpenJunars等均以「任务」作为输入单位
    ///   * 📌对于「语句」的情况，也可以通过「附加『空预算』」隐式转换为任务
    NSE(LexicalTask),

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
    /// * 📌此处的「操作符名」不带尖号「^」，等价于「原子操作」
    REG { name: String },

    /// `Info`
    /// 指令：让CIN输出某类信息
    INF { target: String },

    /// `Help` | 帮助
    /// 指令：打印（CIN的）帮助文档
    HLP { name: String },

    /// `Remark`
    /// 指令：注释
    /// * 📌仅存储内容，后续通常翻译为空字串
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
