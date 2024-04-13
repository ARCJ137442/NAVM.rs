# CIN

[🔙概念](./doc.md)

📍最后更新：【2024-04-10 15:38:34】

📄全名：Computer Implement of NARS（NARS计算机实现）

🎯对各版本「NARS计算机实现」的总称

- 可指代所有**实现NARS**的计算机软件系统

## 📌对具体实现的要求

- 推理器：能启动并运行内部推理器
- 输入输出：能向「内部推理器」输入Narsese语句（不限形式），并输出「派生」「回答」「操作」等信息

该要求意味着：

- 不要求完整实现NAL 1~9
  - CIN无需完整实现从NAL-1到NAL-9的完整内容，只需让内容满足一定的输入输出格式（并且可被捕获与转换）
- 仅需在「内外输入输出」处提供统一接口
  - 如：在保证「统一外部接口」的情况下，一个CIN可以附带多个内部推理器/子推理器，这些推理器间可以形成层级、网络等多样关系

## 📄主要CIN举例

|具体实例|编程语言|NAL支持情况|开发情况|
|:--|:--|:--|:--|
|[OpenNARS (3.x)](https://github.com/opennars/opennars) |Java|NAL 1 ~ 9|稳定|
|[ONA](https://github.com/opennars/OpenNARS-for-Applications) |C|NAL 1 ~ 8|稳定|
|[PyNARS](https://github.com/bowen-xu/PyNARS) |Python|NAL 1 ~ 6|活跃|
|[NARS-Python](https://github.com/ccrock4t/NARS-Python) |Python|NAL 1 ~ 8|稳定|
|[OpenJunars](https://github.com/AIxer/OpenJunars) |Julia|NAL 1 ~ 6|稳定|
