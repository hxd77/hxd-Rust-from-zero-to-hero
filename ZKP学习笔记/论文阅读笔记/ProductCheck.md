# ProductCheck

**ProductCheck（乘积校验）** 是 HyperPlonk 协议中的一个重要组件，用于证明多项式在布尔超立方体上的所有评估值的乘积等于某个声明的值。

它在 HyperPlonk 的架构中起着承上启下的作用：它构建在 **ZeroCheck** 之上 ，同时又是 **Multiset Check（多重集校验）** 的基础 ，最终服务于电路的连线（置换）检查。

以下是 ProductCheck 的详细解释：

### 1\. 定义与目标

ProductCheck 的目标是证明关系  $R_{PROD}$ 。

* **输入：** 两个多项式  $f_{1},f_{2}\in F_{\mu \left(\le d\right)}$ ，以及一个声明的标量  $s$ 。

* **声明：** 有理多项式  $f^{′}=f_{1}/f_{2}$  在布尔超立方体  $B_{\mu }$  上的所有评估值的乘积等于  $s$ 。即：
  $$
  \prod_{x\in B_{\mu }} \frac{f_{1}\left(x\right)}{f_{2}\left(x\right)}=s
  $$

### 2\. 核心思想：递归归约

直接计算乘积很难验证。ProductCheck 利用了 Quark 系统 的思想，通过引入一个辅助多项式  $\tilde{v}$ ，将乘积验证转化为递归的乘法关系检查，最终归约为 **ZeroCheck** 问题。

### 3\. 协议流程

协议通过以下步骤执行 ：

1.  **引入辅助 Oracle  $\tilde{v}$ **： 证明者发送一个新的多线性多项式 Oracle  $\tilde{v}\in F_{\mu +1\left(\le 1\right)}$ 。这个多项式包含了  $\mu +1$  个变量，用于“记录”乘积计算过程中的中间状态。
2.  **定义递归关系**：  $\tilde{v}$  需要满足两个性质：
    *   **初始化（叶子节点）：** 当第一个变量为 0 时，它等于我们要计算的目标多项式。即  $\tilde{v}\left(0,x\right)=f^{′}\left(x\right)=f_{1}\left(x\right)/f_{2}\left(x\right)$ 。
    *   **递归步（内部节点）：** 当第一个变量为 1 时，它等于子节点的乘积。即  $\tilde{v}\left(1,x\right)=\tilde{v}\left(x,0\right)\cdot \tilde{v}\left(x,1\right)$ 。
    *   _注：这里利用了将超立方体视为树状结构的特性，通过不断折半相乘，最终根节点就是总乘积。_
3.  **构造检查多项式**： 为了验证上述两个性质，构造两个检查多项式：
    *    $\hat{g}\left(X\right):=f_{2}\left(X\right)\cdot \tilde{v}\left(0,X\right)-f_{1}\left(X\right)$  （验证初始化，通过移项避免除法）。
    *    $\hat{f}\left(X\right):=\tilde{v}\left(1,X\right)-\tilde{v}\left(X,0\right)\cdot \tilde{v}\left(X,1\right)$  （验证递归乘积关系）。
4.  **合并为 ZeroCheck**： 利用 `merge` 函数将  $\hat{f}$  和  $\hat{g}$  合并为一个多项式  $\hat{h}=merge\left(\hat{f},\hat{g}\right)$ 。然后运行 **ZeroCheck PIOP** 来证明  $\hat{h}$  在超立方体上恒为 0 。
    *   如果 ZeroCheck 通过，说明上述两个递归关系在所有点上都成立。
5.  **边界检查**： 验证者查询 Oracle  $\tilde{v}$  在点  $\left(1,\dots ,1,0\right)$  处的值，并检查它是否等于声明的乘积  $s$ 。

### 4\. 为什么叫 ProductCheck？

虽然它的底层机制依赖于 ZeroCheck，但对外表现为：验证者只需提供  $f_{1},f_{2}$  和预期乘积  $s$ ，协议就能保证  $∏\left(f_{1}/f_{2}\right)=s$ 。

### 5\. 复杂度

*   **证明者时间：**  $O\left(d\log ^{2}d\cdot 2^{\mu }\right)$ （主要由 ZeroCheck 决定）。
*   **验证者时间：**  $O\left(\mu \right)$ 。
*   **查询复杂度：**  $\mu +2$  次（比 ZeroCheck 多一次查询，用于检查最终乘积点  $s$ ）。

### 总结

ProductCheck 通过引入一个额外的维度（ $\mu +1$  个变量）和辅助多项式，将“连乘”运算拆解为一颗二叉树的递归乘法关系，并使用 **ZeroCheck** 来批量验证这些乘法关系是否正确，从而高效地证明了整个超立方体上的乘积结果。



# ProductCheck 例子

这是一个基于 HyperPlonk 论文中 **ProductCheck**（乘积校验）的具体例子。

**ProductCheck** 的核心任务是：证明者（Prover）向验证者（Verifier）证明，一个多项式在布尔超立方体上所有取值的**连乘积**等于某个数  $s$ 。

为了高效证明这一点，ProductCheck 不会一次性算出结果，而是构建了一棵\*\*“乘积树”\*\*，并通过多项式约束来验证树的结构。

* * *

### 场景设定

*   **维度**： $\mu =2$ （2 个变量  $X_{1},X_{2}$ ），布尔超立方体有  $2^{2}=4$  个点。
*   **输入多项式**： $f_{1}$  和  $f_{2}$ 。我们需要计算  $f^{′}=f_{1}/f_{2}$  的乘积。
*   **具体数值**： 假设在 4 个点  $\left(0,0\right),\left(0,1\right),\left(1,0\right),\left(1,1\right)$  上：
    *    $f_{1}$  的值为： $\left[2,6,4,3\right]$ 
    *    $f_{2}$  的值为： $\left[1,2,2,1\right]$ 
    *    **目标多项式  $f^{′}$ ** 的值（做除法）为：
         *    $\left(0,0\right):2/1=2$ 
         *    $\left(0,1\right):6/2=3$ 
         *    $\left(1,0\right):4/2=2$ 
         *    $\left(1,1\right):3/1=3$ 
*   **目标乘积  $s$ **： $2\times 3\times 2\times 3=36$ 。

* * *

### 第一步：构建“乘积树” (The Tree)

为了证明乘积是 36，证明者在脑海中（以及数学构造中）建立一棵二叉树，层层向上相乘。

1.  **第 0 层（叶子层，即原始数据）**：
    *   数据： $\left[2,3,2,3\right]$ 
    *   对应坐标： $\left(0,0\right),\left(0,1\right),\left(1,0\right),\left(1,1\right)$ 
2.  **第 1 层（两两相乘）**：
    *   第一组： $2\times 3=6$ 
    *   第二组： $2\times 3=6$ 
    *   数据： $\left[6,6\right]$ 
3.  **第 2 层（根节点，最终乘积）**：
    *   计算： $6\times 6=36$ 
    *   数据： $\left[36\right]$ 

### 第二步：引入辅助多项式  $\tilde{v}$ 

在 HyperPlonk 中，这棵树被“压扁”放进了一个新的辅助多项式  $\tilde{v}$  中。根据论文，这个多项式比原多项式多一个变量（ $\mu +1$  维）。

证明者发送  $\tilde{v}$  的 Oracle（承诺），它包含了树上**所有节点**的信息：

*   当第一个变量为 0 时（ $\tilde{v}\left(0,\dots \right)$ ）：它代表**第 0 层（叶子）**，即原始多项式  $f^{′}$ 。
*   当第一个变量为 1 时（ $\tilde{v}\left(1,\dots \right)$ ）：它代表**树的上一层节点**（乘积结果）。

### 第三步：定义约束（如何检查树是对的？）

验证者无法看到整棵树，但他可以检查树的**局部逻辑**是否正确。这归结为两个检查：

**检查 1：叶子是否正确？（初始化）** 我们要确认树的底层确实是我们的输入  $f^{′}$ 。

*   约束公式： $\hat{g}\left(X\right):=f_{2}\left(X\right)\cdot \tilde{v}\left(0,X\right)-f_{1}\left(X\right)=0$ 
*   代入例子：比如在点  $\left(0,0\right)$ ，检查  $1\cdot \tilde{v}\left(0,0,0\right)-2=0\Rightarrow \tilde{v}\left(0,0,0\right)=2$ 。这是对的。
*   这通过 **ZeroCheck** 批量完成，确保  $\tilde{v}$  的底层等于  $f_{1}/f_{2}$ 。

**检查 2：父节点是否等于子节点之积？（递归步）** 我们要确认树上的每个节点真的等于它下面两个子节点的乘积。

*   约束公式： $\tilde{v}\left(1,X\right)=\tilde{v}\left(X,0\right)\cdot \tilde{v}\left(X,1\right)$ 。 _(注：论文中的公式利用了变量移位来模拟层级关系)_
*   **直观解释**：
    *   对于第 1 层的一个节点（比如值 6），它检查： $6=^{?} 2\times 3$ 。
    *   对于根节点（值 36），它检查： $36=^{?} 6\times 6$ 。
*   这个关系也通过 **ZeroCheck** 批量完成，确保整棵树的乘法逻辑没有造假。

### 第四步：最终验证

所有的准备工作（构建树、检查叶子、检查乘法逻辑）都完成后，验证者只需要看一眼**树顶**。

1.  验证者查询 Oracle  $\tilde{v}$  在特定点（对应根节点位置，如  $\left(1,\dots ,1,0\right)$ ）的值 。
2.  在我们的例子中，Oracle 应该返回 **36**。
3.  验证者检查  $36=^{?} s$ （声明的乘积）。

### 总结

在这个例子中，ProductCheck 没有让验证者去做  $2\times 3\times 2\times 3$  这种串行计算，而是：

1.  让证明者把中间结果（6, 6, 36）都算好并承诺在  $\tilde{v}$  中。
2.  验证者通过 **ZeroCheck** 确认  $6=2\times 3$  和  $36=6\times 6$  这些局部关系是成立的。
3.  最后确认终点是 36。

这就是 ProductCheck 如何将一个复杂的全局连乘问题，拆解为多个简单的局部乘法约束问题。



# ProductCheck PIOP和例子

**ProductCheck PIOP**（乘积校验多项式交互预言机证明）是 HyperPlonk 协议中用于证明一个有理多项式在布尔超立方体上的连乘积等于特定值的机制。

它是构建 **Multiset Check（多重集校验）** 和 **Permutation Check（置换校验）** 的基础 。

以下是 ProductCheck PIOP 的详细解释和具体例子。

* * *

### 1\. 核心定义

* **目标**：证明者（Prover）想要证明有理多项式  $f^{′}\left(x\right)=f_{1}\left(x\right)/f_{2}\left(x\right)$  在布尔超立方体  $B_{\mu }$  上的所有值的乘积等于  $s$ 。即：
  $$
  \prod_{x\in B_{\mu }} \frac{f_{1}\left(x\right)}{f_{2}\left(x\right)}=s
  $$

* **输入**：验证者拥有  $f_{1},f_{2}$  的 Oracle（承诺）以及声明的乘积  $s$ 。

### 2\. 核心原理：递归乘积树

直接计算庞大的连乘积不仅计算量大，而且难以验证。ProductCheck 使用了类似 Quark 系统的思路，引入一个辅助多项式  $\tilde{v}$  来构建一棵**递归乘积树** 。

*   **辅助多项式  $\tilde{v}$ **：这是一个  $\mu +1$  变量的多项式。你可以把它想象成一个存储了“从叶子到根节点”所有中间乘积结果的表格 。
    *   **第 0 层（叶子）**：当第一个变量为 0 时，存储原始输入值。即  $\tilde{v}\left(0,x\right)=f^{′}\left(x\right)$ 。
    *   **递归层（树节点）**：当第一个变量为 1 时，存储子节点的乘积。即  $\tilde{v}\left(1,x\right)=\tilde{v}\left(x,0\right)\cdot \tilde{v}\left(x,1\right)$ 。
    *   _注意：这里的递归公式通过变量移位（Shift）模拟了二叉树的层级结构。_

### 3\. 协议流程

1.  **发送辅助 Oracle**：证明者发送  $\tilde{v}$  的 Oracle 。
2.  **构造约束多项式**：为了验证  $\tilde{v}$  构建的“树”是正确的，定义两个约束：
    *   **初始化约束**（叶子等于输入）： $\hat{g}\left(X\right):=f_{2}\left(X\right)\cdot \tilde{v}\left(0,X\right)-f_{1}\left(X\right)$ 。
        *   _注：这里用乘法  $f_{2}\cdot \tilde{v}=f_{1}$  替代除法，避免了除零风险。_
    *   **递归约束**（父节点等于子节点积）： $\hat{f}\left(X\right):=\tilde{v}\left(1,X\right)-\tilde{v}\left(X,0\right)\cdot \tilde{v}\left(X,1\right)$ 。
3.  **运行 ZeroCheck**：将上述两个约束合并，并运行 **ZeroCheck PIOP**，证明它们在超立方体上恒为 0（即树的结构和叶子数据都是正确的） 。
4.  **检查根节点**：验证者查询  $\tilde{v}$  在点  $\left(1,\dots ,1,0\right)$  处的值，并检查其是否等于声明的乘积  $s$ 。

* * *

### 4\. 具体例子

假设我们要证明一个简单的乘积。

#### 场景设定

*   **变量数**： $\mu =2$ （输入为 00, 01, 10, 11）。
*   **输入多项式**：为了简化，假设  $f_{2}\left(x\right)=1$ （即只验证  $f_{1}$  的乘积）， $f_{1}$  在 4 个点的值为  $\left[2,3,4,2\right]$ 。
*   **目标乘积  $s$ **： $2\times 3\times 4\times 2=48$ 。

#### 步骤一：构建乘积树（证明者视角）

证明者在内心构建如下计算过程：

1.  **原始数据（Level 0）**： $2,3,4,2$ 。
2.  **第一轮两两相乘（Level 1）**：
    *    $2\times 3=6$ 
    *    $4\times 2=8$ 
    *    结果序列： $6,8$ 
3.  **第二轮相乘（Level 2，根）**：
    *    $6\times 8=48$ 

#### 步骤二：编码进  $\tilde{v}$ （辅助多项式）

证明者构造多项式  $\tilde{v}\left(Y,X_{1},X_{2}\right)$ （3 个变量）：

*   **当  $Y=0$ **（对应 Level 0）：
    *    $\tilde{v}\left(0,0,0\right)=2$ 
    *    $\tilde{v}\left(0,0,1\right)=3$ 
    *    $\tilde{v}\left(0,1,0\right)=4$ 
    *    $\tilde{v}\left(0,1,1\right)=2$ 
*   **当  $Y=1$ **（对应递归逻辑）： 根据公式  $\tilde{v}\left(1,x_{1},x_{2}\right)=\tilde{v}\left(x_{1},x_{2},0\right)\cdot \tilde{v}\left(x_{1},x_{2},1\right)$ ，这层存储了上一层折叠后的结果。
    *    $\tilde{v}\left(1,0,0\right)=\tilde{v}\left(0,0,0\right)\cdot \tilde{v}\left(0,0,1\right)=2\cdot 3=6$ 
    *    $\tilde{v}\left(1,0,1\right)=\tilde{v}\left(0,1,0\right)\cdot \tilde{v}\left(0,1,1\right)=4\cdot 2=8$ 
    *    更深层的递归（根节点 48）隐含在更高维度的坐标或最终查询点中。在论文的特定构造中，根节点的最终验证点被定义为  $\left(1,\dots ,1,0\right)$ 。

#### 步骤三：验证者校验（运行 PIOP）

验证者通过 **ZeroCheck** 批量验证以下逻辑是否成立：

1.  **检查叶子**：验证者随机抽查（通过 ZeroCheck 隐式进行），比如点  $\left(0,0\right)$ 。
    *    $\hat{g}\left(0,0\right)=1\cdot \tilde{v}\left(0,0,0\right)-f_{1}\left(0,0\right)=1\cdot 2-2=0$ 。
    *    **含义**：确认了  $\tilde{v}$  的底层确实是我们要计算的数据。
2.  **检查递归**：验证者随机抽查，比如点  $\left(0,0\right)$ 。
    *    $\hat{f}\left(0,0\right)=\tilde{v}\left(1,0,0\right)-\left(\tilde{v}\left(0,0,0\right)\cdot \tilde{v}\left(0,0,1\right)\right)$ 
    *    代入数值： $6-\left(2\cdot 3\right)=0$ 。
    *    **含义**：确认了  $\tilde{v}$  中的“6”确实是由底层的“2”和“3”相乘得来的。
3.  **检查最终结果**：
    *   验证者直接查询  $\tilde{v}$  在特定点（对应根节点）的值。
    *   证明者提供的 Oracle 返回  $48$ 。
    *   验证者确认  $48==s$ 。

### 总结

ProductCheck PIOP 并没有让验证者去执行 48 次乘法，而是让证明者把“乘法树”画出来（编码进  $\tilde{v}$ ），然后验证者通过 **ZeroCheck** 验证这棵树的**结构**（父节点是否等于子节点之积）和**基础**（叶子节点是否等于输入）是对的，最后看一眼**树顶**是否等于  $s$ 。

