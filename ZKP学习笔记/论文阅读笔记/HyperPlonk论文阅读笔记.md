# HyperPlonk论文阅读笔记

这篇论文主要介绍了 **HyperPlonk**，这是一种对流行的 zk-SNARK 证明系统 Plonk 的改进方案。HyperPlonk 的核心创新在于将 Plonk 移植到了**布尔超立方体（Boolean Hypercube）**上，利用多线性多项式（multilinear polynomials）代替了原本的单变量多项式 1111。



以下是这篇论文的核心内容和技术亮点：

### 1. 核心痛点与解决方案

- 

  **传统 Plonk 的瓶颈：** 传统的 Plonk 系统使用单变量多项式，在生成证明时依赖于快速傅里叶变换（FFT）。对于大规模电路，FFT 是主要的性能瓶颈，导致证明生成时间为 $O(N \log N)$ 2222。此外，在传统 Plonk 中使用高阶（High-degree）自定义门会导致 FFT 规模显著增加，从而限制了其灵活性 。

- **HyperPlonk 的方案：** HyperPlonk 使用多线性多项式承诺，并在布尔超立方体 $\{0,1\}^\mu$ 上通过 **SumCheck 协议**来执行校验 。

- **主要优势：**

  - **移除 FFT：** 通过使用 SumCheck 协议，HyperPlonk 完全消除了证明生成过程中的 FFT 操作，将证明者的运行时间降低到**线性时间 $O(N)$** 。
  - **高效支持高阶自定义门：** 在 HyperPlonk 中，证明者的工作量主要取决于门的乘法数量，而不是总度数。这使得它能够极低成本地支持非常高阶的自定义门（例如度数为 32 的门），从而大幅减少电路所需的门数量 。

### 2. 技术架构与组件

HyperPlonk 将 Plonk 的各个组件模块化，并在超立方体上重新实现了它们 ：



- **Gate Identity (门恒等式)：** 通过 **ZeroCheck** 协议来证明，该协议基于 SumCheck，无需 FFT 。

- **Wiring Identity (连线恒等式)：** 通过改进的 **Permutation Check（置换校验）** 来实现。论文提出了一种在超立方体上进行集合相等性检查的协议 。

- **HyperPlonk+ (支持查找表)：** 传统的 Plookup 依赖于循环群的旋转特性，这在超立方体上不直接存在。作者设计了一种新的技术，通过构造一个遍历超立方体的“next”函数，实现了在超立方体上的查找表（Lookup）功能，称为 **HyperPlonk+** 。

  

### 3. 多线性多项式承诺 (Multilinear PCS) 的改进

由于 HyperPlonk 依赖于多线性多项式承诺，论文回顾并改进了两种现有的方案 ：

- **Orion+：** 论文改进了 Orion 方案，提出了 Orion+。它保持了线性时间的证明者速度，但将证明大小从 Orion 的 5.5MB 减少到了 **<10KB**（缩小了近 1000 倍），且验证时间也是线性的。

- **FRI-based PCS：** 论文展示了如何将基于 FRI 的单变量承诺转换为多线性承诺，提供了一种抗量子计算的线性时间证明生成方案 。

  

### 4. 性能评估

- **证明生成速度：** 在单线程和多线程环境下，HyperPlonk 的证明生成速度均优于现有的先进系统（如 Jellyfish Plonk 和 Spartan）。特别是对于大于 $2^{14}$ 规模的电路，优势更加明显 。
- **并行化：** 由于移除了 FFT，HyperPlonk 具有更好的并行化潜力。测试显示，随着线程数增加，性能提升几乎是线性的。
- **证明大小：** 使用 KZG 承诺方案时，HyperPlonk 的证明大小非常小（例如 $\mu=25$ 时约为 5.5KB）。

### 总结

HyperPlonk 是一项针对 zk-SNARK 证明生成效率的重要优化工作。通过转向布尔超立方体和 SumCheck 协议，它成功移除了昂贵的 FFT 操作，实现了线性时间的证明生成，并为使用更复杂、表达能力更强的高阶自定义门铺平了道路。



这篇论文的第四章 **"HyperPlonk: Plonk on the boolean hypercube"** 是全篇的核心章节。在这一章中，作者将第三章介绍的各种工具（SumCheck, ZeroCheck, Permutation Check）组合起来，正式定义了 HyperPlonk 协议。

这一章主要讲述了两个核心内容：**约束系统的定义**（我们要证明什么）和 **PolyIOP 协议的流程**（我们如何证明它）。

以下是详细解读：

### 1\. 定义 HyperPlonk 约束系统 ( $R_{PLONK}$ )

这一节首先定义了 HyperPlonk 要处理的问题模型，即“索引关系”（Indexed Relation）。它将传统的 Plonk 约束移植到了**布尔超立方体  $B_{\mu }$ ** 上 。

一个合法的 HyperPlonk 证明必须同时满足以下三个核心条件（恒等式）：

*   **门恒等式 (The Gate Identity):**
    *   这是用来检查电路逻辑（加法、乘法或自定义门）是否正确的规则。
    *   它定义了一个虚拟多项式  $\tilde{f}$ ，该多项式由选择器（Selectors）和见证（Witnesses）组成 。
    *   必须满足的条件是：这个多项式在超立方体上的每一个点都等于 0。这通过调用 **ZeroCheck** 关系来实现 。
    *   _优势_：这种定义方式允许支持**高阶自定义门**，而不像传统 Plonk 那样受限于 FFT 的性能瓶颈 。
*   **连线恒等式 (The Wiring Identity):**
    *   这是用来检查电路中的连线是否正确（即数据是否正确地从一个门的输出流向另一个门的输入）。
    *   它通过一个置换  $\sigma$  来捕捉连线约束，必须满足 **Permutation Check（置换校验）** 关系 。
*   **公共输入一致性 (Public Input Consistency):**
    *   这是用来确保电路使用的公共输入  $p$  与内部计算使用的 Witness  $w$  是匹配的。
    *   它要求公共输入多项式  $p$  与 Witness 多项式  $w$  在特定区域是一致的 。

此外，这一节还特别提到 HyperPlonk 可以很好地模拟\*\*状态机（State Machines）\*\*的计算 。

### 2\. The PolyIOP Protocol (协议流程)

在定义了约束系统后，4.2 节详细描述了证明者（P）和验证者（V）之间的交互流程。这个协议完全移除了昂贵的 FFT 操作 。

流程图（Figure 2）概括如下：

1.  **发送 Witness:** 证明者首先发送 Witness 的 Oracle（承诺） $\left[\left[w\right]\right]$  给验证者 。
2.  **检查门 (Gate Check):** 双方运行一个 **ZeroCheck PIOP**，证明门恒等式成立（即所有门的计算逻辑正确） 。
3.  **检查连线 (Wiring Check):** 双方运行一个 **Permutation PIOP**，证明连线恒等式成立（即置换  $\sigma$  被正确执行） 。
4.  **检查输入 (Input Check):** 验证者通过在随机点  $r$  查询 Oracle，检查公共输入  $p\left(r\right)$  是否等于 Witness  $w\left(r\right)$ 。

### 3\. 性能与优化

这一章最后给出了协议的复杂度分析和优化方案：

*   **线性时间证明者:** 证明者的时间复杂度为  $O\left(nd\log ^{2}d\right)$ （其中  $n$  是门数量， $d$  是门的度数），实现了线性时间 。
*   **对数级验证者:** 验证者的时间复杂度仅为  $O\left(\mu +l\right)$ （与变量数线性相关，即与电路大小呈对数关系） 。
*   **批处理优化 (Batching):** 论文指出，底层的 SumCheck 协议（用于 ZeroCheck 和 Permutation Check）可以通过随机线性组合**合并为一个单一的 SumCheck**。这样可以显著减少查询次数和通信量 。

**总结：** 第四章正式确立了 HyperPlonk 的工作方式：它不再依赖单变量多项式和 FFT，而是将电路约束定义在布尔超立方体上，并利用 SumCheck、ZeroCheck 和 Permutation Check 这一套“组合拳”来高效地证明电路的正确性。



为了帮你理解如何将第三章的“工具箱”组合成第四章的 **HyperPlonk 协议**，我们可以把这个过程想象成**搭积木**或者**层层外包**的过程。

HyperPlonk 的核心思想是：把复杂的电路验证问题，一层层向下“外包”，最后全部变成一个最基础的数学问题——**SumCheck**。

我们可以参考论文中的 **Figure 1** ，它完美展示了这种层级依赖关系。下面我用一个具体的**电路验证场景**来演示这种组合过程。

* * *

### 场景设定：我们要验证一个电路

假设我们有一个电路，包含  $2^{\mu }$  个门。我们需要证明两件事：

1.  **算得对（Gate Identity）**：比如  $a\times b=c$ 。
2.  **连得对（Wiring Identity）**：比如门 1 的输出  $c$  正确连接到了门 2 的输入。

### 组合步骤 1：处理“连线”（Wiring Identity）

这是最复杂的一条线，它展示了第三章工具是如何像俄罗斯套娃一样组合的。

1.  **最高层需求**：证明连线正确。
    *   **数学表达**： $w\left(x\right)=w\left(\sigma \left(x\right)\right)$  （我的值等于置换后的值）。
    *   **使用的工具**：调用 **Permutation Check (置换校验)** 。
2.  **外包给下一层**：
    *   Permutation Check 说：“要证明置换关系太难了，但我知道如果把数值和索引打包成元组，证明这两个元组集合相等就可以了。”
    *   **使用的工具**：调用 **Multiset Check (多重集校验)** 。
3.  **再外包**：
    *   Multiset Check 说：“要比对集合太麻烦，但我有个技巧。如果给每个元素加个随机数  $r$ （指纹化），然后把它们连乘起来，只要乘积相等，集合就相等。”
    *   **使用的工具**：调用 **Product Check (乘积校验)** 。
4.  **继续外包**：
    *   Product Check 说：“要算连乘积（Product）很难，但我可以构建一棵乘积树，只要证明树的父子节点关系正确（ $Parent-Left\cdot Right=0$ ）就行了。”
    *   **使用的工具**：调用 **ZeroCheck (零值校验)** 。
5.  **最终外包**：
    *   ZeroCheck 说：“要检查所有点都为 0 很累，但我可以引入随机挑战，把它们加权求和。只要总和为 0，原多项式就全是 0。”
    *   **使用的工具**：调用 **SumCheck** 。

**结论**：通过这 5 层外包，复杂的“连线检查”最终变成了一个 **SumCheck** 问题。

* * *

### 组合步骤 2：处理“计算逻辑”（Gate Identity）

这条线比较短，直接利用 ZeroCheck。

1.  **最高层需求**：证明每个门计算正确。
    *   **数学表达**： $G\left(x\right)=q_{L}\cdot a+q_{R}\cdot b+q_{M}\cdot a\cdot b-c=0$ （在所有点上成立）。
2.  **外包**：
    *   这直接就是一个“检查全为 0”的问题。
    *   **使用的工具**：直接调用 **ZeroCheck** 。
3.  **最终外包**：
    *   ZeroCheck 再次把它转化为 **SumCheck**。

* * *

### 组合步骤 3：HyperPlonk 的“总装车间” (The PolyIOP)

现在我们手里有了两个独立的任务流，最终都指向了 SumCheck。第四章的 **PolyIOP** 就是负责把它们并在一次交互中完成的“总指挥”。

**实际执行流程（组合）：**

1.  **随机挑战（Batching）**： 验证者发送随机数  $\alpha$ 。PolyIOP 说：“别分别跑两个 SumCheck 了，太浪费。” 它构造一个超级多项式  $F$ ：
    $$
    F=\left(连线检查的归约多项式\right)+\alpha \cdot \left(门检查的归约多项式\right)
    $$
2.  **统一执行**： 现在，证明者只需要对这个超级多项式  $F$  运行**一次 SumCheck 协议**。
    *   如果这个 SumCheck 通过，说明  $F$  的和为 0。
    *   说明“连线检查”和“门检查”同时通过。
    *   说明电路验证成功。

### 总结图示

你可以想象一个倒金字塔结构：

*   **顶层（HyperPlonk 协议）**：我要证明电路是对的。
    *   **左支柱（连线）**：Permutation Check  $\to$  Multiset Check  $\to$  Product Check  $\to$  ZeroCheck
    *   **右支柱（逻辑）**：Gate Identity  $\to$  ZeroCheck
*   **汇聚点**：**Batching (随机线性组合)**
*   **底层基石**：**SumCheck**

这就是论文第四章如何利用第三章定义的各种“Check”工具，像搭积木一样，将复杂的电路验证逻辑层层转化，最终简化为单一的 SumCheck 协议的过程。



