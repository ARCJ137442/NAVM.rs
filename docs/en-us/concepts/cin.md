# CIN

[🔙 Concept](./doc.md)

📍 Last updated: 【2024-04-10 15:38:34】

📄 Full Name: Computer Implementation of NARS (NARS Computer Implementation)

🎯 A collective term for all versions of "NARS Computer Implementation"

- Can refer to any computer software system that **implements NARS**

## 📌 Requirements for Specific Implementations

- Reasoner: Capable of launching and running an internal reasoner
- Input and Output: Capable of inputting Narsese statements to the "internal reasoner" (in any form) and outputting "derived", "answers", "operations", and other information

This requirement implies:

- No requirement for a complete implementation of NAL 1~9
  - CIN does not need to fully implement the content from NAL-1 to NAL-9, only requiring the content to meet certain input and output formats (and can be captured and converted)
- Only a unified interface is required at the "internal and external input and output"
  - For example: While ensuring a "unified external interface", a CIN can have multiple internal reasoners/sub-reasoners, which can form hierarchical, network, and other diverse relationships

## 📄 Main Examples of CIN

| Specific Instance | Programming Language | NAL Support Status | Development Status |
|:--|:--|:--|:--|
| [OpenNARS (3.x)](https://github.com/opennars/opennars) | Java | NAL 1 to 9 | Stable |
| [ONA](https://github.com/opennars/OpenNARS-for-Applications) | C | NAL 1 to 8 | Stable |
| [PyNARS](https://github.com/bowen-xu/PyNARS) | Python | NAL 1 to 6 | Active |
| [NARS-Python](https://github.com/ccrock4t/NARS-Python) | Python | NAL 1 to 8 | Stable |
| [OpenJunars](https://github.com/AIxer/OpenJunars) | Julia | NAL 1 to 6 | Stable |
