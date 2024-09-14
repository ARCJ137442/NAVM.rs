# NAVM - Non-Axiomatic Virtual Machine

|[简体中文](README.md) | **English**|
|:-:|:-:|

![License](https://img.shields.io/crates/l/navm?style=for-the-badge&color=ff7043)
![Code Size](https://img.shields.io/github/languages/code-size/ARCJ137442/NAVM.rs?style=for-the-badge&color=ff7043)
![Lines of Code](https://www.aschey.tech/tokei/github.com/ARCJ137442/NAVM.rs?style=for-the-badge&color=ff7043)
[![Language](https://img.shields.io/badge/language-Rust-orange?style=for-the-badge&color=ff7043)](https://www.rust-lang.org)

<!-- Towards Users -->

Cargo Status:

[![crates.io](https://img.shields.io/crates/v/navm?style=for-the-badge)](https://crates.io/crates/navm)
[![docs.rs](https://img.shields.io/docsrs/navm?style=for-the-badge)](https://docs.rs/navm)
![Crate Size](https://img.shields.io/crates/size/navm?style=for-the-badge)

![Recent Downloads](https://img.shields.io/crates/dr/navm?style=for-the-badge)
![Downloads](https://img.shields.io/crates/d/navm?style=for-the-badge)
![Crates.io Dependents](https://img.shields.io/crates/dependents/navm?style=for-the-badge)

<!-- Towards Developers -->

Development Status:

[![CI status](https://img.shields.io/github/actions/workflow/status/ARCJ137442/NAVM.rs/ci.yml?style=for-the-badge)](https://github.com/ARCJ137442/NAVM.rs/actions/workflows/ci.yml)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-2.0.0-%23FE5196?style=for-the-badge)](https://conventionalcommits.org)
![GitHub commits since latest release](https://img.shields.io/github/commits-since/ARCJ137442/NAVM.rs/latest?style=for-the-badge)

![Created At](https://img.shields.io/github/created-at/ARCJ137442/NAVM.rs?style=for-the-badge)
![Last Commit](https://img.shields.io/github/last-commit/ARCJ137442/NAVM.rs?style=for-the-badge)

## Introduction

A **library of instruction machine architecture** for unified input and output modeling of NARS.

- Derived from [NAVM.jl](https://github.com/ARCJ137442/NAVM.jl)
- Provides a unified abstraction for the NARS (Non-Axiomatic Reasoning System)

(Note: This library is only an **abstract model** and API. For its application in specific implementations, please refer to [BabelNAR.rs](https://github.com/ARCJ137442/BabelNAR.rs))

## Concepts

Abstraction of various versions of NARS systems by NAVM

For more information on NAVM concepts, see [📝 "Concepts"](./docs/en-us/concepts/doc.md)

<!-- ## Installation -->

<!-- * 📌【2024-04-10 10:19:40】The specific steps have been completed in crates.io -->

## Usage

### Input and Output

NAVM.rs provides two important **input and output** data types:

- [NAVM **Command**](./docs/en-us/concepts/navm_cmd.md): Used to uniformly represent **input** to CIN
- [NAVM **Output**](./docs/en-us/concepts/navm_output.md): Used to uniformly represent the **output** of CIN

Any program that can input and output related to NARS can be abstracted as NAVM through 'Cmd → Program-specific input' and 'Program-specific output → Output'.

Both data types provide at least one unified method of data conversion:

- NAVM Command: Features **easy-to-read assembly-like syntax** that can be converted to and from a string `String`
  - See [NAVM Command/Syntax](./docs/en-us/concepts/navm_cmd.md/#Syntax)
- NAVM Output: Can be converted to a **JSON object** and can also be parsed from a JSON object
  - See [NAVM Output/JSON Format](./docs/en-us/concepts/navm_output.md/#JSON Format)

## See Also

- Julia predecessor: [NAVM.jl](https://github.com/ARCJ137442/NAVM.jl)
- Narsese support: [Narsese.rs](https://github.com/ARCJ137442/Narsese.rs)
