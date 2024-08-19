# NAVM - Non-Axiomatic Virtual Machine

[简体中文](./README.md) | **English**

    🏗️The **English document** of the project is still under preparation. PR is welcome.

[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-2.0.0-%23FE5196?logo=conventionalcommits&logoColor=white)](https://conventionalcommits.org)

This project uses [Semantic Versioning 2.0.0](https://semver.org/) for version number management.

A **library of instruction machine architecture** for unified input and output modeling of NARS.

- Derived from [NAVM.jl](https://github.com/ARCJ137442/NAVM.jl)
- Provides a unified abstraction for the NARS (Non-Axiomatic Reasoning System)

(Note: This library is only an **abstract model** and API. For its application in specific implementations, please refer to [BabelNAR.rs](https://github.com/ARCJ137442/BabelNAR.rs))

## Introduction to Concepts

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
