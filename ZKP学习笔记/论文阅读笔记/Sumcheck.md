# Sumcheck

在 HyperPlonk 这篇论文中，**SumCheck** 协议是整个系统的基石。HyperPlonk 之所以能移除昂贵的 FFT（快速傅里叶变换）并实现线性时间的证明者（Prover），核心原因就是它利用了针对布尔超立方体优化的 SumCheck 协议 。

以下是基于论文内容的 SumCheck 协议详细介绍：

### 1\. SumCheck 的基本定义

SumCheck 协议用于证明一个多变量多项式  $f$  在布尔超立方体  $B_{\mu }:=\{0,1\}^{\mu }$  上的所有求和结果等于某个声明的值  $v$ 。 形式化地说，证明者（Prover, P）试图向验证者（Verifier, V）证明：

$$
\sum_{b\in \{0,1\}^{\mu }} f\left(b\right)=v
$$

其中  $f$  是一个  $\mu$  个变量、单变量最高次数为  $d$  的多项式 。

### 2\. 协议执行流程 (The Protocol)

SumCheck 是一个交互式协议，共进行  $\mu$  轮（对应  $\mu$  个变量）。

*   **初始化：** 证明者声称求和结果为  $v$ 。
*   **第  $i$  轮 ( $i$  从  $\mu$  到 1)：**
    1.  **证明者计算：** 证明者将前  $i-1$  个变量视为固定（基于之前的随机挑战），将第  $i$  个变量视为未知量  $X$ ，对剩余变量求和，从而计算出一个单变量多项式  $r_{i}\left(X\right)$ 。
        *   该多项式  $r_{i}$  的次数至多为  $d$ 。
    2.  **证明者发送：** 证明者将  $r_{i}\left(X\right)$  发送给验证者 。
    3.  **验证者检查：** 验证者检查  $v=r_{i}\left(0\right)+r_{i}\left(1\right)$  是否成立。如果成立，说明当前轮次的求和与上一轮声明的值一致 。
    4.  **验证者挑战：** 验证者随机采样一个挑战值  $\alpha _{i}\in F$  发送给证明者，并更新目标值为  $v\leftarrow r_{i}\left(\alpha _{i}\right)$ 。
*   **最终检查：** 在  $\mu$  轮结束后，验证者拥有了随机向量  $\left(\alpha _{1},\dots ,\alpha _{\mu }\right)$ 。验证者需要通过一次对多变量多项式  $f$  的查询（Oracle Query）来确认  $f\left(\alpha _{1},\dots ,\alpha _{\mu }\right)$  是否等于最后更新的  $v$ 。

### 3\. HyperPlonk 对 SumCheck 的关键优化

为了在 HyperPlonk 中高效使用 SumCheck，论文提出了几项重要的优化：

#### A. 仅发送 Oracle 而非完整多项式

在经典 SumCheck 中，证明者每轮发送完整的多项式系数。HyperPlonk 中，证明者发送的是该单变量多项式  $r_{i}$  的 **Oracle（承诺）** 。这减少了通信量，特别是当多项式度数  $d$  较高时（例如用于高阶自定义门）。

#### B. 验证点的缩减优化

通常验证者需要评估  $r_{i}\left(0\right),r_{i}\left(1\right)$  和  $r_{i}\left(\alpha _{i}\right)$ 。论文提出了一种优化，证明者发送一个辅助多项式  $r_{i′}\left(X\right)$  和值  $r_{i}\left(0\right)$ ，验证者只需**查询一个点**即可完成验证。 公式如下：

$$
r_{i}\left(\alpha \right)=\left(1-\alpha \right)r_{i}\left(0\right)+\alpha \left(v-r_{i}\left(0\right)\right)+\alpha \left(1-\alpha \right)r_{i′}\left(\alpha \right)
$$

这使得每轮只需查询一次 Oracle 和传输一个域元素 。

#### C. 线性时间的证明者算法

这是 HyperPlonk 性能的核心。对于高阶多项式，朴素地计算每轮的  $r_{i}\left(X\right)$  可能很慢。论文改编了 \[72, 79\] 的算法，利用动态规划表（Dynamic Programming），使得证明者可以在 ** $O\left(2^{\mu }\cdot d\log ^{2}d\right)$ ** 的时间内完成计算，而不是  $O\left(2^{\mu }\cdot d^{2}\right)$  或更高 。

*   该算法并行化程度很高，适合硬件加速 。
*   它利用了快速多项式乘法（对于支持 FFT 的域）。

### 4\. 复杂度分析

对于一个  $\mu$  个变量、度数为  $d$  的多项式  $f$ ：

*   **证明者时间：**  $O\left(2^{\mu }d\log ^{2}d\right)$ 。
*   **验证者时间：**  $O\left(\mu \right)$ 。验证者只需进行少量的域运算和最后一次多项式查询。
*   **证明大小：**  $O\left(d\cdot \mu \right)$  个域元素（如果不使用 Oracle 优化），或者更小 。
*   **轮数：**  $\mu$  轮 。

### 5\. 在 HyperPlonk 中的应用

SumCheck 在 HyperPlonk 中不仅仅用于简单的求和，它是构建其他复杂校验的基础：

*   **ZeroCheck (零值校验)：** 用于证明门约束（Gate Identity）。通过引入随机多项式  $eq\left(X,r\right)$ ，将“检查多项式在超立方体上全为 0”的问题转化为 SumCheck 问题 。
*   **Permutation Check (置换校验)：** 用于证明连线约束（Wiring Identity）。通过将集合相等性检查转化为 ProductCheck，再转化为 ZeroCheck，最后归结为 SumCheck 。
*   **Batching (批处理)：** 多个 SumCheck 实例可以通过随机线性组合（Random Linear Combination）合并为一个实例进行证明，从而分摊成本 。

总结来说，HyperPlonk 中的 SumCheck 是一个经过高度优化的协议，它使得系统能够处理包含高阶自定义门的大型电路，同时保持证明生成过程的线性时间复杂度，从而消除了传统 Plonk 中 FFT 的瓶颈。



# Sumcheck例子

为了让你更直观地理解 SumCheck 协议，我们结合 HyperPlonk 论文中的定义（Source 1），构造一个具体的数学例子。

### 场景设定

假设我们有一个包含 2 个变量的多项式  $f\left(X_{1},X_{2}\right)$ ，我们需要证明它在布尔超立方体  $B_{2}=\{0,1\}^{2}$  上的总和。

**1\. 定义多项式：** 设  $f\left(X_{1},X_{2}\right)=2X_{1}+X_{1}X_{2}$ 。

**2\. 目标：** 证明者（Prover, P）想向验证者（Verifier, V）证明该多项式在所有布尔输入下的和  $H$  为 5。 我们先自己在心里算一下正确答案（验证者此时还不知道）：

*    $f\left(0,0\right)=2\left(0\right)+0=0$ 
*    $f\left(0,1\right)=2\left(0\right)+0=0$ 
*    $f\left(1,0\right)=2\left(1\right)+0=2$ 
*    $f\left(1,1\right)=2\left(1\right)+1=3$ 
*    **总和  $H=0+0+2+3=5$ **。

HyperPlonk 论文中描述的 SumCheck 顺序是从变量  $X_{\mu }$  到  $X_{1}$  逆序进行的 。在这个例子中，即先处理  $X_{2}$ ，再处理  $X_{1}$ 。

* * *

### 第一轮：处理变量  $X_{2}$  ( $i=2$ )

根据论文定义，第一轮的多项式  $r_{2}\left(X\right)$  是将  $X_{2}$  视为变量  $X$ ，对剩下的变量（这里是  $X_{1}$ ）在布尔值  $\{0,1\}$  上进行求和 。

1. **证明者计算  $r_{2}\left(X\right)$ ：**
   $$
   r_{2}\left(X\right)=\sum_{x_{1}\in \{0,1\}} f\left(x_{1},X\right)
   $$

   *   当  $x_{1}=0$  时： $f\left(0,X\right)=2\left(0\right)+0\left(X\right)=0$ 
   *   当  $x_{1}=1$  时： $f\left(1,X\right)=2\left(1\right)+1\left(X\right)=2+X$ 
   *   **结果：**  $r_{2}\left(X\right)=0+\left(2+X\right)=X+2$ 

2. **证明者发送：** 证明者将单变量多项式  $r_{2}\left(X\right)=X+2$  发送给验证者（在 HyperPlonk 中，通常发送的是这个多项式的 Oracle/承诺 ）。

3. **验证者检查：** 验证者检查  $r_{2}\left(0\right)+r_{2}\left(1\right)$  是否等于声明的总和  $H=5$ 。

   *    $r_{2}\left(0\right)=0+2=2$ 
   *    $r_{2}\left(1\right)=1+2=3$ 
   *    $2+3=5$ 。**检查通过。**

4. **验证者挑战：** 验证者随机选择一个挑战值  $\alpha _{2}$ （比如  $\alpha _{2}=4$ ）发送给证明者，并更新目标值为  $v\leftarrow r_{2}\left(4\right)=4+2=6$ 。

* * *

### 第二轮：处理变量  $X_{1}$  ( $i=1$ )

现在变量  $X_{2}$  已经被固定为  $\alpha _{2}=4$ 。这一轮我们要处理  $X_{1}$ 。根据定义，因为  $i=1$ ，前面没有变量需要求和了，所以  $r_{1}\left(X\right)$  就是将  $X_{1}$  作为变量，后面的变量固定为之前的挑战值的多项式 。

1. **证明者计算  $r_{1}\left(X\right)$ ：**
   $$
   r_{1}\left(X\right)=f\left(X,\alpha _{2}\right)=f\left(X,4\right)
   $$
   我们将  $X_{2}=4$  代入原多项式  $2X_{1}+X_{1}X_{2}$ ：

   *    $f\left(X,4\right)=2X+X\left(4\right)=6X$ 
   *    **结果：**  $r_{1}\left(X\right)=6X$ 

2. **证明者发送：** 证明者发送  $r_{1}\left(X\right)=6X$ 。

3. **验证者检查：** 验证者检查  $r_{1}\left(0\right)+r_{1}\left(1\right)$  是否等于上一轮更新后的目标值  $v=6$ 。

   *    $r_{1}\left(0\right)=6\left(0\right)=0$ 
   *    $r_{1}\left(1\right)=6\left(1\right)=6$ 
   *    $0+6=6$ 。**检查通过。**

4. **验证者挑战：** 验证者随机选择一个挑战值  $\alpha _{1}$ （比如  $\alpha _{1}=3$ ）发送给证明者，并更新目标值  $v\leftarrow r_{1}\left(3\right)=6\left(3\right)=18$ 。

* * *

### 最终验证 (Final Check)

所有轮次结束后，验证者拥有了随机点  $\left(\alpha _{1},\alpha _{2}\right)=\left(3,4\right)$  和最终的目标值  $18$ 。

验证者通过 Oracle 查询原多项式  $f$  在点  $\left(3,4\right)$  的值 ：

*   **验证者计算：**  $f\left(3,4\right)=2\left(3\right)+3\left(4\right)=6+12=18$ 。
*   **比较：** 计算结果  $18$  等于最终目标值  $18$ 。

**结论：** 验证通过。验证者以极高的概率确信原多项式在布尔超立方体上的和确实是 5。

### HyperPlonk 的特别之处

在这个例子中，你可以看到：

1.  **无需 FFT：** 证明者只需要进行简单的代数运算（计算  $r_{i}\left(X\right)$ ），不需要进行傅里叶变换，这正是 HyperPlonk 相比传统 Plonk 的核心优势 。
2.  **Oracle 优化：** 在实际的 HyperPlonk 协议中，为了进一步压缩数据，证明者在每一轮可能不会发送完整的  $r_{i}\left(X\right)$ ，而是发送一个 Oracle（承诺），验证者只需要在这个 Oracle 上查询一个点即可 。



# Sumcheck-PIOP

基于 HyperPlonk 论文，**SumCheck PIOP** 是该系统为了支持\*\*高阶多项式（High-degree polynomials）\*\*并实现线性时间证明者而设计的核心构建模块。

它改编自经典的 SumCheck 协议，将其放入了多项式交互式预言机证明（Polynomial Interactive Oracle Proof, PIOP）的框架中。

以下是关于 SumCheck PIOP 的详细介绍：

### 1\. 定义与目标

SumCheck PIOP 用于证明一个多变量多项式在布尔超立方体上的求和结果。

* **关系  $R_{SUM}$ ：** 给定一个  $\mu$  个变量、单变量最高次数为  $d$  的多项式  $f\in F_{\mu \left(\le d\right)}$ ，以及一个声明的值  $v$ ，证明者试图证明：
  $$
  \sum_{b\in \{0,1\}^{\mu }} f\left(b\right)=v
  $$

### 2\. 协议流程 (The Protocol)

该协议分为  $\mu$  轮。在第  $i$  轮（ $i$  从  $\mu$  递减到 1），流程如下：

1.  **证明者计算（Prover Computation）：** 证明者计算一个单变量多项式  $r_{i}\left(X\right)$ 。该多项式代表了在固定了前  $i-1$  个变量（由验证者之前的挑战决定）并将第  $i$  个变量设为  $X$  后，对剩余变量在布尔超立方体上的求和结果 。
    *   $r_{i}\left(X\right)$  的度数至多为  $d$ 。
2.  **发送 Oracle（Sending Oracle）：** 与经典 SumCheck 直接发送多项式系数不同，在 PIOP 中，证明者发送的是该单变量多项式  $r_{i}$  的 **Oracle（预言机/承诺）** \[\[r\_i\]\] 。
3.  **验证者检查与挑战（Verifier Check & Challenge）：**
    *   验证者检查当前声明的和  $v$  是否满足  $v=r_{i}\left(0\right)+r_{i}\left(1\right)$ 。
    *   验证者采样一个随机挑战点  $\alpha _{i}\leftarrow F$  发送给证明者 。
    *   验证者更新目标值： $v\leftarrow r_{i}\left(\alpha _{i}\right)$ 。
4.  **最终检查（Final Check）：** 协议结束后，验证者通过一次查询，检查原多项式  $f$  在所有挑战点  $\left(\alpha _{1},\dots ,\alpha _{\mu }\right)$  处的值是否等于最终更新的  $v$ 。

### 3\. 针对高阶多项式的关键优化

HyperPlonk 中的 SumCheck PIOP 针对高阶多项式（High-degree polynomials）做了两项重要优化，以提高效率并减少证明大小：

#### 优化 A：发送 Oracle 而非系数

*   **问题：** 如果多项式  $f$  对应于高阶自定义门（例如 degree-32），那么每一轮的单变量多项式  $r_{i}\left(X\right)$  的度数  $d$  也会很高。如果直接发送系数，通信成本会随着  $d$  线性增长。
*   **解决方案：** 证明者每轮只发送  $r_{i}$  的 Oracle。这使得通信量与度数  $d$  解耦，验证复杂度降低 。

#### 优化 B：每轮仅需一次查询

* **问题：** 标准协议中，验证者需要获取  $r_{i}\left(0\right)$ 、  $r_{i}\left(1\right)$  和  $r_{i}\left(\alpha _{i}\right)$  三个值。

* **解决方案：** 论文提出了一种优化技巧，证明者发送一个辅助的“商”多项式  $r_{i′}\left(X\right)$  的 Oracle 以及值  $r_{i}\left(0\right)$ 。验证者可以通过以下公式仅通过**一次查询**  $r_{i′}\left(\alpha _{i}\right)$  来恢复所需信息：
  $$
  r_{i}\left(\alpha \right)=\left(1-\alpha \right)r_{i}\left(0\right)+\alpha \left(v-r_{i}\left(0\right)\right)+\alpha \left(1-\alpha \right)r_{i′}\left(\alpha \right)
  $$
  这进一步缩减了证明大小 。

### 4\. 证明者算法的效率 (Prover Complexity)

为了实现线性时间的证明者，HyperPlonk 采用了基于动态规划的算法来计算每一轮的  $r_{i}\left(X\right)$ ：

*   **算法核心：** 利用之前的计算结果表（Table），每轮将表的大小减半。
*   **高阶处理：** 对于形式为  $h\left(g_{1}\left(X\right),\dots ,g_{c}\left(X\right)\right)$  的高阶多项式（其中  $h$  是电路， $g$  是多线性多项式），证明者不需要在  $O\left(d\right)$  个点上逐个求值。相反，算法利用快速多项式乘法（Fast Polynomial Multiplication）或 FFT（针对单变量）来加速计算 。
*   **时间复杂度：** 对于低深度的  $h$ （可以  $O\left(d\right)$  时间计算），证明者的总时间复杂度为 ** $O\left(2^{\mu }\cdot d\log ^{2}d\right)$ ** 。这是极其高效的，因为它避免了对整个电路进行昂贵的 FFT。

### 5\. 复杂度总结 (Complexity Summary)

对于  $\mu$  个变量、度数为  $d$  的多项式  $f$ ：

*   **证明者时间 ( $tp$ )：**  $O\left(2^{\mu }\cdot d\log ^{2}d\right)$  次域运算 。
*   **验证者时间 ( $tv$ )：**  $O\left(\mu \right)$ 。
*   **查询复杂度 ( $q$ )：**  $\mu +1$  次（每轮查询一次单变量 Oracle，最后查询一次多变量 Oracle） 。
*   **通信量：** 证明者发送  $\mu$  个域元素（每轮一个  $r_{i}\left(0\right)$ ） 。
*   **证明 Oracle 大小：**  $d\cdot \mu$ （取决于多项式的度数） 。

### 6\. 在 HyperPlonk 中的作用

SumCheck PIOP 是 HyperPlonk 协议栈的最底层基础：

*   它直接被用于 **ZeroCheck**（零值校验） 。
*   ZeroCheck 进而被用于 **ProductCheck** 。
*   ProductCheck 进而被用于 **Permutation Check**（置换校验）和 **Lookup**（查找表） 。 因此，SumCheck PIOP 的效率直接决定了整个 HyperPlonk 系统的证明生成效率。



# Sumcheck-PIOP例子

这是一个基于 HyperPlonk 论文中 **SumCheck PIOP**（多项式交互式预言机证明）的具体例子。

这个例子与经典 SumCheck 的核心数学逻辑一致，但请注意**关键的区别**：在 PIOP 中，证明者发送的是每一轮多项式的 **Oracle（预言机/承诺）**，而不是直接发送系数 。

### 场景设定

*   **多项式**：假设有一个 2 变元多项式  $f\left(X_{1},X_{2}\right)=2X_{1}+X_{1}X_{2}$ 。
    *    $f\in F_{2\left(\le 2\right)}$ （2 个变量，度数为 2）。
*   **目标**：证明  $f$  在布尔超立方体  $B_{2}=\{0,1\}^{2}$  上的总和  $v=5$ 。
    *   验证（自己算）： $f\left(0,0\right)=0,f\left(0,1\right)=0,f\left(1,0\right)=2,f\left(1,1\right)=3$ 。总和  $0+0+2+3=5$ 。
*   **交互过程**：协议分为  $\mu =2$  轮，变量处理顺序通常从  $X_{\mu }$  到  $X_{1}$ （即先处理  $X_{2}$ ，再处理  $X_{1}$ ）。

* * *

### 第一轮：处理变量  $X_{2}$  ( $i=2$ )

1. **证明者计算  $r_{2}\left(X\right)$ **： 根据公式  $r_{i}\left(X\right):=\sum_{b\in B_{i-1}} f\left(b,X,\alpha _{i+1},\dots \right)$ ，这里需对  $X_{1}$ （即  $b\in \{0,1\}$ ）求和，保留  $X_{2}$  为变量  $X$ 。
   $$
   r_{2}\left(X\right)=f\left(0,X\right)+f\left(1,X\right)
   $$

   *    $f\left(0,X\right)=2\left(0\right)+0\left(X\right)=0$ 
   *    $f\left(1,X\right)=2\left(1\right)+1\left(X\right)=2+X$ 
   *    **结果**： $r_{2}\left(X\right)=X+2$ 

2. **PIOP 关键动作**： 证明者**不直接发送**多项式  $X+2$ ，而是发送该多项式的 **Oracle（承诺）**  $\left[\left[r_{2}\right]\right]$  给验证者 。

   *   注：为了验证方便，证明者通常会同时发送  $r_{2}\left(0\right)$  和  $r_{2}\left(1\right)$  的值，或者使用论文中提到的优化（仅发送一个辅助预言机和  $r_{2}\left(0\right)$ ）。

3. **验证者检查**： 验证者检查  $v=^{?} r_{2}\left(0\right)+r_{2}\left(1\right)$ 。

   *   假设验证者通过查询 Oracle（或接收值）得到： $r_{2}\left(0\right)=2,r_{2}\left(1\right)=3$ 。
   *   $2+3=5$ 。检查通过（与声明的总和  $v=5$  一致）。

4. **验证者挑战**： 验证者随机采样  $\alpha _{2}\leftarrow F$ （假设  $\alpha _{2}=4$ ），发送给证明者 。 验证者更新目标值： $v\leftarrow r_{2}\left(\alpha _{2}\right)=4+2=6$ 。

* * *

### 第二轮：处理变量  $X_{1}$  ( $i=1$ )

1. **证明者计算  $r_{1}\left(X\right)$ **： 公式中  $B_{i-1}=B_{0}$  为空集（无需对任何变量求和）， $X_{2}$  已被固定为  $\alpha _{2}=4$ 。
   $$
   r_{1}\left(X\right)=f\left(X,4\right)
   $$

   *   代入原多项式： $2X+X\left(4\right)=6X$ 
   *   **结果**： $r_{1}\left(X\right)=6X$ 

2. **PIOP 关键动作**： 证明者发送多项式  $r_{1}\left(X\right)$  的 **Oracle**  $\left[\left[r_{1}\right]\right]$  给验证者 。

3. **验证者检查**： 验证者检查  $v=^{?} r_{1}\left(0\right)+r_{1}\left(1\right)$ 。

   *    $r_{1}\left(0\right)=0,r_{1}\left(1\right)=6$ 。
   *    $0+6=6$ 。检查通过（与当前目标值  $v=6$  一致）。

4. **验证者挑战**： 验证者随机采样  $\alpha _{1}\leftarrow F$ （假设  $\alpha _{1}=3$ ），发送给证明者 。 验证者更新目标值： $v\leftarrow r_{1}\left(\alpha _{1}\right)=6\left(3\right)=18$ 。

* * *

### 最终验证 (Final Check)

协议结束时，验证者手中有随机点  $\left(\alpha _{1},\alpha _{2}\right)=\left(3,4\right)$  和最终声称的值  $18$ 。

1. **查询 Oracle**： 验证者对原始多项式  $f$  的 Oracle  $\left[\left[f\right]\right]$  发起查询，位置为  $\left(\alpha _{1},\alpha _{2}\right)$ 。

   *   _在 PIOP 编译为 SNARK 后，这一步通常对应于多项式承诺方案（如 KZG）的 Evaluation Proof。_

2. **验证计算**： 验证者计算（或通过电路约束检查）：
   $$
   f\left(3,4\right)=2\left(3\right)+3\left(4\right)=6+12=18
   $$
   结果与  $v=18$  一致，验证通过 。

### PIOP 的特殊优化点 (HyperPlonk 特有)

在上述标准流程中，验证者每轮需要查询  $r_{i}\left(0\right)$ 、 $r_{i}\left(1\right)$  和  $r_{i}\left(\alpha _{i}\right)$ 。HyperPlonk 的 SumCheck PIOP 包含一个优化：

*   证明者可以发送一个降阶（degree  $d-2$ ）的辅助多项式 Oracle  $\left[\left[r_{i′}\right]\right]$  以及值  $r_{i}\left(0\right)$ 。
*   验证者利用公式  $r_{i}\left(\alpha \right)=\left(1-\alpha \right)r_{i}\left(0\right)+\alpha \left(v-r_{i}\left(0\right)\right)+\alpha \left(1-\alpha \right)r_{i′}\left(\alpha \right)$  来计算  $r_{i}\left(\alpha \right)$ 。
*   这样，每一轮验证者只需要进行**一次 Oracle 查询**（查询  $\left[\left[r_{i′}\right]\right]$  在  $\alpha _{i}$  处的值）。

