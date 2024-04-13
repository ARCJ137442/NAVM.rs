# NAVM输出 `Output`

[🔙概念](./doc.md)

[📃源码](./../../../src/output/mod.rs)

📍最后更新：【2024-04-10 10:54:45】

- 🎯用以规范化CIN的输出
  - 📄如「派生」「回答」「错误」「操作」

## 类别和语义

📍最后更新：【2024-04-10 10:54:51】

### 分类的整体原则

NAVM输出具有多种不同的输出类型，其设计遵循以下原则：

- 📌**普遍**：在多种主要CIN版本中（以某种语义共通的形式）普遍存在
- 📌**完备**：能无损保留原CIN的各类输出（一般用于已构建的现有CIN）
- 📌**易用**：能以[统一格式](./common_narsese.md)方便地提取其中有关「所含Narsese」「所输出NARS操作」等NARS相关信息

### 具体类别

📍最后更新：【2024-04-10 11:00:04】

基于以上[原则](#分类的整体原则)，NAVM输出分类如下：

（用「✏️」表示字段名，「📄」表示CIN输出样例）

#### **`IN`**

CIN输入Narsese的回显

- ✏️原始内容：字符串
- ✏️所含Narsese：词法Narsese（可能没有）
- 📄ONA: `Input: <A --> B>. Priority=1.000000 Truth: frequency=1.000000, confidence=0.900000`

#### **`OUT`**

CIN输出一行Narsese，一般为导出结论、中间推理结果

- ✏️原始内容：字符串
- ✏️所含Narsese：词法Narsese（可能没有）
- 📄OpenNARS: `OUT: <A1 --> A2>. %1.00;0.90% {3404 : (-6241115378434429522,0)}`
- 📄ONA: `Derived: <A --> C>. Priority=0.407250 Truth: frequency=1.000000, confidence=0.810000`

#### **`ERROR`**

CIN内部运行时产生的错误，与上层接口无关

- ✏️描述：字符串
- 📄OpenNARS: `ERR: java.lang.ClassCastException: class org.opennars.language.Term cannot be cast to class org.opennars.language.CompoundTerm (org.opennars.language.Term and org.opennars.language.CompoundTerm are in unnamed module of loader 'app')`

#### **`ANSWER`**

CIN对输入的「问题」产生的「回答」，在NARS层面上对应「系统呈现已有经验」

- ✏️原始内容：字符串
- ✏️Narsese：词法Narsese（可能没有）
- 📄OpenNARS: `Answer: <B --> C>. %1.00;0.81% {11778 : (-5483911157924289284,1);(-5483911157924289284,0)}`

#### **`ACHIEVED`**

CIN对输入的「目标」产生的「回应」，在NARS层面上对应「系统（目前）完成已有目标」

- ✏️原始内容：字符串
- ✏️Narsese：词法Narsese（可能没有）
- 📄PyNARS: `ACHIEVED: A. :|: %1.000:0.900%`

#### **`EXE`**

CIN输出一个NAL-8意义上的「操作」，指示「对外部过程的调用」

- ✏️原始内容：字符串
- ✏️操作：NARS操作
  - ✏️操作符名：字符串 | 📄如`left`，不带语法上的尖号
  - ✏️操作参数：词法Narsese词项数组
- 📄OpenNARS: `EXE: $0.00;0.04;0.55$ ^left([{SELF}, (*,P1,P2)])=null`

#### **`INFO`**

CIN输出一条与Narsese无关的信息

- ✏️消息：字符串
- 📄PyNARS: `INFO  : Loading RuleMap <LUT.pkl>...`

#### **`COMMENT`**

CIN输出一条注释或日志信息，相比`INFO`更多但也更琐碎；常用于运行时debug，或执行[NAVM指令](./navm_cmd.md)中的`REM`指令时

- ✏️内容：字符串

#### **`TERMINATED`**

CIN终止运行，见于「CIN主动结束运行」的情况

- ✏️描述：字符串
- 📄ONA: `Parsing error: Punctuation has to be belief . goal ! or question ?` (换行) `Test failed.`

#### **`UNCLASSIFIED`**

拥有可被识别的「输出类型」，但不在标准「NAVM输出」范围内的输出

- ✏️类别：字符串 | 实际作为「类型」的标识
- ✏️内容：字符串 | 输出的原始内容
- ✏️Narsese：词法Narsese（可能没有） | 输出中可能包含的Narsese
- 📄OpenNARS: `ANTICIPATE: <{SELF} --> [satisfied]>` (ANTICIPATE)
- 📄ONA: `decision expectation=0.507988 implication: <(<a --> b> &/ <(* {SELF}) --> ^left>) =/> <{SELF} --> [good]>>. Truth: frequency=0.992043 confidence=0.237070 dt=12.000000 precondition: <a --> b>. :|: Truth: frequency=1.000000 confidence=0.900000 occurrenceTime=171` (ANTICIPATE)

#### **`OTHER`**

其它未被归类、不含重要Narsese、NARS操作信息的CIN输出

- ✏️内容：字符串
- 📄OpenNARS: `[l]: attaching Shell to Nar...`
- 📄OpenNARS: `Executed based on: $0.2680;0.0862;0.5836$ <(&/,<{#1} --> [seen]>,+10,<{#1} --> [seen]>,+62817,(^right,{SELF}),+19485) =/> <{SELF} --> [satisfied]>>. %0.75;0.33%`
- 📄PyNARS: `Setup: Changing random seed=137..`

另请参考源码中有关[`Output`](https://github.com/ARCJ137442/NAVM.rs/blob/main/src/output/structs.rs)的定义

## JSON格式

📍最后更新：【2024-04-10 12:28:34】

「NAVM输出」将[上述](#具体类别)所定义的「输出」类型提炼压缩到如下**四个属性**：

- ✏️类型 `type`：字符串 | 输出的[类别](#具体类别)
- ✏️内容 `content`：字符串 | 输出的原始内容
- ✏️Narsese `narsese`：字符串（可缺省） | 输出所含Narsese（遵循ASCII [CommonNarsese](./common_narsese.md)语法）
- ✏️操作 `operation`：字符串数组（可缺省） | 输出所含的「NARS操作」信息

表格：

|名称|字段名|字段类型|Rust类型|描述|
|:--|:--|:--|:--|:--|
|类型|`type`|字符串|`String`|输出的[类别](#具体类别)|
|内容|`content`|字符串数组|`Vec<String>`|输出的原始内容|
|Narsese|`narsese`|字符串（可缺省）|`Option<String>`|输出所含Narsese（遵循ASCII [CommonNarsese](./common_narsese.md)语法）|
|操作|`operation`|字符串数组（可缺省）|`Option<Vec<String>>`|输出所含的「NARS操作」信息（若有必非空）|

由此总结出如下TypeScript定义：

```typescript
export type NARSOutput = {
    /** 输出的类别（全大写） */
    type: string
    /** 输出的（原始）内容，可能会截去类别信息 */
    content: string
    /** 若输出包含被识别出的Narsese，则为相应的Narsese字符串 */
    narsese?: string
    /** 若输出包含被识别出的NARS操作，则为`[无尖号操作名, ...操作参数]`字符串数组 */
    operation?: [string, ...string[]]
}
```

另请参考源码中有关[`OutputJSON`](https://github.com/ARCJ137442/NAVM.rs/blob/main/src/output/conversion.rs)的定义
