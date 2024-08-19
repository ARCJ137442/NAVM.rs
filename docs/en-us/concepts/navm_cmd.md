# NAVM Command `Cmd`

[🔙Concept](./doc.md)

[📃Source Code](./../../../src/cmd/mod.rs)

- 🎯Used to represent common input methods for CIN
  - 📄Such as "Input Narsese", "Reasoner Step", etc.
- 🎯Facilitates the establishment of a "front-end and back-end" parsing and execution architecture
  - 📌Can **unify the input forms of various CIN implementations**
    - Front-end: Handles various input data (e.g., terminal, scripts) and translates them into intermediate language (NAVM commands)
    - Back-end: Processes intermediate language objects and translates them into corresponding CIN commands

## Categories and Semantics

📍Last Updated: 【2024-04-10 15:12:04】

### Overall Principles of Classification

NAVM commands have various types, and their design follows these principles:

- 📌**Universality**: Commonly exist in various major CIN versions (in some form of semantic commonality)
- 📌**Completeness**: Can correspond to all types of inputs of the original CIN without loss (generally used for existing CINs that have been constructed)
- 📌**Usability**: Can represent the involved Narsese, loop stepping in a [unified format](./common_narsese.md)

⚠️Implementation Completeness: NAVM commands currently only have some complete and determined uses

- The most commonly used commands currently
  - NSE Input Narsese
  - CYC Reasoning cycle step
  - VOL Set output volume
  - REM Comment
- Other commands are awaiting the establishment of usage standards that can span across CINs

### Specific Categories

Based on the above [principles](#Overall Principles of Classification), NAVM commands are classified as follows:

(Use "✏️" to represent field names, "📄" to represent CIN input use cases)

#### **`SAV`**

`Save` | Save the current data to a file

- ✏️Target: String | The object to be saved (memory area, experience history, buffer, etc.)
- ✏️Path: String | The path where the object will be saved (file path, etc.)

⚠️Standard not yet formed

#### **`LOA`**

`Load` | Load data from a file

- ✏️Target: String | The object to be loaded into (memory area, experience history, etc.)
- ✏️Path: String | The path from which the object will be loaded (file path, etc.)

⚠️Standard not yet formed

#### **`RES`**

`Reset` | Reset CIN data

- ✏️Target: String | The object to be reset (memory area, experience history, etc.)

- Reset path

⚠️Standard not yet formed

#### **`NSE`**

`Narsese` | Input Narsese statement

- ✏️Lexical Narsese Task | The Narsese task to be input into CIN (use an empty budget for "statement" to simulate)

#### **`NEW`**

`New` | Create a new reasoner

- ✏️Target: String | Reasoner name

📄Migrated from PyNARS: The `/new` command in ConsolePlus

⚠️Standard not yet formed

#### **`DEL`**

`Delete` | Delete (stop) the reasoner

- ✏️Target: String | Reasoner name

📄Migrated from PyNARS: The `/delete` command in ConsolePlus

⚠️Standard not yet formed

#### **`CYC`**

`Cycle` | Control the reasoner step

- ✏️Positive Integer | The reasoning cycle step length of CIN stepping
- 📄OpenNARS: `5`
- 📄OpenJunars: `:c 5`

#### **`VOL`**

`Volume` | Control the output volume of CIN to shield

- ✏️Positive Integer | Output volume, usually 0~100, 0=mute (only answers, operations, etc.), 100=maximum volume (allows all minor conclusions to be displayed)
- 📄OpenNARS: `*volume=0`
- 📄ONA: `*volume=0`

#### **`REG`**

`Register` | Register an operator (NAL-8) with CIN to customize the operation mechanism of CIN

- 📄ONA: `*setopname 1 ^left`
- 📄PyNARS: `/register left`

- ✏️Name: String | The name of the operator to be registered

#### **`INF`**

`Information` | Have CIN output a certain type of information

- ✏️Source: String | The source from which to obtain information (reasoner, memory area, buffer, etc.)

⚠️Standard not yet formed

#### **`HLP`**

`Help` | Print (the CIN's) help document

- ✏️Name: String | The entry name to query

⚠️Standard not yet formed

#### **`REM`**

`Remark` | Comment, generally not executed by CIN; even if executed, it will not affect the reasoning process

- ✏️Comment: String | The content contained in the comment
- 📄OpenNARS: `'the detective claims that tim lives in graz`

#### **`Custom`**

`Custom` | Custom, can be used to interface some "magic inputs" that are particularly present in CIN

- ✏️Command Head: String | The command head of the custom command (all uppercase)
- ✏️Command Content: String | The content of the custom command

⚠️No standard, generally only supported by specific CIN (transcompilers)

## Basic Syntax

📍Last Updated: 【2024-04-10 15:11:54】

The NAVM command uses a syntax similar to assembly language `【Command Head】 【Command Content】…`

- 🎯Aim to balance readability and performance

The syntax for all types of commands:

|Command Type|Syntax|
|:-:|:--|
|SAV|`SAV <Target> [Path]`|
|LOA|`LOA <Target> [Path]`|
|RES|`RES [Target]`|
|NSE|`NSE <CommonNarsese Sentence/Task>`|
|NEW|`NEW <Name>`|
|DEL|`DEL <Name>`|
|CYC|`CYC <Number of Steps>`|
|VOL|`VOL <Volume>`|
|REG|`REG <Operator Name>`|
|INF|`INF <Source>`|
|HLP|`HLP [Name]`|
|REM|`REM [Single-line Comment]`|
|Custom|`<Any Command Head Outside the Table> [Any Single-line Content]`|
