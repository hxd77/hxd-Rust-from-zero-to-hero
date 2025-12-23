# Constraint System

在 **HyperPlonk** 论文中，**Constraint System（约束系统）** 是指一套数学规则，用于定义证明者（Prover）必须满足哪些条件，才能证明其知道正确的秘密输入（Witness）。

简单来说，约束系统就是把“程序的运行逻辑”翻译成“多项式方程”。HyperPlonk 定义了两种主要的约束系统：基础的 ** $R_{PLONK}$ ** 和支持查找表的 ** $R_{PLONK+}$ **。

以下是详细介绍：

* * *

### 1\. 基础约束系统 ( $R_{PLONK}$ )

这是 HyperPlonk 的核心模型，它是在布尔超立方体  $B_{\mu }=\{0,1\}^{\mu }$  上定义的，是对传统 Plonk 约束系统的泛化。

一个合法的 HyperPlonk 实例必须同时满足以下**三个核心恒等式（Identities）**：

#### A. 门恒等式 (Gate Identity)

这是描述电路逻辑（如加法、乘法或更复杂的运算）的规则。

* **定义**：对于每一个门  $x$ （超立方体上的点），其输入线（Witness  $w$ ）和控制开关（Selector  $q$ ）必须满足特定的代数方程。

* **公式**：
  $$
  f\left(q\left(x\right),w\left(x\right)\right)=0
  $$
  其中  $f$  是一个高阶代数函数。

* **HyperPlonk 的优势**：

  *   传统的 Plonk 受限于 FFT，只能高效处理低阶门（如 degree-2 的乘法门）。
  *   HyperPlonk **没有 FFT**，因此支持**极高阶的自定义门（High-Degree Custom Gates）**。例如，它可以定义一个 degree-5 甚至 degree-32 的门而不会显著增加证明时间。这意味着它可以把复杂的逻辑（如哈希函数的一轮运算）压缩进一个门里。

* **校验方法**：使用 **ZeroCheck** 证明该多项式在所有点上为 0。

#### B. 连线恒等式 (Wiring Identity) / 复制约束 (Copy Constraint)

这是描述电路连线（即数据如何流动）的规则。比如，第 1 个门的输出必须等于第 5 个门的输入。

* **定义**：通过一个预定义的**置换（Permutation  $\sigma$ ）**，重新排列 witness 的值，使得原本在不同位置但物理上相连的值变得相等。

* **公式**：
  $$
  w\left(x\right)=w\left(\sigma \left(x\right)\right)
  $$

* **校验方法**：使用 **Permutation Check**。

#### C. 公共输入一致性 (Public Input Consistency)

这是确保证明是针对特定公共输入生成的规则。

* **定义**：Witness 中的某一部分必须等于公开的输入值  $p$ 。

* **公式**：
  $$
  p\left(x\right)=w\left(x\right)
  $$
  （针对特定的  $x$  范围）

* **校验方法**：直接在随机点检查  $p$  和  $w$  的值是否一致。

* * *

### 2\. 增强约束系统 ( $R_{PLONK+}$ )

这是在基础系统上增加了**查找表（Lookup Gates）**功能的版本。

#### 为什么需要它？

代数约束（加减乘除）很难高效表达某些逻辑，比如：

*   **范围检查**：证明  $0\le x<256$ 。用代数方程需要检查  $\left(x\right)\left(x-1\right)\dots \left(x-255\right)=0$ ，度数太高。
*   **位运算**：XOR, AND 等操作。

#### 查找约束 (Lookup Constraint)

* **定义**：证明电路中的某些中间值（由 selector 和 witness 组合而成）必然存在于一个预定义的\*\*表格（Table）\*\*中。

* **公式**：
  $$
  g\left(q\left(x\right),w\left(x\right)\right)\in Table
  $$
  其中  $g$  是查找函数， $Table$  是预先计算好的合法值列表（如  $0\dots 255$ ）。

* **校验方法**：使用 **Lookup PIOP**（基于 Multiset Check）。

* * *

### 3\. HyperPlonk 约束系统的特点总结

| 特性         | 传统 R1CS (Spartan 等)                 | 传统 Plonk                             | \*\*HyperPlonk ( $R_{PLONK}$ ) \*\*        |
| ------------ | -------------------------------------- | -------------------------------------- | ------------------------------------------ |
| **基础结构** | 矩阵乘法  $\left(Az\circ Bz=Cz\right)$ | 单变量多项式                           | **多线性多项式 (Multilinear Polynomials)** |
| **域**       | 任意域                                 | 乘法子群 (需要 FFT 友好)               | **布尔超立方体 (Boolean Hypercube)**       |
| **门度数**   | 通常限制为 2                           | 限制较低 (受 FFT 限制)                 | **极高 (只受域运算限制，无关 FFT)**        |
| **证明生成** | 线性时间                               | $O\left(N\log N\right)$  (由 FFT 主导) | **线性时间  $O\left(N\right)$ **           |

### 举个简单的例子

假设你要证明你知道两个数  $a,b$ ，使得  $a\cdot b=12$ ，且  $a$  是一个 0 到 9 之间的数字。

在 **HyperPlonk 约束系统**中，这会被表示为：

1.  **Gate Identity**（算术逻辑）： 设置一个乘法门，约束  $w_{a}\left(x\right)\cdot w_{b}\left(x\right)-w_{out}\left(x\right)=0$ 。且  $w_{out}$  固定为 12。
2.  **Lookup Constraint**（范围检查）： 设置一个查找表  $T=\{0,1,\dots ,9\}$ 。约束  $w_{a}\left(x\right)\in T$ 。
3.  **Wiring Identity**： 确保输入里的  $a$  和查找表里检查的那个  $a$  是同一个值。

HyperPlonk 的强大之处在于，它可以用一个极其复杂的公式（高阶门）一次性表达比  $a\cdot b$  复杂得多的逻辑，而不需要像 R1CS 那样拆分成无数个小步骤。



# Gate Identity 、Wiring Identity 和 Plookup



在 **HyperPlonk**（以及标准的 Plonk）协议中，要证明一个电路的计算是正确的，必须同时满足两个核心条件：**Gate Identity（门恒等式）** 和 **Wiring Identity（连线恒等式）**。

简单来说：

*   **Gate Identity** 负责检查\*\*“每一个零件（门）是否算对了”\*\*（局部逻辑）。
*   **Wiring Identity** 负责检查\*\*“零件之间的导线是否接对了”\*\*（全局连接）。

只有两者都通过，才能证明整个电路的计算结果是正确的。以下是详细解释和举例。

* * *

### 1\. Gate Identity（门恒等式）

**定义：** Gate Identity 用于确保电路中每一个逻辑门（加法、乘法或自定义门）的输入和输出满足预定的数学关系 。

**原理：** HyperPlonk 通过一组\*\*选择器多项式（Selectors,  $S$ ）\*\*来控制每个门的行为。对于每一个门  $x$ （在超立方体上的索引），都有一个约束方程必须等于 0 ：

$$
0=S_{add}\left(x\right)\cdot \left(\dots \right)+S_{mul}\left(x\right)\cdot \left(\dots \right)+\dots
$$

*   如果  $S_{add}\left(x\right)=1$ ，则激活加法约束（如  $L+R-O=0$ ）。
*   如果  $S_{mul}\left(x\right)=1$ ，则激活乘法约束（如  $L\cdot R-O=0$ ）。
*   HyperPlonk 的优势在于它支持非常**高阶的自定义门**（如 degree-5 或更高），而不像传统 Plonk 那样昂贵 。

**如何证明：** 使用 **ZeroCheck** 协议证明该约束多项式在所有点上都为 0 。

#### 举个例子：计算  $3\times 4+5=17$ 

假设我们有两个门：

*   **门 0**：乘法门（ $3\times 4=12$ ）
*   **门 1**：加法门（ $12+5=17$ ）

**Gate Identity 检查表：**

| 门索引 ( $x$ ) | 左输入 ( $L$ ) | 右输入 ( $R$ ) | 输出 ( $O$ ) | 乘法选择器 ( $S_{mul}$ ) | 加法选择器 ( $S_{add}$ ) | 常数选择器 ( $S_{const}$ ) |
| -------------- | -------------- | -------------- | ------------ | ------------------------ | ------------------------ | -------------------------- |
| **0**          | 3              | 4              | **12**       | **1**                    | 0                        | 0                          |
| **1**          | **12**         | 5              | 17           | 0                        | **1**                    | 0                          |

*   **检查门 0（乘法）：** 公式： $S_{mul}\cdot \left(L\cdot R\right)-O=0$  代入： $1\cdot \left(3\cdot 4\right)-12=12-12=0$ 。**通过。** _(注意：此时  $S_{add}$  为 0，所以加法逻辑不生效)_
*   **检查门 1（加法）：** 公式： $S_{add}\cdot \left(L+R\right)-O=0$  代入： $1\cdot \left(12+5\right)-17=17-17=0$ 。**通过。**

**注意：** Gate Identity **只管当前行**算得对不对。它**不管**门 0 的输出  $O_{0}$  是否真的等于门 1 的输入  $L_{1}$ 。那是 Wiring Identity 的工作。

* * *

### 2\. Wiring Identity（连线恒等式）

**定义：** Wiring Identity（也称为 Copy Constraint / 复制约束）用于确保电路中的值在不同位置之间正确传递。它通过一个\*\*置换（Permutation,  $\sigma$ ）\*\*来定义“谁应该等于谁” 。

**原理：** 假设电路的所有输入输出值平铺成一个大列表  $M$ 。置换函数  $\sigma \left(x\right)$  规定了索引  $x$  处的值应该来自于索引  $\sigma \left(x\right)$ 。 为了证明连线正确，必须满足：

$$
M\left(x\right)=M\left(\sigma \left(x\right)\right)
$$

这意味着：**在连线两端的位置上，数值必须相等** 。

**如何证明：** 使用 **Permutation Check**（基于 Multiset Check）来证明值集合在置换前后是相等的 。

#### 接上面的例子：连接两个门

在刚才的例子中，我们需要把 **门 0 的输出 (12)** 连到 **门 1 的左输入 (12)**。

我们把所有变量铺开编号（全局索引）：

*   索引 0:  $L_{0}$  (3)
*   索引 1:  $R_{0}$  (4)
*   **索引 2:  $O_{0}$  (12)** \<-- 输出端
*   **索引 3:  $L_{1}$  (12)** \<-- 输入端
*   索引 4:  $R_{1}$  (5)
*   索引 5:  $O_{1}$  (17)

**Wiring Identity 的任务：** 证明 **索引 2 的值** 等于 **索引 3 的值**。

**定义置换  $\sigma$ ：** 置换  $\sigma$  会交换索引 2 和 3 的位置（或者形成循环  $2\to 3\to 2$ ），其他索引保持不变（ $i\to i$ ）。

**检查过程：**

*   **对于  $x=0,1,4,5$ **：  $\sigma \left(x\right)=x$ 。检查  $M\left(x\right)=M\left(x\right)$ 。显然成立。
*   **对于  $x=3$ （关键点）**：  $\sigma \left(3\right)=2$ （定义：位置 3 的值应当来自位置 2）。 检查： $M\left(3\right)=^{?} M\left(2\right)$  代入数值： $L_{1}=^{?} O_{0}$   $\Rightarrow$   $12=12$ 。**通过。**

### 总结

想象你在组装一台机器：

1.  **Gate Identity** 就像是\*\*“零件质检员”\*\*。他拿起一个齿轮（门），检查它能不能转动（计算逻辑对不对）。如果一个加法器算出了  $1+1=3$ ，质检员就会报警（ZeroCheck 失败）。
2.  **Wiring Identity** 就像是\*\*“线路检查员”\*\*。他拿着图纸（置换  $\sigma$ ），检查这根红线是不是从A点连到了B点。如果A点电压是12V，B点电压却是5V，说明线断了或者接错了，检查员就会报警（Permutation Check 失败）。

**HyperPlonk** 必须同时通过这两个检查，才能生成有效的证明。



**Plookup**（全称 **Polynomial Lookup**）是由 Gabizon 和 Williamson 提出的一种协议，用于在零知识证明中高效地实现\*\*查找表（Lookup Table）\*\*功能。

在 HyperPlonk 这篇论文中，Plookup 被视为一种核心的构建模块（PolyIOP），用于证明电路中的某些中间值属于一个预定义的表格 。

以下是 Plookup 的详细原理解析和具体例子。

### 1\. 核心目标

*   **输入**：
    *   **查找值向量  $f$ **：电路计算出的私有数据（Witness）。
    *   **表格向量  $t$ **：预先公开的合法数据列表（如  $0∼255$  的所有整数）。
*   **目标**：证明  $f$  中的每一个元素都存在于  $t$  中。即  $f\subseteq t$ （ $f$  是  $t$  的子集）。

### 2\. 核心原理：差分与排序

Plookup 的魔法在于它不进行“搜索”，而是通过**排序**和**邻居关系检查**来验证。

#### 逻辑推导：

1.  如果我们要证明  $f$  中的数都在  $t$  里，我们可以把  $f$  和  $t$  混合在一起，形成一个新的向量  $h$ 。
2.  按照表格  $t$  的顺序对  $h$  进行排序。
3.  **关键观察**：如果  $f$  确实是  $t$  的子集，那么在排序后的  $h$  中，任意两个相邻的元素  $\left(h_{i},h_{i+1}\right)$  只有两种可能：
    *   **相等**： $h_{i}=h_{i+1}$ （说明这是插入的  $f$  中的值）。
    *   **在表中相邻**： $h_{i}$  和  $h_{i+1}$  也是原表格  $t$  中的相邻元素。
4.  如果  $h$  中出现了既不相等、也不是表内邻居的跳跃（例如表中是 10, 20，但  $h$  中出现了 10, 15），说明  $f$  中混入了非法值（15）。

#### HyperPlonk 中的数学实现

HyperPlonk 论文通过多重集校验（Multiset Check）来实现这一逻辑 。 它证明以下两个元组集合是相等的：

$$
\{\left(f(x), f(x)\right)\} 
\cup 
\{\left(t(x), t_{\text{next}}(x)\right)\}
=
\{\left(h(x), h_{\text{next}}(x)\right)\}
$$

*   **左边（LHS）**：包含两部分。
    *    $f$  的“自对”： $\left(val,val\right)$ 。这代表  $f$  并没有改变数值。
    *    $t$  的“邻居对”： $\left(val,next_val\right)$ 。这代表表格原本的顺序结构。
*   **右边（RHS）**： $h$  的“邻居对”。
    *    $h$  是  $f$  和  $t$  的混合排序结果。
    *   如果  $f$  合法， $h$  的相邻元素关系要么是“原地踏步”（来自  $f$ ），要么是“走向下一格”（来自  $t$ ）。这正好能由左边的集合拼凑出来。

* * *

### 3\. 具体例子

#### 场景设定

*   **合法表格  $t$ **： $\left[10,20,30\right]$ 。
    *   表格的“下一个”关系（循环）： $10\to 20\to 30\to 10$ 。
*   **待查数据  $f$ **： $\left[20,10\right]$ 。 （这两个数都在表里，合法）。

#### 步骤一：构造混合向量  $h$ 

证明者将  $f$  插入到  $t$  中，并保持  $t$  的顺序：

*    $t$ :  $10,20,30$ 
*    $f$ :  $10,20$ 
*   ** $h$  (排序后)**:  $\left[10,10,20,20,30\right]$  _(注：为了对齐长度，实际协议中会有填充，这里仅演示逻辑)_

#### 步骤二：构建元组集合 (Tuples)

我们来看看左右两边的“邻居对”分别是什么：

**1\. 左边集合 (LHS)**

*   **来自  $f$  (自对)**：
    *    $\left(20,20\right)$ 
    *    $\left(10,10\right)$ 
*   **来自  $t$  (表内邻居)**：
    *    $\left(10,20\right)$ 
    *    $\left(20,30\right)$ 
    *    $\left(30,10\right)$  （循环回到开头）
*   **LHS 总集**： ${\left(20,20\right),\left(10,10\right),\left(10,20\right),\left(20,30\right),\left(30,10\right)}$ 

**2\. 右边集合 (RHS)** 我们看  $h$  中相邻元素的跳变： $\left[10\to 10\to 20\to 20\to 30\to 10\right]$ 

*    $10\to 10$ :  $\left(10,10\right)$ 
*    $10\to 20$ :  $\left(10,20\right)$ 
*    $20\to 20$ :  $\left(20,20\right)$ 
*    $20\to 30$ :  $\left(20,30\right)$ 
*    $30\to 10$ :  $\left(30,10\right)$  （循环）
*   **RHS 总集**： $\{\left(10,10\right),\left(10,20\right),\left(20,20\right),\left(20,30\right),\left(30,10\right)\}$ 

#### 步骤三：比对

对比 LHS 和 RHS 的集合内容：

*   都有  $\left(10,10\right)$ 。
*   都有  $\left(20,20\right)$ 。
*   都有  $\left(10,20\right)$ 。
*   都有  $\left(20,30\right)$ 。
*   都有  $\left(30,10\right)$ 。

**结论**：集合完全相等。Plookup 验证通过。

#### 反例（非法数据）

假设  $f=\left[15\right]$ （非法值）。

*    $h$  可能会变成  $\left[10,15,20,30\right]$ 。
*   RHS 会生成一个元组 ** $\left(10,15\right)$ **。
*   但是 LHS 里：
    *    $f$  只能提供  $\left(15,15\right)$ 。
    *    $t$  只能提供  $\left(10,20\right)$ 。
*   LHS 里根本没有  $\left(10,15\right)$  这个组合。
*   **集合不相等，验证失败。**

### 4\. HyperPlonk 的特殊改进

传统的 Plookup 依赖于群的旋转（ $x\to \omega x$ ）来定义“下一个”。 HyperPlonk 运行在布尔超立方体上，没有这种旋转。因此，HyperPlonk 发明了一种基于**二次生成器 (Quadratic Generator)** 的方法来模拟这种“遍历”顺序，从而使得上述 Plookup 逻辑在超立方体上也能跑通 。



