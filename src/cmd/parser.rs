//! 定义「NAVM指令」的简易解析器
//! * 从字符串简要解析出NAVM指令指令类型

use super::Cmd;
use narsese::conversion::string::impl_lexical::format_instances::FORMAT_ASCII;
use std::{error::Error, fmt::Display};
use util::*;

/// * 📝定长数组非Copy初始化：如果需要在定长数组中初始化一个方法，应该先声明一个const，然后从中初始化
const EMPTY_STRING: std::string::String = String::new();
/// 封装「获取N个命令参数」的功能
fn get_cmd_params<const N: usize>(s: &str) -> ParseResult<[String; N]> {
    let mut split = s.split_whitespace();

    // 初始化，拷贝N个空字串
    let mut result: [String; N] = [EMPTY_STRING; N];
    #[allow(clippy::needless_range_loop)] // ! 此处因为需要中断返回，所以无法用Clippy简化
    for i in 0..N {
        match split.next() {
            None => return Err(ParseError(format!("参数个数不足{N}个！"))),
            Some(s) => result[i].push_str(s),
        }
    }
    // 开始拆分：过长⇒忽略，过短⇒报错
    Ok(result)
}

/// 封装「指令解析结果」相关功能
mod parse_error {
    use super::*;

    /// 解析错误的类型
    #[derive(Debug)]
    pub struct ParseError(pub String);

    impl ParseError {
        pub fn new(s: &str) -> ParseError {
            ParseError(s.to_string())
        }
    }
    impl Display for ParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "NAVM Cmd ParseError: {}", self.0)
        }
    }
    impl Error for ParseError {
        // 本身就只包含描述
        fn description(&self) -> &str {
            &self.0
        }
    }

    /// * 🎯从其它「错误」类型转换到「解析错误」
    /// * ⚠️实现[`From`]特征会起冲突
    pub fn to_parse_error(e: impl ToString) -> ParseError {
        ParseError(e.to_string())
    }

    /// 简记的类型别名：解析结果
    pub type ParseResult<T> = Result<T, ParseError>;
}
use parse_error::*;

/// 扩展指令[`Cmd`]类型的功能
impl super::Cmd {
    /// 从字符串构造NAVM指令
    pub fn parse(line: &str) -> ParseResult<Self> {
        // 空字串
        if_return! {
            line.trim().is_empty() => Err(ParseError::new("尝试解析空行！"))
        }
        // 拆分字符串为两个部分
        let (head, params) = line
            .split_once(char::is_whitespace)
            .ok_or(ParseError::new("无法分割出指令头！"))?;
        // 构造指令
        Self::parse_str_params(head, params)
    }

    /// 从字符串参数中构造NAVM指令
    /// * 🚩除了「指令头」以外，均为「指令行」
    ///   * ⚠️「指令行」不包括「指令头」
    pub fn parse_str_params(head: &str, line: &str) -> ParseResult<Self> {
        Ok(match head.to_uppercase().as_str() {
            // 内置：各自有各自的处理方法
            "SAV" => {
                // 以空格分隔
                let [target, path] = get_cmd_params::<2>(line)?;
                Cmd::SAV { target, path }
            }
            "LOA" => {
                // 以空格分隔
                let [target, path] = get_cmd_params::<2>(line)?;
                Cmd::LOA { target, path }
            }
            "RES" => {
                // 以空格分隔
                let [target] = get_cmd_params::<1>(line)?;
                Cmd::RES { target }
            }
            "NSE" => {
                // 🚩以CommonNarsese ASCII语法解析出「词法Narsese」
                // * 📌此处旨在统一格式，如`NSE <A --> B>.`
                // * 📌【2024-03-22 17:45:47】至于「转换为子程序输入」的形式，这是留给后续运行时的
                let narsese = FORMAT_ASCII
                    // 尝试解析
                    .parse(line)
                    // 转换其中的错误类型
                    .transform_err(to_parse_error)?;
                // 尝试进行隐式转换，以统一使用`Task`类型
                // * ⚠️其中的「语句」将会被转换为「空预算任务」
                let task = narsese
                    .try_into_task_compatible()
                    .transform_err(to_parse_error)?;
                // 返回
                Cmd::NSE(task)
            }
            "NEW" => {
                // 以空格分隔
                let [target] = get_cmd_params::<1>(line)?;
                Cmd::NEW { target }
            }
            "DEL" => {
                // 以空格分隔
                let [target] = get_cmd_params::<1>(line)?;
                Cmd::DEL { target }
            }
            "CYC" => {
                // 以空格分隔
                let [num_str] = get_cmd_params::<1>(line)?;
                let num = match num_str.parse::<usize>() {
                    Ok(n) => n,
                    Err(e) => return Err(to_parse_error(e)),
                };
                Cmd::CYC(num)
            }
            "VOL" => {
                // 以空格分隔
                let [num_str] = get_cmd_params::<1>(line)?;
                let num = match num_str.parse::<usize>() {
                    Ok(n) => n,
                    Err(e) => return Err(to_parse_error(e)),
                };
                Cmd::VOL(num)
            }
            "REG" => {
                // 以空格分隔
                let [name] = get_cmd_params::<1>(line)?;
                Cmd::REG { name }
            }
            "INF" => {
                // 以空格分隔
                let [target] = get_cmd_params::<1>(line)?;
                Cmd::INF { source: target }
            }
            "HLP" => {
                // 以空格分隔
                let [name] = get_cmd_params::<1>(line)?;
                Cmd::HLP { name }
            }
            "REM" => Cmd::REM {
                comment: line.into(),
            },
            "EXI" => Cmd::EXI {
                reason: line.into(),
            },
            // 自定义：存入「自定义」类型中
            other => Self::Custom {
                head: other.into(),
                tail: line.into(),
            },
        })
    }
}

/// 单元测试
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_ascii_whitespace() {
        let s = get_cmd_params::<3>("a b \tc").unwrap();
        // 能解析出来就是成功
        assert_eq!(dbg!(s), ["a", "b", "c"]);
    }

    /// 测试/解析单个指令
    /// * 🎯保证「正常指令解析不出错」
    fn _test_parse(cmd_str: &str) -> Cmd {
        let cmd = Cmd::parse(cmd_str).expect("NAVM指令解析失败");
        dbg!(cmd)
    }

    /// 测试/解析
    #[test]
    fn test_parse() {
        let cmd_lines = "
        SAV target path
        LOA target path
        RES target
        NSE <(&&, <A --> $B>, <#C --> +1>) --> ^D>. :|: %1.0; 0.9%
        NEW reasoner
        DEL reasoner
        CYC 137
        VOL 0
        REG operator_name
        INF memory
        HLP self
        REM this is a comment or remark
        CUSTOM_HEAD tail
        "
        .trim();

        // 逐行解析
        for line in cmd_lines.lines().map(str::trim) {
            _test_parse(line);
        }
    }
}
