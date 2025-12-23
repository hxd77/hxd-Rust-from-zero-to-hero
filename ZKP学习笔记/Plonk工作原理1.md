# Plonk工作原理-第一部分

PLONK是一种最先进的zk-SNARK证明系统。像Groth16这样的早期zk-SNARK具有电路特定的设置，这意味着任何新电路都需要新的可信设置。PLONK的可信设置是通用的，也就是说它可以初始化一次，然后被所有电路重复使用¹。它还是可更新的：人们可以不断添加新的随机性，直到确信该设置没有受到损害。

本文将阐述如何使用PLONK证明计算的核心思想。下图展示了使用PLONK证明计算时需要采取的步骤。

![img](https://miro.medium.com/v2/resize:fit:700/1*oVDPXiJz5JQENb46CPoV7Q.png)

我们将详细解释每个步骤：

## 算术电路

PLONK并不理解人们想要证明的程序。它首先必须被转换为一种称为算术电路的格式。算术电路是由两种类型的门构成的电路：加法门和乘法门。

假设我们想要证明我们知道方程 $P (x) = x³ + x + 5 = 35$ 的解（该解为 $3$）。我们可以将其转换为以下电路。

![image-20251104213518508](https://raw.githubusercontent.com/hxd77/BlogImage/master/TyporaImage/20251104213518538.png)

## 约束系统

接下来，我们将一个算术电路转换为一个约束系统。电路导线上的约束有两类。

### 1. 浇口约束

这些是电路中每个门内的约束条件。上述电路有四个门和以下约束条件，即具有特定格式的方程式。

![](https://raw.githubusercontent.com/hxd77/BlogImage/master/TyporaImage/20251105172758207.png)

​														图2：约束

在PLONK中，所有约束都被规范化/标准化为以下形式：

![img](https://miro.medium.com/v2/resize:fit:600/1*eOF0_6_Ff0u6PHvJcye-Fw.png)

​															图3

$a$、$b$、$c$ 分别是一个门电路的左导线、右导线和输出导线。所有的 $Q$ 都是常数。

![img](https://miro.medium.com/v2/resize:fit:700/1*p97f5ErKNgDdVYRkLhPJBg.png)

它可以被视为一个通用门，通过调整 $Qs$ 可以对其进行配置。例如，按如下方式设置 $Qs$ ，就可以将这个通用门转变为一个加法门。

![](https://raw.githubusercontent.com/hxd77/BlogImage/master/TyporaImage/20251105173604889.png)

为了了解具体情况，我们设定 $Q_L = 1，Q_R = 1，Q_O = -1，Q_M = 0，Q_C = 0$，图3就变成了
$$
a+b-c=0
$$
类似地，以下配置表示乘法门。

![img](https://miro.medium.com/v2/resize:fit:700/1*fjkkU-BkZhOh-TjADBNYHA.png)

图2归一化为以下内容：

![](https://raw.githubusercontent.com/hxd77/BlogImage/master/TyporaImage/20251105173636858.png)

​														图7：Plonk约束

我们可以将 $Qs$ 写成向量形式：
$$
Q_L=(0,0,1,1), Q_R=(0,0,1,0), Q_O=(−1,−1,−1,0), Q_M=(1,1,0,0), Q_C=(0,0,0,−30).\\
$$
$Q$ 向量被称为选择器，它们对电路结构（即程序）进行编码。

同样地，我们可以将所有的 $a$ 、$b$ 和 $c$ 收集到向量中：
$$
a = (a_0, a_1, a_2, a_3), b = (b_0, b_1, b_2, b_3), c = (c_0, c_1, c_2, c_3).
$$
这些向量被称为见证赋值，它们代表了从用户输入中推导出来的所有线路值，其中一些可能是私密的，且仅为证明者所知。





### 2. 复制约束

这些是不同门之间的约束条件，例如 $c_0=a_1 $ 。我们将在第2部分对其进行解释。



## 多项式

我们可以通过将向量的索引作为 $x$ 坐标，把向量转化为一系列点。例如，$Q_L$ 可以转化为$(0，0)、(1，0)、(2，1)$和$(3，1)$。存在一个唯一的3次多项式经过这些点。${0，1，2，3}$ 被称为求值域。 $Q_L$ 处于“求值形式”，而其“系数形式”可以通过插值得到。

![](https://raw.githubusercontent.com/hxd77/BlogImage/master/TyporaImage/20251105184018393.png)

![](https://raw.githubusercontent.com/hxd77/BlogImage/master/TyporaImage/20251105184118575.png)

同样地，我们可以将其他向量转换为多项式，并在相同的定义域上对其进行求值。我们来定义 $f(x)$ 为：

![](https://raw.githubusercontent.com/hxd77/BlogImage/master/TyporaImage/20251105184242837.png)

$f(0)$ 的计算结果为 $0$，因为

![](https://raw.githubusercontent.com/hxd77/BlogImage/master/TyporaImage/20251105184353079.png)

同样地，你可以在 $1、2、3$ 处对 $f$ 进行求值，结果都是 $0$ 。

>根据图7知

当且仅当满足以下条件时，图7中的所有约束才会得到满足：

![](https://raw.githubusercontent.com/hxd77/BlogImage/master/TyporaImage/20251105185100336.png)

​																图9

我们已将所有约束条件压缩为一个多项式 $f(x)$，其表达式如下。

![](https://raw.githubusercontent.com/hxd77/BlogImage/master/TyporaImage/20251105185358965.png)

这是因为 $0、1、2 $和 $3$ 是 $f(x)$ 的根。只要存在这样的多项式 $H(x)$，使得 $f(x)$ 能够整除 $Z(x)$ 且没有余数，图9就成立。$Z(x)$ 被称为零多项式，$H(x)$被称为商多项式。

我们可以定义另一个多项式$g(x)$:

![](https://raw.githubusercontent.com/hxd77/BlogImage/master/TyporaImage/20251105185543975.png)

​																图11

我们只需要证明 $g(x)$ 在各处都为 $0$ ，也就是说，它是一个零多项式。



## Schwartz-Zippel 定理

Schwartz-Zippel引理指出：设 $f(x)$ 是域 $F$ 上的一个非零 $d$ 次多项式，其中域 $F$ 的规模为 $n$ ，那么对于随机选取的 $s$ ，$f(s)=0 $ 的概率至多为 $d/n$ 。直观地说，这是因为 $f(x)$ 至多有 $d$ 个根。在实际情况中， $d$ 通常不超过 $1$ 亿，而 $n$ 接近 $2^{256}$ ，这意味着 $d/n=1/10^{69} $。

**这意味着，如果我们在某个随机点对多项式进行求值，且求值结果为 $0$ ，那么这极有可能表明该多项式在所有地方实际上都是零。**

一个推论是，**如果我们在某个随机点对两个多项式进行求值且它们相等，那么它们几乎必然在所有地方都相等。**





## 多项式承诺方案

![](https://raw.githubusercontent.com/hxd77/BlogImage/master/TyporaImage/20251105190223002.png)

为了证明一个多项式 $P(x)$ 等于 $0$ ，我们会使用多项式承诺方案（PCS）。多项式承诺方案可以被视为某种多项式 $P(x)$ 的“哈希”，承诺者能够通过一个证明来证实 $P$ 在某个点处的取值为特定值，而无需披露该多项式P。

PCS由证明者和验证者之间的三轮组成：

1. Commit：证明者对某个多项式 $P$ 做出承诺，并将其发送给验证者
2. Challenge：验证者想要在一个随机点 $s$ 处对 $P$ 进行评估，并将该点发送给证明者。
3. Open：证明者在 $s$ 处对 $P$ 进行求值，并将结果 $y$ 连同求值证明一同发回。验证者检查该证明，若证明有效，则得出 $P(s)=y$ 的结论。多项式方程本身可通过在一个随机值处求值来高效验证。

我们可以使用多项式承诺方案（PCS）来证明图11中的 $g(x)$为零。PLONK论文使用了基于配对的Kate承诺。其他多项式承诺方案也同样可以使用。