# Lookup PIOP

**Lookup PIOP** 是 HyperPlonk 协议中用于支持**查找表（Lookup Table）**功能的机制。

它的作用是证明：电路中的某些中间变量（Witness）的值，肯定存在于一个预定义的表格（Table）中。这常用于范围检查（Range Check，比如证明数值在 0-255 之间）或复杂的位运算（比如 XOR、AND）。

在 HyperPlonk 中实现 Lookup 比传统 Plonk 要难得多，因为 HyperPlonk 运行在布尔超立方体上，缺乏自然的“顺序”概念。

* * *

### 1\. 核心挑战：如何在立方体上“排队”？

传统的 **Plookup** 协议依赖于一个核心操作：**“Next”（下一个）**。

*   在传统 Plonk 中，数据在循环群上，天生有一个旋转操作  $x\to \omega x$ ，可以很容易地拿到“下一个”元素。
*   Plookup 的逻辑是：把查找值插入到表中并**排序**，如果排序后的序列中，相邻元素要么相等、要么是表中的相邻元素，那么查找就是合法的。

**HyperPlonk 的难题**： 在布尔超立方体  $\{0,1\}^{\mu }$  上，没有一个简单的线性函数能像“旋转”一样遍历所有点。如果强行构造一个非线性函数来遍历，会导致多项式度数爆炸，证明成本剧增。

### 2\. HyperPlonk 的解决方案

Lookup PIOP 通过两个巧妙的数学构建解决了这个问题：

#### A. 构造“伪”遍历路径 (Quadratic Generator)

论文利用有限域理论（基于本原多项式），构造了一个**二次函数**  $g_{\mu }\left(x\right)$ ，它可以像贪吃蛇一样遍历超立方体上除零以外的所有点。

*   这建立了一个确定的“顺序”。

#### B. 线性化移位技巧 (Linearized Shift)

直接计算  $f\left(g_{\mu }\left(x\right)\right)$  会导致度数变高。论文设计了一个特殊的线性算子  $f_{\Delta }$ 。

*   **效果**： $f_{\Delta }\left(x\right)$  的值等于  $f\left(g_{\mu }\left(x\right)\right)$ （即  $f$  在序列中“下一个”位置的值）。
*   **优势**：这个操作不会增加多项式的度数，计算非常廉价。

### 3\. 协议流程

Lookup PIOP 的逻辑本质上是**多变量版本的 Plookup**。

**输入**：

*   **表 ( $t$ )**：预定义的数据（例如  $0\dots 255$ ）。
*   **查询 ( $f$ )**：电路中产生的需要检查的数据。

**步骤**：

1. **构造排序向量 ( $h$ )**： 证明者构造一个新的向量  $h$ ，它是  $\left(f,t\right)$  合并后按照表格  $t$  的顺序重新排列的结果。

   *   如果  $f$  中的值都在  $t$  中，那么  $h$  中的元素应当看起来像是在  $t$  的元素之间插入了  $f$  的重复项。

2. **构造“当前-下一个”对**： 利用前面提到的  $f_{\Delta }$  技巧，构造“当前值”和“下一个值”的元组。

   *   表的转换关系： $\left(t\left(x\right),t_{\Delta }\left(x\right)\right)$  —— 原始的相邻关系。
   *   排序后的转换关系： $\left(h\left(x\right),h_{\Delta }\left(x\right)\right)$  —— 混合后的相邻关系。

3. **Multiset Check（多重集校验）**： 证明者证明以下两个集合（多重集）是相等的：
   $$
   \{\left(f\left(x\right),f\left(x\right)\right)\}\cup \{\left(t\left(x\right),t_{\Delta }\left(x\right)\right)\}=\{\left(h\left(x\right),h_{\Delta }\left(x\right)\right)\}
   $$

### 4\. 具体例子

为了理解这个 Multiset Check 的逻辑，我们来看一个极简的例子。

#### 场景

*   **表格  $t$ **： $\left[10,20,30\right]$ 。
    *   表格内部的“下一个”关系是： $10\to 20$ ,  $20\to 30$ ,  $30\to 10$  (循环)。
    *   表格对： $\left(10,20\right),\left(20,30\right),\left(30,10\right)$ 。
*   **查询  $f$ **： $\left[20,10\right]$ 。 （这两个数都在表里，是合法的）。

#### 逻辑推演

1.  **构造  $h$ **：将  $f$  插入  $t$  并保持  $t$  的顺序。
    *    $h=\left[10,10,20,20,30\right]$ 。 （注意：实际长度需要填充对齐，这里仅做逻辑演示）。
    *    $h$  的“下一个”关系： $10\to 10$ ,  $10\to 20$ ,  $20\to 20$ ,  $20\to 30$ ,  $30\to 10$ 。
2.  **验证集合相等**：
    *   **左边 (LHS)**：
        *    $f$  的自对： $\left(20,20\right),\left(10,10\right)$ 
        *    $t$  的转对： $\left(10,20\right),\left(20,30\right),\left(30,10\right)$ 
        *    **LHS 集合**： $\{\left(20,20\right),\left(10,10\right),\left(10,20\right),\left(20,30\right),\left(30,10\right)\}$ 
    *   **右边 (RHS)**：
        *    $h$  的转对： $\left(10,10\right),\left(10,20\right),\left(20,20\right),\left(20,30\right),\left(30,10\right)$ 
        *    **RHS 集合**： $\{\left(10,10\right),\left(10,20\right),\left(20,20\right),\left(20,30\right),\left(30,10\right)\}$ 
3.  **结论**： 左右两边的集合完全一致。
    *   如果  $f$  中有个值（比如 15）不在  $t$  中， $h$  就无法形成这种完美的“闭环链条”，等式就不会成立。

### 总结

Lookup PIOP 是 HyperPlonk 为了克服布尔超立方体“无序”缺陷而发明的机制。它通过**数学构造的伪遍历路径**和**线性化移位**，在不牺牲性能的前提下，复现了 Plookup 的功能，让线性时间的证明者也能支持查找表。