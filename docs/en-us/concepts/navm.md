# NAVM

[🔙Concept](./doc.md)

[📃Source Code](./../../../src/vm/mod.rs)

📍Last Updated: 【2024-04-13 15:15:27】

📄Full Name: Non-Axiomatic Virtual Machine

🎯The **middle layer abstraction** of NARS for engineering applications

- ✅Essentially covers the [computer implementation](./cin.md) of various versions of NARS
- ✨Unified input and output formats
  - Input: [NAVM Command](./navm_cmd.md) `Cmd`
  - Output: [NAVM Output](./navm_output.md) `Output`
- 📌Uses an API architecture of "Launcher → Runtime" to separate "launch configuration" from "runtime state"
  - 🔗From: [NAVM Launcher](#launcher)
  - 🔗To: [NAVM Runtime](#runtime)

## Launcher

[🔙NAVM](#navm)

[📃Source Code](./../../../src/vm/traits.rs)

- 🎯Records launch parameters
  - 📄In the launcher of CIN based on JVM, you can attach parameters related to JVM (such as OpenNARS)
  - 📄In the launcher of CIN based on "standalone executable files", there will be parameters related to the "path where the executable file is located" (such as ONA)
- 🎯And itself serves as the "launch configuration" to start the virtual machine
  - Through a unified `launch` method, it starts up to [NAVM Runtime](#runtime)

## Runtime

[🔙NAVM](#navm)

[📃Source Code](./../../../src/vm/traits.rs)

- 🎯Records and tracks the runtime state and input/output of the virtual machine
  - 📍"Virtual Machine State"
    - [📃Source Code](./../../../src/vm/structs.rs)
    - 🎯Indicates the current operation of the virtual machine: "running" or "terminated/stopped"
    - 📄There are currently two main states
      - Running
      - Terminated (with "termination result")
  - 📍"Virtual Machine Input and Output"
    - 🎯Through unified "NAVM Command" and "NAVM Output," **as much as possible** to hide the underlying operational differences of each [CIN](./cin.md)
    - ⚠️Can eliminate differences in "different Narsese dialects", but differences at the **NAL, control mechanism level are inevitable**
      - 📄ONA: Compound terms can have at most two sub-terms; only supports a limited number of registered operations
      - 📄PyNARS: As of 2024-04-13, it has not yet officially supported NAL 7~9
- 📌The input and output are **asynchronous**: the order of input and output cannot be fully determined
  - 🚩Input: By calling the "input command" method, an NAVM Command is placed into CIN
  - 🚩Output: By calling the "fetch output" or "try to fetch output" method, an NAVM Output is fetched from the CIN's cached output
  - ⚠️After inputting an NAVM Command, the output will not be immediately returned (as a function return value, etc.)
- ✨Can be terminated by its creator at any time
  - ⚠️After the virtual machine is terminated, input and output will no longer be processed
