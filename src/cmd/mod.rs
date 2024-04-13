//! 提供对「NAVM指令」的数据结构、解析支持

util::mod_and_pub_use! {
    // 结构
    structs
    // 格式化
    formatter
    // 解析
    parser
}

/// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use util::asserts;

    /// 测试/转换
    /// * 🎯解析、格式化的稳定性：相等的指令
    #[test]
    fn test_conversion() {
        // 取样本集
        let samples = super::structs::tests::samples();
        // 逐个展开测试
        for cmd in samples {
            // 格式化
            let cmd_str = String::from(&cmd);
            // 重解析
            let reconverted = Cmd::parse(&cmd_str).expect("指令重解析失败");
            // 重解析
            let reformatted = reconverted.to_string();
            // 比对
            asserts! {
                // 重新解析的指令应与原指令相等
                cmd => reconverted
                // 重新格式化后的文本应与首次格式化后的文本相等
                cmd_str => reformatted
            };
        }
    }
}
