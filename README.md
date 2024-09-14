# NAVM - Non-Axiomatic Virtual Machine | 非公理虚拟机

|**简体中文** | [English](README.en.md)|
|:-:|:-:|

<!-- 徽章安排参考：https://daily.dev/blog/readme-badges-github-best-practices#organizing-badges-in-your-readme -->

![License](https://img.shields.io/crates/l/navm?style=for-the-badge&color=ff7043)
![Code Size](https://img.shields.io/github/languages/code-size/ARCJ137442/NAVM.rs?style=for-the-badge&color=ff7043)
![Lines of Code](https://www.aschey.tech/tokei/github.com/ARCJ137442/NAVM.rs?style=for-the-badge&color=ff7043)
[![Language](https://img.shields.io/badge/language-Rust-orange?style=for-the-badge&color=ff7043)](https://www.rust-lang.org)

<!-- 面向用户 -->

Cargo状态：

[![crates.io](https://img.shields.io/crates/v/navm?style=for-the-badge)](https://crates.io/crates/navm)
[![docs.rs](https://img.shields.io/docsrs/navm?style=for-the-badge)](https://docs.rs/navm)
![Crate Size](https://img.shields.io/crates/size/navm?style=for-the-badge)

![Recent Downloads](https://img.shields.io/crates/dr/navm?style=for-the-badge)
![Downloads](https://img.shields.io/crates/d/navm?style=for-the-badge)
![Crates.io Dependents](https://img.shields.io/crates/dependents/navm?style=for-the-badge)

<!-- 面向开发者 -->

开发状态：

[![CI status](https://img.shields.io/github/actions/workflow/status/ARCJ137442/NAVM.rs/ci.yml?style=for-the-badge)](https://github.com/ARCJ137442/NAVM.rs/actions/workflows/ci.yml)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-2.0.0-%23FE5196?style=for-the-badge)](https://conventionalcommits.org)
![GitHub commits since latest release](https://img.shields.io/github/commits-since/ARCJ137442/NAVM.rs/latest?style=for-the-badge)

![Created At](https://img.shields.io/github/created-at/ARCJ137442/NAVM.rs?style=for-the-badge)
![Last Commit](https://img.shields.io/github/last-commit/ARCJ137442/NAVM.rs?style=for-the-badge)

## 简介

一个对NARS进行**统一输入输出建模**的**指令机架构库**

- 承继于[NAVM.jl](https://github.com/ARCJ137442/NAVM.jl)
- 对NARS（非公理推理系统）进行统一抽象

（注：该库只是一个**抽象模型**与API，对于其在具体实现中的应用，请参考[BabelNAR.rs](https://github.com/ARCJ137442/BabelNAR.rs)）

## 概念

NAVM对各版本NARS系统的抽象

更多有关NAVM概念的介绍，详见[📝《概念》](./docs/zh-cn/concepts/doc.md)

<!-- ## 安装 -->

<!-- * 📌【2024-04-10 10:19:40】有关具体环节，在crates.io中已经完善 -->

## 使用

### 输入输出

NAVM.rs提供两个重要的**输入输出**数据类型：

- [NAVM**指令**](./docs/zh-cn/concepts/navm_cmd.md)：用于统一表示对CIN的**输入**
- [NAVM**输出**](./docs/zh-cn/concepts/navm_output.md)：用于统一表示CIN的**输出**

任何能输入输出与NARS有关的程序，通过「`Cmd`→程序专用输入」与「程序专用输出`→Output`」，即可被抽象为NAVM。

这两种数据类型都提供至少一种统一的数据转换方式：

- NAVM指令：具有**简单易读的类汇编语法**，可与字符串 `String` 相互转换
  - 详见[NAVM指令/语法](./docs/zh-cn/concepts/navm_cmd.md/#语法)
- NAVM输出：可被转换为**JSON对象**，并且亦可从JSON对象中解析
  - 详见[NAVM输出/JSON格式](./docs/zh-cn/concepts/navm_output.md/#JSON格式)

## 参见

- Julia前身：[NAVM.jl](https://github.com/ARCJ137442/NAVM.jl)
- Narsese支持：[Narsese.rs](https://github.com/ARCJ137442/Narsese.rs)
