//! 提供对「NAVM指令」的字符串格式化支持
//! * 📌统一格式，不以具体CIN为转移
//!   * 📄指令[`Cmd::NSE`]只使用**CommonNarsese**语法
//!
use super::Cmd;
use narsese::{
    api::TryCastToSentence, conversion::string::impl_lexical::format_instances::FORMAT_ASCII,
    lexical::Narsese,
};
use util::ResultBoostSingular;

impl Cmd {
    /// 获取指令头
    /// * 🚩直接匹配并返回静态字串引用
    pub fn head(&self) -> &str {
        match self {
            Cmd::SAV { .. } => "SAV",
            Cmd::LOA { .. } => "LOA",
            Cmd::RES { .. } => "RES",
            Cmd::NSE(..) => "NSE",
            Cmd::NEW { .. } => "NEW",
            Cmd::DEL { .. } => "DEL",
            Cmd::CYC(..) => "CYC",
            Cmd::VOL(..) => "VOL",
            Cmd::REG { .. } => "REG",
            Cmd::INF { .. } => "INF",
            Cmd::HLP { .. } => "HLP",
            Cmd::REM { .. } => "REM",
            Cmd::Custom { head, .. } => head,
        }
    }

    /// 获取指令尾
    /// * 🚩直接匹配并返回动态字串[`String`]
    /// * 🎯便于后续重复利用
    pub fn tail(&self) -> String {
        match self {
            // 目标+路径
            Cmd::SAV { target, path } | Cmd::LOA { target, path } => format!("{} {}", target, path),
            // 目标
            Cmd::RES { target }
            | Cmd::NEW { target }
            | Cmd::DEL { target }
            | Cmd::INF { target } => target.clone(),
            // 词法Narsese
            // * 🚩【2024-03-24 03:36:40】目前将尝试先「将『空预算任务』隐式转换为语句」然后再进行格式化
            //   * 📌避免「空预算任务」`A.`变为`$$ A.`导致的「非法输入」情况
            //   * 💭虽说后续大概率不会再以此作为直接输入
            //   * 💫不可避免地要进行一次拷贝（相比「格式化器层面的向下转换」还是差了点性能）
            Cmd::NSE(narsese) => FORMAT_ASCII.format_narsese(
                &Narsese::Task(narsese.clone())
                    .try_cast_to_sentence()
                    .collapse(),
            ),
            // 数值
            Cmd::CYC(n) | Cmd::VOL(n) => n.to_string(),
            // 名称
            Cmd::REG { name } | Cmd::HLP { name } => name.clone(),
            // 注释
            Cmd::REM { comment } => comment.clone(),
            Cmd::Custom {
                tail: args_line, ..
            } => args_line.clone(),
        }
    }
}

impl From<&Cmd> for String {
    fn from(cmd: &Cmd) -> Self {
        // 新字串
        let mut s = Self::new();
        // 通用：指令头
        s.push_str(cmd.head());
        // 空格分隔
        s.push(' ');
        // 专用：指令尾
        s.push_str(&cmd.tail());
        // 返回
        s
    }
}

impl ToString for Cmd {
    fn to_string(&self) -> String {
        self.into()
    }
}

/// 单元测试
#[cfg(test)]
mod tests {

    /// 测试/格式化
    #[test]
    fn test_format() {
        // 取样本集
        let samples = super::super::structs::tests::samples();
        // 逐个格式化并打印
        for cmd in samples {
            println!("{}", cmd.to_string());
        }
    }
}
