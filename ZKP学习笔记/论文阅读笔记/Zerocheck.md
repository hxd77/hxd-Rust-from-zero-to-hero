# Zerocheck

**ZeroCheck（零值校验）** 是 HyperPlonk 协议中的一个核心组件。它的主要功能是证明一个多变量多项式在布尔超立方体上的**所有点**上的取值都为零。

在 HyperPlonk 中，ZeroCheck 主要用于证明**门恒等式（Gate Identity）**，即证明电路中所有的门约束（例如  $L_{i}\cdot R_{i}-O_{i}=0$ ）对于每一个门  $i$  都成立 。

以下是 ZeroCheck 的详细解释：

### 1\. 定义与目标

*   **关系  $R_{ZERO}$ **：给定一个  $\mu$  个变量的多项式  $f\in F_{\mu \left(\le d\right)}$ ，证明者需要向验证者证明：对于所有的  $x\in B_{\mu }=\{0,1\}^{\mu }$ ，都有  $f\left(x\right)=0$ 。
*   **挑战**：布尔超立方体  $B_{\mu }$  包含  $2^{\mu }$  个点。直接检查每个点是不可能的（对于大电路来说计算量太大）。

### 2\. 核心原理：归约到 SumCheck

ZeroCheck 巧妙地利用了 **SumCheck 协议**。它并没有逐个检查  $f\left(x\right)$  是否为 0，而是将问题转化为证明一个新的多项式的**总和**为 0。

其数学原理如下： 如果  $f\left(x\right)$  在超立方体上全为 0，那么它与任何权重的线性组合之和也应该为 0。为了安全地验证这一点，引入了一个基于随机挑战  $r$  的多项式  $eq\left(X,r\right)$ 。

### 3\. 协议流程

ZeroCheck 协议建立在 SumCheck PIOP 之上，流程如下 ：

1. **验证者挑战**：验证者采样一个随机向量  $r\leftarrow F^{\mu }$  发送给证明者。

2. **构造新多项式**：证明者构造一个新的虚拟多项式  $\hat{f}\left(X\right)$ ：
   $$
   \hat{f}\left(X\right):=f\left(X\right)\cdot eq\left(X,r\right)
   $$
   这里的  $eq\left(X,r\right)$  是相等函数的**多线性扩展（Multilinear Extension）**，定义为：
   $$
   eq\left(X,r\right):=\prod_{i=1}^{\mu } \left(X_{i}r_{i}+\left(1-X_{i}\right)\left(1-r_{i}\right)\right)
   $$

3. **运行 SumCheck**：证明者和验证者运行一个 **SumCheck PIOP**，证明  $\hat{f}\left(X\right)$  在布尔超立方体上的总和为 0：
   $$
   \sum_{x\in \{0,1\}^{\mu }} \hat{f}\left(x\right)=0
   $$

### 4\. 为什么这样能行？（完备性与可靠性）

*   **完备性（如果证明者是诚实的）：** 如果  $f\left(x\right)$  对于所有  $x\in B_{\mu }$  确实都等于 0，那么  $\hat{f}\left(x\right)=0\cdot eq\left(x,r\right)$  也恒为 0。因此， $\hat{f}$  在超立方体上的总和自然为 0，SumCheck 会通过 。
*   **可靠性（如果证明者在撒谎）：** 如果  $f\left(x\right)$  并不是在所有点上都为 0（即至少有一个点不为 0），那么辅助多项式  $g\left(Y\right):=\sum_{x\in B_{\mu }} f\left(x\right)\cdot eq\left(x,Y\right)$  就不是零多项式。根据 Schwartz-Zippel 引理，对于随机选择的  $r$ ， $\sum_{x\in B_{\mu }} f\left(x\right)\cdot eq\left(x,r\right)=0$  的概率极低 。 因此，如果  $f$  不满足条件，SumCheck 将以极高概率拒绝证明。

### 5\. 复杂度与应用

*   **效率**：ZeroCheck 的开销主要由底层的 SumCheck 决定。
    *   证明者时间： $O\left(d\log ^{2}d\cdot 2^{\mu }\right)$ （与 SumCheck 相同）。
    *   验证者时间： $O\left(\mu \right)$ 。
*   **在 HyperPlonk 中的角色**： 在 HyperPlonk 中，有一个巨大的多项式  $G\left(X\right)$  代表了所有门的约束检查。要证明电路计算正确，就等同于证明  $G\left(X\right)$  在所有输入  $x\in \{0,1\}^{\mu }$  下都为 0。这正是 ZeroCheck 的用武之地——它将成千上万个门的独立检查压缩成了一个 SumCheck 问题 。

总结来说，**ZeroCheck 是一个“适配器”**，它通过引入随机挑战  $r$  和  $eq$  函数，将“检查多项式处处为零”的问题转换成了“检查多项式总和为零”的问题，从而能够利用高效的 SumCheck 协议来解决。



# ZeroCheck例子

为了让你更直观地理解 **ZeroCheck**（零值校验），我们可以通过一个具体的例子来演示它是如何工作的，特别是它是**如何捕捉到电路中的错误**的。

ZeroCheck 的目标是证明多项式  $f\left(X\right)$  在布尔超立方体上的**每一个点**都等于 0 。如果哪怕有一个点不为 0，ZeroCheck 就会失败。

* * *

### 场景设定：一个有错误的电路

假设我们有一个非常简单的电路，只有 2 个输入变量  $X_{1},X_{2}$ （即  $\mu =2$ ），布尔超立方体有  $2^{2}=4$  种输入情况。

我们定义多项式  $f\left(X_{1},X_{2}\right)$  为电路的约束检查多项式（例如  $L\cdot R-O$ ）。

*   **理想情况**：对于所有输入  $\left(0,0\right),\left(0,1\right),\left(1,0\right),\left(1,1\right)$ ，  $f\left(x\right)$  都应该等于 0。
*   **实际情况（Bug）**：假设电路在输入  $\left(1,1\right)$  处计算错了。
    *    $f\left(0,0\right)=0$ 
    *    $f\left(0,1\right)=0$ 
    *    $f\left(1,0\right)=0$ 
    *    ** $f\left(1,1\right)=5$ ** （这里出错了，本该是 0）

**ZeroCheck 的任务就是发现这个“5”，并拒绝证明。**

* * *

### 第一步：验证者发起挑战

验证者无法检查所有 4 个点（在实际大电路中是数百万个点）。因此，验证者随机选择一个挑战向量  $r=\left(r_{1},r_{2}\right)$  发送给证明者 。

*   假设随机选择 ** $r=\left(3,4\right)$ **。

### 第二步：构造新多项式  $\hat{f}$ 

证明者必须构造一个新的多项式  $\hat{f}\left(X\right)$ ，公式如下 ：

$$
\hat{f}\left(X\right)=f\left(X\right)\cdot eq\left(X,r\right)
$$

这里的  $eq\left(X,r\right)$  是一个特殊的函数，定义为  $∏\left(X_{i}r_{i}+\left(1-X_{i}\right)\left(1-r_{i}\right)\right)$ 。 让我们算一下在那个**出错点  $\left(1,1\right)$ ** 上的  $eq$  值：

* 对于  $x=\left(1,1\right)$  和  $r=\left(3,4\right)$ ：
  $$
  eq\left(\left(1,1\right),\left(3,4\right)\right)=\left(1\cdot 3+0\cdot \left(1-3\right)\right)\times \left(1\cdot 4+0\cdot \left(1-4\right)\right)
  $$

  $$
  =3\times 4=12
  $$

所以，在出错点  $\left(1,1\right)$  上，新多项式  $\hat{f}$  的值是：

$$
\hat{f}\left(1,1\right)=f\left(1,1\right)\cdot eq\left(\left(1,1\right),\left(3,4\right)\right)=5\cdot 12=60
$$

对于其他点（比如  $\left(0,0\right)$ ），因为  $f\left(0,0\right)=0$ ，所以  $\hat{f}\left(0,0\right)=0\cdot eq\left(\dots \right)=0$ 。

### 第三步：归约为 SumCheck

ZeroCheck 协议要求证明者证明：**新多项式  $\hat{f}$  在超立方体上的总和为 0** 。

让我们看看**实际的总和**是多少：

$$
\sum_{x\in \{0,1\}^{2}} \hat{f}\left(x\right)=\hat{f}\left(0,0\right)+\hat{f}\left(0,1\right)+\hat{f}\left(1,0\right)+\hat{f}\left(1,1\right)
$$

$$
=0+0+0+60=60
$$

**关键点来了：** 实际总和是 **60**，而不是 **0**。

### 第四步：证明者陷入困境

现在证明者和验证者开始运行 **SumCheck 协议**。

*   **如果不撒谎**：证明者必须声称总和是 60。但 ZeroCheck 协议硬性要求声称的总和必须是 0（因为通过 Gate Identity 必须为 0 ）。如果证明者诚实地说“总和是 60”，验证者立刻拒绝，因为 60  $\ne$  0。
*   **如果撒谎**：证明者硬着头皮声称“总和是 0”。
    *   证明者发送第一轮 SumCheck 承诺。
    *   由于真实总和是 60 而证明者声称是 0，这就不仅是 ZeroCheck 的问题，而是变成了 SumCheck 协议中“声称值与实际值不符”的情况。
    *   根据 SumCheck 的安全性（Soundness），验证者会在最后一轮通过 Oracle 查询捕捉到这个谎言（概率极高）。

### 总结这个例子

1.  **原问题**： $f\left(1,1\right)=5$ ，这违反了电路约束。
2.  **随机混合**：验证者给出随机挑战  $r$ ，通过  $eq\left(x,r\right)$  将这个错误值“放大”或保留，变成了  $\hat{f}\left(1,1\right)=60$ 。
3.  **求和暴露**：因为有一个非零项 60，导致  $\hat{f}$  的总和变成了 60，不再是 0。
4.  **捕获**：因为总和不为 0，证明者无法生成一个有效的 SumCheck 证明来声称总和为 0。

这就是 ZeroCheck 如何通过**随机挑战**和**求和**，将“检查每一个点是否为 0”这一不可能的任务，转化为“检查加权总和是否为 0”这一可以通过 SumCheck 高效解决的问题。



# ZeroCheck-PIOP

**ZeroCheck PIOP** 是 HyperPlonk 协议中的一个关键构建模块，用于证明一个多变量多项式在布尔超立方体上的每一个点取值都为零。

以下是基于论文原文对 ZeroCheck PIOP 的详细解释：

### 1\. 定义与目标

ZeroCheck 的目标是证明关系  $R_{ZERO}$ 。

*   **输入：** 一个  $\mu$  个变量、度数为  $d$  的多项式  $f\in F_{\mu \left(\le d\right)}$ 。
*   **声明：** 对于布尔超立方体  $B_{\mu }=\{0,1\}^{\mu }$  上的**每一个**点  $x$ ，都有  $f\left(x\right)=0$ 。

在 HyperPlonk 中，这主要用于证明**门恒等式（Gate Identity）**，即证明电路中所有的门约束（如加法、乘法逻辑）对所有输入都成立 。

### 2\. 核心思想：归约到 SumCheck

直接检查  $2^{\mu }$  个点是非常昂贵的。ZeroCheck 使用了一个技巧，将“检查所有点为零”的问题转化为“检查一个新多项式的总和为零”的问题，从而可以利用高效的 **SumCheck 协议** 。

### 3\. 协议流程

协议包含以下步骤：

1. **随机挑战：** 验证者 (V) 采样并发送一个随机向量  $r\leftarrow F^{\mu }$  给证明者 (P) 。

2. **构造新多项式：** 证明者构造一个新的多项式  $\hat{f}\left(X\right)$ ，定义为：
   $$
   \hat{f}\left(X\right):=f\left(X\right)\cdot eq\left(X,r\right)
   $$
   其中  $eq\left(X,r\right)$  是相等函数的**多线性扩展**，定义为  $eq\left(x,y\right):=\prod_{i=1}^{\mu } \left(x_{i}y_{i}+\left(1-x_{i}\right)\left(1-y_{i}\right)\right)$ 。

3. **运行 SumCheck：** 证明者和验证者运行一个 SumCheck PIOP，证明  $\hat{f}$  在布尔超立方体上的总和为 0，即  $\left(\left(0,\left[\left[\hat{f}\right]\right]\right);\hat{f}\right)\in R_{SUM}$ 。

### 4\. 原理分析（为什么有效？）

*   **完备性（Completeness）：** 如果  $f\left(x\right)$  在超立方体上处处为 0，那么  $\hat{f}\left(x\right)=0\cdot eq\left(x,r\right)$  也恒为 0。因此  $\hat{f}$  的总和自然为 0，验证通过 。
*   **可靠性（Soundness）：**
    *   如果  $f$  在超立方体上不全为 0，我们定义辅助多项式  $g\left(Y\right):=\sum_{x\in B_{\mu }} f\left(x\right)\cdot eq\left(x,Y\right)$ 。
    *   由于  $eq\left(x,y\right)$  当且仅当  $x=y$  时为 1（在布尔域上），否则为 0，因此  $g\left(y\right)$  实际上等于  $f\left(y\right)$ 。
    *   如果  $f$  不全为 0，则  $g$  是非零多项式。根据 Schwartz-Zippel 引理，对于随机选择的  $r$ ， $\sum_{x\in B_{\mu }} \hat{f}\left(x\right)=g\left(r\right)=0$  的概率极低 。
    *   因此，如果  $f$  不满足条件，SumCheck 将以极大概率拒绝该证明。

### 5\. 性能复杂度

ZeroCheck 的效率很大程度上取决于底层的 SumCheck 协议：

*   **证明者时间：**  $O\left(d\log ^{2}d\cdot 2^{\mu }\right)$  次域运算（与 SumCheck 相同）。
*   **验证者时间：**  $O\left(\mu \right)$ 。
*   **查询复杂度：**  $\mu +1$  次（ $\mu$  次单变量 Oracle 查询，1 次多变量 Oracle 查询）。
*   **通信量：** 证明者发送的域元素数量为  $\mu$ 。

### 6\. 批处理 (Batching)

如果需要同时证明多个多项式（例如  $f$  和  $g$ ）都为零，可以通过随机线性组合  $f+\alpha g$  来进行批处理，只需运行一次 ZeroCheck 协议 。



# ZeroCheck-PIOP例子

这是一个基于 HyperPlonk 论文中 **ZeroCheck PIOP**（零值校验多项式交互预言机证明）的详细例子。

ZeroCheck PIOP 的核心任务是：证明者（Prover）持有由 Oracle（承诺） $\left[\left[f\right]\right]$  代表的多项式  $f$ ，并向验证者（Verifier）证明  $f$  在布尔超立方体  $B_{\mu }$  上的**每一个点**都为 0。

### 场景设定

*   **多项式**：假设有一个 2 变元多项式  $f\left(X_{1},X_{2}\right)=X_{1}\left(1-X_{1}\right)$ 。
    *   **验证零值**：在布尔域  $\{0,1\}$  上，
        *    $f\left(0,X_{2}\right)=0\left(1\right)=0$ 
        *    $f\left(1,X_{2}\right)=1\left(0\right)=0$ 
    *   因此，该多项式在布尔超立方体  $B_{2}=\{\left(0,0\right),\left(0,1\right),\left(1,0\right),\left(1,1\right)\}$  上确实**处处为 0**。这是一个合法的 ZeroCheck 场景。
*   **输入**：验证者拥有  $f$  的 Oracle（承诺） $\left[\left[f\right]\right]$ 。

* * *

### 第一阶段：发起挑战 (The Challenge)

1. **验证者动作**： 验证者无法逐一检查所有点，因此发送一个随机挑战向量  $r=\left(r_{1},r_{2}\right)$  给证明者。

   *   假设随机选择 ** $r=\left(3,5\right)$ **。

2. **隐式构造  $\hat{f}$ **： 此时，双方在数学上定义了一个新的“虚拟多项式”  $\hat{f}\left(X\right)$ ：
   $$
   \hat{f}\left(X\right)=f\left(X\right)\cdot eq\left(X,r\right)
   $$

   *   根据 ZeroCheck 协议，现在的目标变成了：证明  $\sum_{x\in \{0,1\}^{2}} \hat{f}\left(x\right)=0$ 。
   *   _注意：证明者不需要发送  $\hat{f}$  的 Oracle，它是虚拟存在的。_

* * *

### 第二阶段：运行 SumCheck PIOP

现在进入 SumCheck 阶段，证明  $\hat{f}$  的总和为 0。

#### 第 1 轮（处理  $X_{2}$ ）

1.  **证明者计算**： 证明者在本地计算单变量多项式  $h_{2}\left(X_{2}\right)=\sum_{x_{1}\in \{0,1\}} \hat{f}\left(x_{1},X_{2}\right)$ 。
    *   由于我们知道  $f$  在布尔点上全是 0，所以  $\hat{f}$  在布尔点上也全是 0。
    *   因此  $h_{2}\left(0\right)=0,h_{2}\left(1\right)=0$ 。
    *   在这个简单的合法例子中， $h_{2}\left(X\right)$  就是零多项式。
2.  **PIOP 动作**： 证明者发送  $h_{2}$  的 **Oracle**  $\left[\left[h_{2}\right]\right]$ （以及声明的值  $0$ ）给验证者 。
3.  **验证者检查与挑战**：
    *   验证者检查  $0=^{?} h_{2}\left(0\right)+h_{2}\left(1\right)$ 。通过。
    *   验证者发送随机挑战  $\alpha _{2}=4$ 。

#### 第 2 轮（处理  $X_{1}$ ）

1.  **证明者计算**： 证明者计算  $h_{1}\left(X_{1}\right)=\hat{f}\left(X_{1},4\right)=f\left(X_{1},4\right)\cdot eq\left(\left(X_{1},4\right),\left(3,5\right)\right)$ 。
    *   虽然  $f$  在非布尔点（如  $X_{1}=0.5$ ）可能不为 0，但在  $X_{1}\in \{0,1\}$  时， $f$  必须为 0。
    *   所以  $h_{1}\left(0\right)=0,h_{1}\left(1\right)=0$ 。
2.  **PIOP 动作**： 证明者发送  $h_{1}$  的 **Oracle**  $\left[\left[h_{1}\right]\right]$  给验证者。
3.  **验证者检查与挑战**：
    *   验证者检查  $h_{1}\left(0\right)+h_{1}\left(1\right)=^{?} h_{2}\left(\alpha _{2}\right)$ （也就是上一轮 Oracle 在 4 处的值，这里是 0）。通过。
    *   验证者发送随机挑战  $\alpha _{1}=7$ 。

* * *

### 第三阶段：最终验证 (The Verification)

现在 SumCheck 结束了，验证者需要验证  $h_{1}\left(\alpha _{1}\right)$  是否真的等于  $\hat{f}\left(\alpha _{1},\alpha _{2}\right)$ 。 当前挑战点为  $\left(\alpha _{1},\alpha _{2}\right)=\left(7,4\right)$ 。

1. **验证者查询 Oracle**： 验证者需要计算  $\hat{f}\left(7,4\right)$  的值。 根据定义  $\hat{f}\left(X\right)=f\left(X\right)\cdot eq\left(X,r\right)$ 。

   *    $eq\left(\left(7,4\right),\left(3,5\right)\right)$  是公开可计算的（验证者自己算）。
   *    $f\left(7,4\right)$  需要查询原始的 Oracle  $\left[\left[f\right]\right]$ 。
   *    验证者向 Oracle  $\left[\left[f\right]\right]$  发起查询，点为  $\left(7,4\right)$ 。

2. **证明者响应**： 证明者（或 Oracle）返回  $y=f\left(7,4\right)$  的值。

   *   让我们算一下： $f\left(7,4\right)=7\left(1-7\right)=-42$ 。
   *   _注意：虽然 ZeroCheck 保证布尔点为 0，但  $f$  在随机点  $\left(7,4\right)$  上完全可以是非零的！_

3. **一致性检查**： 验证者计算期望值：
   $$
   v_{expected}=y\cdot eq\left(\left(7,4\right),\left(3,5\right)\right)
   $$
   然后验证者查询第 2 轮发送的 Oracle  $\left[\left[h_{1}\right]\right]$  在  $\alpha _{1}=7$  处的值，检查两者是否相等：
   $$
   h_{1}\left(7\right)=^{?} v_{expected}
   $$

如果相等，则 ZeroCheck PIOP 成功。证明者成功证明了  $f$  在布尔超立方体上全为 0。

### 总结：为什么叫 PIOP？

在这个例子中体现了 **PIOP（多项式交互预言机证明）** 的特征：

1.  **对象是 Oracle**：验证者从头到尾没有看到  $f$  的完整系数表，只拥有  $\left[\left[f\right]\right]$ 。
2.  **中间也是 Oracle**：在 SumCheck 过程中，证明者发送的也是  $h_{1},h_{2}$  的 Oracle  $\left[\left[h_{i}\right]\right]$ ，而不是系数 。
3.  **虚拟多项式**：ZeroCheck 实际上是对一个“虚拟”存在的多项式  $\hat{f}$  运行 SumCheck，验证者通过组合对  $\left[\left[f\right]\right]$  的查询和本地计算来模拟对  $\left[\left[\hat{f}\right]\right]$  的查询 。