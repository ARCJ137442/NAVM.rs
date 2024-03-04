//! 定义「NAIR中间语」的简易解析器
//! * 从字符串简要解析出NAIR指令类型
//!
//! TODO: 有待完成

impl super::Cmd {
    /// 从字符串参数中构造NAIR中间语
    /// * 🚩除了「指令头」以外，均为「指令行」（不包括指令头）
    pub fn from_str_params(head: &str, line: &str) -> Self {
        match head {
            // 内置：各自有各自的处理方法
            "SAV" => todo!(),
            "LOA" => todo!(),
            "RES" => todo!(),
            "NSE" => todo!(),
            "NEW" => todo!(),
            "DEL" => todo!(),
            "CYC" => todo!(),
            "VOL" => todo!(),
            "REG" => todo!(),
            "INF" => todo!(),
            "HLP" => todo!(),
            "REM" => todo!(),
            // 自定义：存入「自定义」类型中
            other => Self::Custom {
                head: other.into(),
                args_line: line.into(),
            },
        }
    }
}
