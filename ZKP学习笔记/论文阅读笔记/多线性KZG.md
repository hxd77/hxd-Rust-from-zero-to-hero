# KZG

基于 HyperPlonk 这篇论文，**KZG** 指的是一种**多项式承诺方案（Polynomial Commitment Scheme, PCS）**。

它的名字来源于其发明者 **Kate, Zaverucha 和 Goldberg**（在参考文献 \[56\] 中提出）。

在零知识证明系统中，KZG 的作用是允许证明者对一个巨大的多项式生成一个很小的“指纹”（承诺），并在随后证明该多项式在特定点的值是正确的，而无需发送整个多项式。

以下是 KZG 在这篇论文中的具体含义和应用：

### 1\. 两种 KZG 变体

论文中区分了两种 KZG 方案：

*   **单变量 KZG (Univariate KZG)**：这是传统 Plonk（如 Jellyfish Plonk）使用的方案。它针对的是单变量多项式 。
*   **多线性 KZG (Multilinear KZG)**：这是 **HyperPlonk** 使用的方案。它针对的是多线性多项式（即有多个变量，但每个变量的次数最高为 1）。

### 2\. HyperPlonk 为什么要用多线性 KZG？

HyperPlonk 选择使用多线性 KZG（基于 \[65\] 的构造）是为了解决传统 Plonk 的性能瓶颈：

*   **避免 FFT**：传统 Plonk 结合单变量 KZG 需要进行大规模的快速傅里叶变换（FFT），这是生成证明的主要瓶颈 。HyperPlonk 通过使用多线性 KZG 配合 SumCheck 协议，消除了对 FFT 的需求 。
*   **支持高阶门**：在 HyperPlonk 中使用多线性 KZG，证明者所需的群指数运算（Group Exponentiations）数量仅为  $O\left(s+d\log s\right)$ （其中  $s$  是门数量， $d$  是门的度数）。相比之下，传统 Plonk 需要  $O\left(sd\right)$  次运算。这使得 HyperPlonk 能够高效支持度数非常高（如 degree-32）的自定义门 。
*   **证明大小极小**：KZG 的一大优势是生成的证明非常小。在论文的对比表中，基于 KZG 的方案证明大小仅为 0.8KB，远小于基于 FRI 或其他方案的证明大小 。

### 3\. 特性总结

根据论文中的表格和描述，KZG 方案具有以下特性：

*   **设置 (Setup)**：需要**通用可信设置 (Universal Trusted Setup)**（论文表格中标注为 "Univ."）。这意味着它需要一个一次性的初始化过程，但该过程生成的参数可以用于任何电路。
*   **依赖**：依赖于配对友好（Pairing-friendly）的椭圆曲线群（如 BLS12-381）。

## 
**多线性 KZG (Multilinear KZG)** 是一种多项式承诺方案（PCS），它是经典单变量 KZG 方案在**多变量（Multivariate）**场景下的变体。

在 HyperPlonk 论文中，多线性 KZG 被用作底层的密码学原语，用于对电路中的数据（如 Witness 和 Selector）进行“压缩”和验证 。

以下是多线性 KZG 的详细解释和具体例子。

* * *

### 1\. 什么是多线性 KZG？

**定义**： 它允许证明者对一个**多线性多项式**  $f\left(X_{1},X_{2},\dots ,X_{\mu }\right)$  生成一个极小的承诺（Commitment），并能证明该多项式在任意点  $z=\left(z_{1},\dots ,z_{\mu }\right)$  的值为  $y$ ，而无需泄露多项式的全部内容。

**与普通 KZG 的区别**：

*   **普通 KZG**：处理单变量多项式  $P\left(x\right)=a_{0}+a_{1}x+a_{2}x^{2}+\dots$ 。依赖于秘密  $\tau$  的幂次  $\tau ^{1},\tau ^{2},\dots$ 。
*   **多线性 KZG**：处理多线性多项式（每个变量的最高次为 1）。它依赖于多个秘密  $\tau _{1},\dots ,\tau _{\mu }$  的组合。

**HyperPlonk 为什么用它？** HyperPlonk 采用了基于配对（Pairing-based）的 \[65\] 方案（PST 方案）的变体。这种方案使得证明者只需进行  $O\left(2^{\mu }\right)$  次群运算即可生成承诺，且**完全不需要 FFT**，从而突破了传统 Plonk 的性能瓶颈 。

* * *

### 2\. 核心原理

#### A. 多线性多项式 (Multilinear Polynomial)

一个  $\mu$  个变量的多线性多项式可以被视为布尔超立方体  $B_{\mu }=\{0,1\}^{\mu }$  上的值的**插值**。 如果我们有一个长度为  $N=2^{\mu }$  的数据向量  $w=\left[w_{0},w_{1},\dots ,w_{N-1}\right]$ ，我们可以构造一个唯一的多线性多项式  $\tilde{f}$ ，使得它在超立方体的第  $i$  个点上的值等于  $w_{i}$ 。

#### B. 可信设置 (Trusted Setup / SRS)

多线性 KZG 需要一个一次性的可信设置（SRS）。假设有  $\mu$  个秘密随机数  $\tau =\left(\tau _{1},\dots ,\tau _{\mu }\right)$ 。 SRS 包含了拉格朗日基函数（Lagrange Basis）在这些秘密点上的值的加密形式：

$$
SRS_{i}=\left[\left[\tilde{eq}_{i}\left(\tau _{1},\dots ,\tau _{\mu }\right)\right]\right]_{1}
$$

其中  $\tilde{eq}_{i}$  是第  $i$  个拉格朗日基多项式（当输入为第  $i$  个布尔点时为 1，其他布尔点为 0）。

#### C. 承诺 (Commitment)

承诺就是一个简单的**多标量乘法 (MSM)**。承诺  $C$  是数据向量  $w$  与 SRS 向量的内积：

$$
C=\sum_{i=0}^{N-1} w_{i}\cdot SRS_{i}
$$

* * *

### 3\. 具体例子

假设我们要对一个 **2 变量 ( $\mu =2$ )** 的多项式进行承诺。

#### 场景设定

*   **变量**： $X_{1},X_{2}$ 。
*   **数据向量 (Evaluations)**：我们想承诺的数据是  $\left[3,1,4,2\right]$ 。
    *   这意味着在超立方体上的取值为：
        *    $f\left(0,0\right)=3$ 
        *    $f\left(0,1\right)=1$ 
        *    $f\left(1,0\right)=4$ 
        *    $f\left(1,1\right)=2$ 

#### 第一步：可信设置 (SRS 生成)

*   **秘密**：假设上帝选了两个秘密数  $\tau _{1}=2,\tau _{2}=5$ （实际中是未知的巨大数字）。
*   **基函数计算**（基于多线性插值公式）：
    *    $L_{00}\left(\tau \right)=\left(1-\tau _{1}\right)\left(1-\tau _{2}\right)=\left(1-2\right)\left(1-5\right)=\left(-1\right)\left(-4\right)=4$ 
    *    $L_{01}\left(\tau \right)=\left(1-\tau _{1}\right)\left(\tau _{2}\right)=\left(1-2\right)\left(5\right)=-5$ 
    *    $L_{10}\left(\tau \right)=\left(\tau _{1}\right)\left(1-\tau _{2}\right)=\left(2\right)\left(1-5\right)=-8$ 
    *    $L_{11}\left(\tau \right)=\left(\tau _{1}\right)\left(\tau _{2}\right)=\left(2\right)\left(5\right)=10$ 
*   **生成 SRS**：SRS 就是这些值在椭圆曲线群  $G_{1}$  上的映射。
    *    $SRS=\left[4\cdot G,-5\cdot G,-8\cdot G,10\cdot G\right]$ 

#### 第二步：生成承诺 (Commitment)

证明者拿到 SRS 和数据  $\left[3,1,4,2\right]$ ，计算承诺  $C$ ：

$$
C=3\cdot SRS_{0}+1\cdot SRS_{1}+4\cdot SRS_{2}+2\cdot SRS_{3}
$$

$$
C=3\left(4G\right)+1\left(-5G\right)+4\left(-8G\right)+2\left(10G\right)
$$

$$
C=\left(12-5-32+20\right)\cdot G=-5\cdot G
$$

最终发给验证者的承诺  $C$  就是椭圆曲线上的一个点（例如  $-5G$ ）。

#### 第三步：评估证明 (Evaluation)

现在验证者想知道：这个多项式在非布尔点  $z=\left(3,4\right)$  处的值是多少？

1.  **证明者计算真实值**：利用多线性插值公式计算  $y=f\left(3,4\right)$ 。
2.  **生成证明**：证明者利用 KZG 算法生成一个证明  $\pi$ （也是一个椭圆曲线点），证明“我承诺的那个多项式在  $\left(3,4\right)$  的值确实是  $y$ ”。
    *   _在 HyperPlonk 中，这一步通常结合 SumCheck 协议来批量处理多个点的评估。_

* * *

### 总结

**多线性 KZG** 就是一种将长度为  $2^{\mu }$  的数据向量，通过与预先计算好的 SRS 向量进行**点积（MSM）**，压缩成**一个群元素**的技术。

它在 HyperPlonk 中的核心价值在于：

1.  **快**：MSM 可以在 GPU 上并行加速，且不需要 FFT。
2.  **小**：无论数据多大（比如  $2^{20}$ ），承诺永远只是 48 字节（BLS12-381  $G_{1}$  点） 。


