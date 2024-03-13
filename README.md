# NAVM - Non-Axiomatic Virtual Machine | 非公理虚拟机

[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits&logoColor=white)](https://conventionalcommits.org)

该项目使用[语义化版本 2.0.0](https://semver.org/)进行版本号管理。

一个指令机架构库

- 基于[NAVM.jl](https://github.com/ARCJ137442/NAVM.jl)
- 对NARS（非公理推理系统）进行**统一指令机抽象**

## 概念

### CIN (Computer Implement of NARS)

- 「NARS计算机实现」之英文缩写
- 指代所有**实现NARS**的计算机软件系统
  - 不要求完整实现NAL 1~9

### ***CommonNarsese***

🔗参考[**JuNarsese.jl**的相关部分](https://github.com/ARCJ137442/JuNarsese.jl?tab=readme-ov-file#commonnarsese)

### 中间语 NAVM指令

NAVM使用一个统一的「中间语言」对CIN的输入进行抽象

- 🎯用以表示CIN常用的输入方式
  - 📄如「输入Narsese」「推理器步进」等
- 🎯便于架设一个「前后端」解析执行架构
  - 📌可**统一各CIN实现的输入形式**
    - 前端：处理各类输入（例如终端、脚本）数据，将其翻译成中间语(NAVM指令)
    - 后端：处理中间语对象，将其翻译成对应CIN命令

## 安装

TODO: 待库成熟时完善内容

## 使用

TODO: 待库成熟时完善内容

## 参见

- Julia前身：[NAVM.jl](https://github.com/ARCJ137442/NAVM.jl)
