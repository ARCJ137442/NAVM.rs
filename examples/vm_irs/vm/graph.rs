//! 使用映射[`Map`]简单存储表示图结构
#![allow(unused)]

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

/// * 定义其中「集合」类型的实现
pub type Set<T> = HashSet<T>;

/// * 定义其中「映射」类型的实现
pub type Map<K, V> = HashMap<K, V>;

/// 简单表示「图」结构
/// * 🚩使用两个[`Map`]，实现双向键值查询
#[derive(Debug)]
pub struct Graph<T> {
    inner: Map<T, Set<T>>,
    inner_reverse: Map<T, Set<T>>,
}

impl<T> Default for Graph<T> {
    /// ! ⚠️不能直接派生：[`Map`]可以直接[`Default::default`]，但`#[derive(..)]`要`T`实现[`Default`]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph {
            inner: Map::new(),
            inner_reverse: Map::new(),
        }
    }
}

/// 直接实现「图」
impl<T: Eq + Hash> Graph<T> {
    /// 判断是否包含某元素
    /// * 🚩仅需判断「正向映射」
    pub fn contains(&self, item: &T) -> bool {
        self.inner.contains_key(item)
    }

    /// 判断图是否有连接
    /// * 🚩只需判断「正向映射」
    /// * 📌只要有「正向映射」就一定有「反向映射」
    pub fn has_link(&self, from: &T, to: &T) -> bool {
        self.inner.get(from).is_some_and(|set| set.contains(to))
    }

    // ! ❌`add`方法需要同时向两个[`Map`]传入所有权

    /// 删除节点
    /// * 🚩仅在存在键时插入值
    /// * 🚩同时更新正向映射、反向映射
    /// * 📝映射使用[`Map::entry`]与[`Map::or_insert_with`]进行「条件插入」
    pub fn remove(&mut self, item: &T) {
        if self.contains(item) {
            // * 🚩在反向映射的「自身所指向的」中移除自身
            if let Some(items_item_map_to) = self.inner.get(item) {
                for item_map_to in items_item_map_to {
                    assert!(
                        self.inner_reverse
                            .get_mut(item_map_to)
                            .unwrap()
                            .remove(item),
                        "不可抵达：应该是同步删除的"
                    );
                }
            }
            // * 🚩在正向映射的「指向自身的」中移除自身
            if let Some(items_map_to_item) = self.inner_reverse.get(item) {
                for item_map_from in items_map_to_item {
                    assert!(
                        self.inner.get_mut(item_map_from).unwrap().remove(item),
                        "不可抵达：应该是同步删除的"
                    );
                }
            }
            // 正向映射
            self.inner.remove(item);
            // 反向映射
            self.inner_reverse.remove(item);
        }
    }

    /// 删除节点连接
    pub fn remove_link(&mut self, from: &T, to: &T) {
        if self.has_link(from, to) {
            // 正向
            self.inner.get_mut(from).unwrap().remove(to);
            // 反向
            self.inner_reverse.get_mut(to).unwrap().remove(from);
        }
    }

    /// 遍历所有节点
    /// * 📌仅需遍历「正向映射」
    pub fn items(&self) -> impl Iterator<Item = &T> {
        self.inner.keys()
    }
}

/// 对实现了[`Cloned`]的`T`实现「图」
/// * 🎯通过假定`T: Clone`，简化背后逻辑
///   * 并且允许后续继续扩展
impl<T: Eq + Hash + Clone> Graph<T> {
    /// 添加新节点
    /// * 🚩仅在不存在键时插入值
    /// * 🚩正向映射、反向映射同时插入
    /// * 📝映射使用[`Map::entry`]与[`Map::or_insert_with`]进行「条件插入」
    pub fn add_cloned(&mut self, item: T) {
        self.inner.entry(item.clone()).or_insert_with(|| Set::new());
        self.inner_reverse.entry(item).or_insert_with(|| Set::new());
    }

    /// 向图添加连接
    /// * 📌会拷贝值，但无需考虑「所有权交接」问题
    /// * 🚩在没有`from`的值时，自动拷贝并创建一个空集合
    pub fn add_link_cloned(&mut self, from: &T, to: &T)
    where
        T: Clone,
    {
        match (self.inner.get_mut(from), self.inner_reverse.get_mut(to)) {
            // 已有⇒双方直接插入
            (Some(set), Some(set_rev)) => {
                set.insert(to.clone());
                set_rev.insert(from.clone());
            }
            // 没有⇒各自先插入值本身，再尝试插入
            _ => {
                self.add_cloned(from.clone());
                self.add_cloned(to.clone());
                self.inner.get_mut(from).unwrap().insert(to.clone());
                self.inner_reverse.get_mut(to).unwrap().insert(from.clone());
            }
        }
    }

    /// 获取其中一个节点所连接的节点
    /// * (x if item => x)
    pub fn items_from(&self, item: &T) -> Option<impl Iterator<Item = &T>> {
        self.inner.get(item).map(Set::iter)
    }

    /// 获取连接了其中一个节点的节点
    /// * (x if x => item)
    /// * 📌采用「正反向双映射」的真正缘起
    pub fn items_to(&self, item: &T) -> Option<impl Iterator<Item = &T>> {
        self.inner_reverse.get(item).map(Set::iter)
    }

    /// 遍历图所有的正向连接
    pub fn links(&'_ self) -> impl Iterator<Item = (&T, &Set<T>)> {
        self.inner.iter()
    }

    /// 遍历图所有的反向连接
    /// * 📌采用「正反向双映射」的真正缘起
    pub fn links_reverse(&'_ self) -> impl Iterator<Item = (&T, &Set<T>)> {
        self.inner_reverse.iter()
    }
}

/// 快捷构造宏
#[macro_export]
macro_rules! graph_cloned {
    {$(
        $k:expr => $v:expr $(,)?
    )*} => {
        {
            let mut g = Graph::new();
            $(
                g.add_link_cloned(&$k, &$v);
            )*
            g
        }
    };
}

/// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use nar_dev_utils::asserts;

    /// 断言图中的连接
    macro_rules! assert_links {
        {
            $g:expr;
            $(
                $from:expr => [$($to:expr $(,)?)*];
            )*
        } => {
            $(
                // 先断言单独连接
                $(
                    assert!($g.has_link(&$from, &$to));
                )*
                // 再断言迭代器
                let s = $g.items_from(&$from).into_iter().flatten().cloned().collect::<Set<_>>();
                let expected = Set::from([$($to),*]);
                assert_eq!(s, expected);
            )*
        };
        {
            $g:expr;
            $(
                [$($from:expr $(,)?)*] => $to:expr;
            )*
        } => {
            $(
                // 先断言单独连接
                $(
                    assert!($g.has_link(&$from, &$to));
                )*
                // 再断言迭代器
                let s = $g.items_to(&$to).into_iter().flatten().cloned().collect::<Set<_>>();
                let expected = Set::from([$($from),*]);
                assert_eq!(s, expected);
            )*
        };
    }

    /// 测试/构造&新增
    /// * 🎯构造图并添加连接
    #[test]
    fn test_add() {
        let g: Graph<usize> = graph_cloned! {
            1 => 2
            1 => 3
        };
        dbg!(&g);
        // 正向连接
        assert_links! {
            g;
            1 => [2, 3];
            2 => [];
            3 => [];
        }
        // 反向连接
        assert_links! {
            g;
            [] => 1;
            [1] => 2;
            [1] => 3;
        }
    }

    /// 测试/构造&新增&删除
    /// * 🎯构造图后删除连接
    #[test]
    fn test_remove() {
        let mut g: Graph<usize> = graph_cloned! {
            1 => 2
            1 => 3
            2 => 3
        };
        dbg!(&g);
        // 正向连接
        assert_links! {
            g;
            1 => [2, 3];
            2 => [3];
            3 => [];
        }
        // 反向连接
        assert_links! {
            g;
            [] => 1;
            [1] => 2;
            [1, 2] => 3;
        }
        // * 📌删除连接
        g.remove_link(&1, &2);
        assert_links! {
            g;
            1 => [3];
            2 => [3];
            3 => [];
        }
        assert_links! {
            g;
            [] => 1;
            [] => 2;
            [1, 2] => 3;
        }
        // * 📌删除节点
        g.remove(&2);
        assert_links! {
            g;
            1 => [3];
            3 => [];
        }
        assert_links! {
            g;
            [] => 1;
            [1] => 3;
        }
    }
}
