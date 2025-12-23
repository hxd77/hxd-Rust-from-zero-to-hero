# Permutation PIOP

**Permutation PIOP（置换校验多项式交互预言机证明）** 是 HyperPlonk 协议中用于证明\*\*电路连线（Wiring）\*\*正确性的核心机制。它建立在 **Multiset Check** 的基础之上。

简单来说，它的作用是证明：多项式  $g$  的数值列表，是多项式  $f$  的数值列表经过了 **特定的重新排列（置换  $\sigma$ ）**后得到的。

在零知识证明电路中，这就是在说：“这个加法门的输出（ $f$  中的某个值），确实被正确地连接到了那个乘法门的输入（ $g$  中的某个值）”。

* * *

### 1\. 核心定义

*   **输入**：
    *   两个多项式  $f$  和  $g$  的 Oracle。
    *   一个预定义的置换  $\sigma$ （它规定了“谁应该换到哪里”）。
*   **目标**：证明对于所有的  $x$ ，都有  $g\left(x\right)=f\left(\sigma \left(x\right)\right)$ 。
    *   这意味着  $g$  在位置  $x$  的值，应该等于  $f$  在位置  $\sigma \left(x\right)$  的值。

### 2\. 核心原理：给数值贴“位置标签”

单纯比较  $f$  和  $g$  的数值集合（Multiset Check）只能证明它们包含相同的数字，但不能证明数字**去到了正确的位置**。

为了锁定位置，Permutation PIOP 给每个数值“绑定”了一个位置标签，形成一个**元组（Tuple）**：

1.  **构造 LHS（左手边）元组**：对于  $f$ ，我们绑定“当前索引”。
    *   元组： $\left(x,f\left(x\right)\right)$ 
    *   在 HyperPlonk 中，用恒等多项式  $S_{id}\left(x\right)$  代表  $x$ 。
    *   集合： $\{\left(S_{id}\left(x\right),f\left(x\right)\right)\}_{x\in B_{\mu }}$ 
2.  **构造 RHS（右手边）元组**：对于  $g$ ，我们绑定“来源索引”（即置换  $\sigma$ ）。
    *   元组： $\left(\sigma \left(x\right),g\left(x\right)\right)$ 
    *   在 HyperPlonk 中，用置换多项式  $S_{\sigma }\left(x\right)$  代表  $\sigma \left(x\right)$ 。
    *   集合： $\{\left(S_{\sigma }\left(x\right),g\left(x\right)\right)\}_{x\in B_{\mu }}$ 

**关键逻辑**： 如果  $g$  确实是由  $f$  根据  $\sigma$  重新排列得到的，那么**上述两个集合（作为多重集）应该是完全相等的**。

*   因为  $g\left(x\right)$  的值就是  $f\left(\sigma \left(x\right)\right)$ 。
*   所以 RHS 的元组实际上是  $\left(\sigma \left(x\right),f\left(\sigma \left(x\right)\right)\right)$ 。
*   因为  $\sigma$  是一个置换（一一映射），遍历所有的  $x$  产生的  $\left(\sigma \left(x\right),f\left(\sigma \left(x\right)\right)\right)$  集合，本质上就是遍历所有的  $\left(y,f\left(y\right)\right)$ 。这与 LHS 完全一致。

### 3\. 协议流程

该协议通过**随机挑战**将元组压缩为单个数，然后运行 Multiset Check ：

1. **随机线性组合**：验证者发送随机挑战  $\beta$ 。

2. **压缩元组**：

   *   LHS 变为： $f\left(x\right)+\beta \cdot S_{id}\left(x\right)$ 
   *   RHS 变为： $g\left(x\right)+\beta \cdot S_{\sigma }\left(x\right)$ 

3. **再次随机挑战**：验证者发送  $\gamma$ （用于 Multiset Check）。

4. **ProductCheck**：运行 ProductCheck 证明以下连乘积为 1：
   $$
   \prod_{x\in B_{\mu }} \frac{f\left(x\right)+\beta \cdot S_{id}\left(x\right)+\gamma }{g\left(x\right)+\beta \cdot S_{\sigma }\left(x\right)+\gamma }=1
   $$

* * *

### 4\. 具体例子：电路连线检查

#### 场景设定

*   我们有一个极小的电路，只有 2 个门，共 4 个“端口”（变量数  $\mu =2$ ）。
*   端口索引：0, 1, 2, 3。
*   **原始数值 ( $f$ )**： $\left[10,20,30,40\right]$ 。
*   **电路连线要求 ( $\sigma$ )**：
    *   我们要求端口 1 的值连到端口 2。即  $0\to 0,1\to 2,2\to 1,3\to 3$ 。（交换了 1 和 2）。
*   **置换多项式 ( $S_{\sigma }$ )**： $\left[0,2,1,3\right]$ 。  
    *   解释：$S_{\sigma }(0)=0,S_{\sigma}(1)=2,S_{\sigma}(2)=1,S_{\sigma}(3)=3$ 。
*   **正确的新数值 ( $g$ )**： $\left[10,30,20,40\right]$ 。
    *   解释： $g\left(1\right)$  应该等于  $f\left(S_{\sigma }\left(1\right)\right)=f\left(2\right)=30$ 。

#### 步骤一：绑定位置标签（元组化）

为了证明  $g$  是合法的，我们对比元组：

*   **LHS 集合  $\{\left(Index,f\left(Index\right)\right)\}$ **：
    *    $\left(0,10\right)$ 
    *    $\left(1,20\right)$ 
    *    $\left(2,30\right)$ 
    *    $\left(3,40\right)$ 
*   **RHS 集合  $\{\left(\sigma \left(Index\right),g\left(Index\right)\right)\}$ **：
    *    $x=0$ :  $\sigma =0,g=10\to \left(0,10\right)$ 
    *    $x=1$ :  $\sigma =2,g=30\to \left(2,30\right)$  \<-- 注意这里！
    *    $x=2$ :  $\sigma =1,g=20\to \left(1,20\right)$  \<-- 注意这里！
    *    $x=3$ :  $\sigma =3,g=40\to \left(3,40\right)$ 

**对比集合**： LHS 有  $\left(1,20\right)$ ，RHS 在  $x=2$  处也有  $\left(1,20\right)$ 。 LHS 有  $\left(2,30\right)$ ，RHS 在  $x=1$  处也有  $\left(2,30\right)$ 。 **结论**：两个集合包含完全相同的元组，只是顺序变了。

#### 步骤二：压缩与校验

假设验证者给出随机挑战  $\beta =100,\gamma =5$ 。

**LHS 分子项 ( $f+100\cdot ID+5$ )**：

*    $x=0:10+0+5=15$ 
*    $x=1:20+100+5=125$ 
*    $x=2:30+200+5=235$ 
*    $x=3:40+300+5=345$ 
*    **分子总积**： $15\times 125\times 235\times 345$ 

**RHS 分母项 ( $g+100\cdot S_{\sigma }+5$ )**：

*    $x=0:10+100\left(0\right)+5=15$ 
*    $x=1:30+100\left(2\right)+5=235$  \<-- 匹配了 LHS 的  $x=2$ 
*    $x=2:20+100\left(1\right)+5=125$  \<-- 匹配了 LHS 的  $x=1$ 
*    $x=3:40+100\left(3\right)+5=345$ 
*    **分母总积**： $15\times 235\times 125\times 345$ 

**最终计算**：

$$
\frac{15\times 125\times 235\times 345}{15\times 235\times 125\times 345}=1
$$

### 总结

Permutation PIOP 巧妙地利用了**位置多项式**  $S_{\sigma }$ ，将“数值是否正确移动”的问题，转化为了“位置-数值”对（元组）的集合相等性问题。

如果在上述例子中，电路连线错了（比如  $g\left(1\right)$  还是 20），那么 RHS 在  $x=1$  处生成的项就是  $20+100\left(2\right)+5=225$ ，这在 LHS 中找不到对应项，最终的乘积就不会是 1，校验失败。