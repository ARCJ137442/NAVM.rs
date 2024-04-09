//! 简单演绎推理机
//! * 🎯展示NAVM的实现
//! * 🎯展示IL-1的演绎推理

mod graph;
use graph::*;

use anyhow::{anyhow, Result};
use nar_dev_utils::list;
use narsese::{
    api::{FormatTo, GetPunctuation, GetTerm},
    conversion::{
        inter_type::lexical_fold::TryFoldInto,
        string::{
            impl_enum::{format_instances::FORMAT_ASCII, NarseseFormat},
            impl_lexical::format_instances::FORMAT_ASCII as FORMAT_ASCII_LEXICAL,
        },
    },
    enum_narsese::{Sentence::Judgement, Stamp, Task, Term, Truth},
    lexical::Narsese as LexicalNarsese,
};
use navm::{
    cmd::Cmd,
    output::Output,
    vm::{VmLauncher, VmRuntime, VmStatus},
};
use std::collections::VecDeque;

/// 虚拟机启动器
pub struct VmDed;

impl VmLauncher<VmRuntimeDed> for VmDed {
    fn launch(self) -> Result<VmRuntimeDed> {
        Ok(VmRuntimeDed::new())
    }
}

/// 虚拟机运行时
#[derive(Debug, Default)]
pub struct VmRuntimeDed {
    /// 演绎词项映射
    /// * 🎯以「邻接表」的形式呈现
    ded_graph: Graph<Term>,

    /// 临时输出缓存
    /// * 🚩在[`Self::fetch_output`]时，从中拿取输出
    output_cache: VecDeque<Output>,

    /// 任务缓冲区
    /// * 🎯记忆「所问之问题」
    /// * 🎯支持`CYC`指令
    /// * 🚩支持基本的「推理周期」概念
    task_buffer: VecDeque<Task>,
}

impl VmRuntimeDed {
    /// 构造函数
    /// * 🚩使用[`Self::default`]构造一个空对象
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加「NAVM输出」
    pub fn add_output(&mut self, output: Output) {
        self.output_cache.push_back(output);
    }

    /// 枚举Narsese→词法Narsese
    /// * 🚩使用双方ASCII转译器实现互转
    /// * 🚩同时保留ASCII转译后的字符串
    pub fn enum_to_lexical<'a>(
        from: &impl FormatTo<&'a NarseseFormat<&'a str>, String>,
    ) -> (String, LexicalNarsese) {
        let narsese_str = FORMAT_ASCII.format(from);
        let narsese: LexicalNarsese = FORMAT_ASCII_LEXICAL
            .parse(&narsese_str)
            .expect("Narsese转换失败");
        (narsese_str, narsese)
    }

    /// 【IL-1】新增一个词项「演绎链接」
    /// * 🚩先添加链接，再传递性更新
    ///   * 更新时生成「OUT」输出
    pub fn add_ded_link(&mut self, from: &Term, to: &Term) {
        // 直接添加
        self.ded_graph.add_link_cloned(from, to);
        // * 📌传递性更新
        self.transitive_update(from, to);
        self.add_output(Output::COMMENT {
            content: format!("🕸️更新后网络：{:?}", self.ded_graph),
        })
    }

    /// 传递性更新
    /// * 🚩更新时产生[`Output::OUT`]输出
    /// * 🎯展示IL-1「演绎」的规则
    /// TODO: 性能有待优化
    fn transitive_update(&mut self, _from: &Term, _to: &Term) {
        loop {
            let to_add = list! [
                {
                    // * 🚩复制，以免借用冲突 | 边遍历边修改
                    (item_from.clone(), item_to.clone())
                }
                for item_mid in (self.ded_graph.items())
                for item_from in (self.ded_graph.items_to(item_mid).unwrap())
                for item_to in (self.ded_graph.items_from(item_mid).unwrap())
                // 限制必须是新结论
                if (!self.ded_graph.has_link(item_from, item_to))
            ];
            // * 🚩没啥可增加⇒退出（应对「长距离推理」的场景）
            if to_add.is_empty() {
                break;
            }
            for (i_from, i_to) in to_add {
                // 添加连接
                self.ded_graph.add_link_cloned(&i_from, &i_to);
                // 添加输出
                let term = Term::new_inheritance(i_from, i_to);
                let judgement = Judgement(term, Truth::Empty, Stamp::Eternal);
                let (narsese_str, narsese) = Self::enum_to_lexical(&judgement);
                self.add_output(Output::OUT {
                    content_raw: narsese_str,
                    narsese: Some(narsese),
                })
            }
        }
    }

    /// 置入一条Narsese任务
    pub fn input_task(&mut self, task: Task) -> Result<()> {
        // 加入缓冲区（末尾）
        self.task_buffer.push_back(task);
        // 返回
        Ok(())
    }

    /// 一个「推理周期」
    /// * 🎯对应`CYC`指令
    /// * 🎯模拟NARS中「推理循环」的概念
    pub fn cycle(&mut self, steps: usize) {
        for _ in 0..steps {
            // 从序列中取出一个任务，并处理
            match self.task_buffer.pop_front() {
                // 若有⇒处理
                Some(task) => {
                    // 若未消耗⇒添加到末尾
                    if let Some(task) = self.process_task(task) {
                        self.task_buffer.push_back(task)
                    }
                }
                // 若无⇒退出（等待下一次循环调用）
                None => break,
            }
        }
    }

    /// 持续推理周期，将自身元素全部推理
    pub fn cycle_all(&mut self) {
        self.cycle(self.task_buffer.len())
    }

    /// 处理一个任务
    /// * 🚩从队列中取出一个任务，控制其「是否消耗」
    ///   * ✨可放回（未处理），可不放回（被消耗）
    /// * 🚩返回一个[`Option`]，指示「被消耗」还是「未处理」
    fn process_task(&mut self, task: Task) -> Option<Task> {
        // 指示状态
        self.add_output(Output::COMMENT {
            content: format!("🏗️处理任务：{}", FORMAT_ASCII.format(&task)),
        });
        // 任务标点
        let punctuation = task.get_punctuation();
        // 任务词项
        let term = task.get_term();
        // 分派处理
        use narsese::enum_narsese::Punctuation::*;
        let is_consumed = match punctuation {
            Judgement => self.process_judgement(term),
            Goal => self.process_goal(term),
            Question => self.process_question(term),
            Quest => self.process_quest(term),
        };
        // 返回
        match is_consumed {
            true => None,
            false => Some(task),
        }
    }

    /// 处理Narsese判断
    /// * 🚩继承⇒直接作为连接处理
    /// * 🚩返回一个[`bool`]，指示「被消耗」还是「未处理」
    #[inline]
    fn process_judgement(&mut self, term: &Term) -> bool {
        match term {
            Term::Inheritance(subject, predicate) => {
                // 区分主词谓词
                let subject = &*subject.clone();
                let predicate = &*predicate.clone();
                // 添加连接
                self.add_ded_link(subject, predicate);
                // 被消耗
                true
            }
            _ => {
                // 用「NAVM输出」替代`println`
                self.add_output(Output::ERROR {
                    description: format!("❌尚未支持的词项类型：{term:?}"),
                });
                // 消耗掉
                true
            }
        }
    }

    /// 处理Narsese目标
    /// * 🚩返回一个[`bool`]，指示「被消耗」还是「未处理」
    #[inline]
    fn process_goal(&mut self, term: &Term) -> bool {
        match term {
            Term::Inheritance(subject, predicate) => {
                // 区分主词谓词
                let subject = &*subject.clone();
                let predicate = &*predicate.clone();
                // * 🚩若有连接⇒立即回答
                if self.ded_graph.has_link(subject, predicate) {
                    // 转换为词法Narsese
                    let judgement = Judgement(term.clone(), Truth::Empty, Stamp::Eternal);
                    let (narsese_str, narsese) = Self::enum_to_lexical(&judgement);
                    // 生成「完成」
                    self.add_output(Output::ACHIEVED {
                        content_raw: narsese_str,
                        narsese: Some(narsese),
                    });
                    // 被消耗
                    return true;
                };
            }
            _ => {
                // 用「NAVM输出」替代`println`
                self.add_output(Output::ERROR {
                    description: format!("❌尚未支持的词项类型：{term:?}"),
                });
                // 消耗掉
                return true;
            }
        }
        // 未消耗
        false
    }

    /// 处理Narsese问题
    /// * 🚩返回一个[`bool`]，指示「被消耗」还是「未处理」
    #[inline]
    fn process_question(&mut self, term: &Term) -> bool {
        match term {
            Term::Inheritance(subject, predicate) => {
                // 区分主词谓词
                let subject = &*subject.clone();
                let predicate = &*predicate.clone();
                // * 🚩若有连接⇒立即回答
                if self.ded_graph.has_link(subject, predicate) {
                    // 转换为词法Narsese
                    let judgement = Judgement(term.clone(), Truth::Empty, Stamp::Eternal);
                    let (narsese_str, narsese) = Self::enum_to_lexical(&judgement);
                    // 生成回答
                    self.add_output(Output::ANSWER {
                        content_raw: narsese_str,
                        narsese: Some(narsese),
                    });
                    // 被消耗
                    return true;
                };
            }
            _ => {
                // 用「NAVM输出」替代`println`
                self.add_output(Output::ERROR {
                    description: format!("❌尚未支持的词项类型：{term:?}"),
                });
                // 消耗掉
                return true;
            }
        }
        // 未消耗
        false
    }

    /// 处理Narsese请求
    /// * 🚩目前当「问题」处理
    /// * 🚩返回一个[`bool`]，指示「被消耗」还是「未处理」
    #[inline]
    fn process_quest(&mut self, term: &Term) -> bool {
        self.process_question(term)
    }
}

/// 实现「NAVM运行时」
impl VmRuntime for VmRuntimeDed {
    fn input_cmd(&mut self, cmd: Cmd) -> Result<()> {
        use Cmd::*;
        match cmd {
            // Narsese⇒输入判断、目标、问题、请求
            NSE(task) => {
                // 词法任务⇒枚举任务
                let task = task
                    .try_fold_into(&FORMAT_ASCII)
                    .map_err(|e| anyhow!("{e:?}"))?;
                self.input_task(task)?;
            }
            // 推理周期
            CYC(steps) => self.cycle(steps),
            // 信息展示
            INF { .. } => self.add_output(Output::INFO {
                message: format!("运行时信息：{self:?}"),
            }),
            // 注释 ⇒ 忽略
            REM { .. } => (),
            // 其它
            _ => self.add_output(Output::ERROR {
                description: format!("❌尚未支持的NAVM指令类型：{cmd}"),
            }),
        }
        Ok(())
    }

    fn fetch_output(&mut self) -> Result<Output> {
        // * 缓存为空时报错
        self.output_cache
            .pop_front()
            .ok_or(anyhow::anyhow!("缓存已空"))
    }

    fn try_fetch_output(&mut self) -> Result<Option<Output>> {
        // * 缓存为空时返回空
        Ok(self.output_cache.pop_front())
    }

    fn status(&self) -> &VmStatus {
        // 始终「运行中」
        &VmStatus::Running
    }

    fn terminate(&mut self) -> Result<()> {
        // 没有任何内容
        Ok(())
    }
}

/// 单元测试
///
/// !  📌【2024-04-09 19:55:08】REPL见`main.rs`
#[cfg(test)]
mod tests {
    use super::*;
}
