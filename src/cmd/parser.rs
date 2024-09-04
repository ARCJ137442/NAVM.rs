//! 定义「NAVM指令」的简易解析器
//! * 从字符串简要解析出NAVM指令指令类型

use super::Cmd;
use nar_dev_utils::{if_return, pipe};
use narsese::conversion::string::impl_lexical::format_instances::FORMAT_ASCII;
use std::{error::Error, fmt::Display};

/// 固定的「空字串」常量
/// * 📝定长数组非Copy初始化：如果需要在定长数组中初始化一个方法，应该先声明一个const，然后从中初始化
const EMPTY_STRING: std::string::String = String::new();

/// 封装「获取N个命令参数」的功能
/// * 🚩【2024-07-02 01:25:18】目前提取出两个函数的共同逻辑，其差异通过闭包体现
#[inline(always)]
fn _get_cmd_params<const N: usize>(
    s: &str,
    split_next_handler: impl Fn(Option<&str>) -> Result<&str, ParseError>,
) -> ParseResult<[String; N]> {
    // 先拆分空格（连续空格缩并）
    let mut split = s.split_whitespace();

    // 初始化，拷贝N个空字串
    let mut result: [String; N] = [EMPTY_STRING; N];
    for result_i in &mut result {
        pipe! {
            split.next() // 取下一个参数
            => split_next_handler // 拆分下一个参数：取默认值，或报错
            => {?}# // 错误上报
            => [result_i.push_str] // 加入参数
        };
    }
    // 开始拆分：过长⇒忽略，过短⇒报错
    Ok(result)
}
/// 封装「获取N个命令参数」的功能
fn get_cmd_params<const N: usize>(s: &str) -> ParseResult<[String; N]> {
    _get_cmd_params(s, |s| {
        // 在「遇到空值」时报错
        s.ok_or_else(|| ParseError(format!("参数个数不足{N}个！")))
    })
}

/// 封装「获取N个命令参数」的功能，但对空值取空字串
fn get_cmd_params_loose<const N: usize>(s: &str) -> ParseResult<[String; N]> {
    // 在参数缺省时取空字串
    _get_cmd_params(s, |s| Ok(s.unwrap_or("")))
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
        // * 📜默认情况：整个指令都是指令头（无参数）
        let (head, params) = line.split_once(char::is_whitespace).unwrap_or((line, ""));
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
                let [target, path] = get_cmd_params_loose::<2>(line)?;
                Cmd::SAV { target, path }
            }
            "LOA" => {
                // 以空格分隔
                let [target, path] = get_cmd_params_loose::<2>(line)?;
                Cmd::LOA { target, path }
            }
            "RES" => {
                // 以空格分隔 | 此处为「松弛获取」：缺省的参数允许填充空格
                let [target] = get_cmd_params_loose::<1>(line)?;
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
                    .map_err(to_parse_error)?;
                // 尝试进行隐式转换，以统一使用`Task`类型
                // * ⚠️其中的「语句」将会被转换为「空预算任务」
                let task = narsese.try_into_task_compatible().map_err(to_parse_error)?;
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
                let num = num_str.parse::<usize>().map_err(to_parse_error)?;
                Cmd::CYC(num)
            }
            "VOL" => {
                // 以空格分隔
                let [num_str] = get_cmd_params::<1>(line)?;
                let num = num_str.parse::<usize>().map_err(to_parse_error)?;
                Cmd::VOL(num)
            }
            "REG" => {
                // 以空格分隔
                let [name] = get_cmd_params::<1>(line)?;
                Cmd::REG { name }
            }
            "INF" => {
                // 以空格分隔
                let [source] = get_cmd_params_loose::<1>(line)?;
                Cmd::INF { source }
            }
            "HLP" => {
                // 以空格分隔 | 此处为「松弛获取」：缺省的参数允许填充空格
                let [name] = get_cmd_params_loose::<1>(line)?;
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

    /// 工具函数/逐行测试
    fn _test_lines(lines: &str) {
        // 逐行解析
        for line in lines.lines().map(str::trim).filter(|line| !line.is_empty()) {
            _test_parse(line);
        }
    }

    /// 测试/解析
    #[test]
    fn test_parse() {
        _test_lines(
            "
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
            EXI reason of exit
            CUSTOM_HEAD tail
            ",
        )
    }

    /// 测试/解析/无附加参数的「松弛解析」
    #[test]
    fn test_parse_no_tail() {
        _test_lines(
            "
            EXI
            REM 以下均为「松弛解析」的用例

            SAV
            LOA
            SAV reasoner
            LOA reasoner
            SAV reasoner ./saves/reasoner
            LOA reasoner ./saves/reasoner
            RES
            RES resetted
            INF
            INF memory
            HLP
            HLP *
            ",
        )
    }
}
