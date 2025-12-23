# Plonk工作原理-第二部分

在第一部分中，我们已经解释了如何将一个想要用PLONK证明的计算转换为一个中间约束系统，该系统最终会使用多项式承诺方案（PCS）来证明。我们只涵盖了一种类型的约束：门约束。在本文中，我们将介绍另一种类型：复制约束。

## 复制约束

![image-20251106210859840](https://raw.githubusercontent.com/hxd77/BlogImage/master/TyporaImage/20251106210906935.png)

在第一部分中，我们在每个门内部施加了约束，例如 $a_0 * b_0 = c_0$。然而，不同的门之间也存在约束，例如门 $0$ 的输出是门 $1$ 的左输入，因此 $c_0 = a_1$。此外，一条线路可以拆分，例如 $a_0 = b_0 = b1 = a_2 $。这些约束被称为复制约束，用于确保线路包含相同的值。





## 排列校验

让我们首先考虑单个向量（即单个多项式）内的复制约束，例如， $a_0 = a_2$。

我们定义一个排列函数 $σ$。 $σ(i)$ 是排列向量中第 $i$ 个元素的新索引。在我们的例子中，正如图2所示， $σ = (2, 1, 0, 3)$，这意味着 $a_0$ 和 $a_2$ 交换位置。

![image-20251106211146168](https://raw.githubusercontent.com/hxd77/BlogImage/master/TyporaImage/20251106211146255.png)

​																	图2

## 大产品

让我们选择两个随机数 $β$ 和 $γ$ 。将 $f$ 和 $g$ 定义为：

![img](https://miro.medium.com/v2/resize:fit:354/1*qLVq2jW2vEBTdn16xW6ghw.png)

​																图3

当且仅当以下等式成立时，排列检查才通过：

![img](https://miro.medium.com/v2/resize:fit:282/1*Zyx_2NZ1mCzKWPbUHxgHJA.png)

​																图4

左侧被称为大乘积。在我们这个具体的例子中，很容易看出当 $a_0 = a_2$ 时等式成立，因为大乘积的分子和分母中的所有项都会相互抵消。

![img](https://miro.medium.com/v2/resize:fit:700/1*gYu6UFt6anJsV8qi3jbcuw.png)

由于 $β$ 和 $γ$ 是随机的，实际上，不太可能出现总乘积为 $1$ 而置换检查失败的情况。也就是说，如果 $a0≠a2$ ，图4中的等式将不成立。



## Proof

我们提供一个非形式化证明：如果总乘积为 $1$，**那么 $σ(i) = j$ 意味着 $a_i = a_j$。**本质上，当且仅当总乘积为 $1$ 时，置换检验才能通过。

回想一下，Schwartz-Zippel 引理指出，**如果两个多项式在一个随机的评估点处相等，那么这两个多项式在各处都完全相同的概率极大。**让我们来考虑两个多项式。

![img](https://miro.medium.com/v2/resize:fit:476/1*aWLhoqhSwapxbzqKywwxGw.png)

由于它们在随机 $γ$ 时相等，我们可以将它们视为相同的，这意味着它们有相同的根。

考虑两个匹配的根：来自 $P_1$ 的第 $j$ 个和来自 $P_2$ 的第 $i$ 个。

![img](https://miro.medium.com/v2/resize:fit:466/1*fF23fqq9h0tFfhmmprMVug.png)

我们可以通过定义两个多项式再次应用上面的技巧：

![img](https://miro.medium.com/v2/resize:fit:312/1*nzFxOtNMqnRMeJyUUNNFVw.png)

由于它们在随机 $β$ 下相等，根据Schwartz-Zippel定理，我们可以认为它们是完全相等的。也就是说，当 $σ(i) = j$ 时， $a_i=a_j$ 。证毕。



## 多项式

### 单位根

在将向量转换为目标多项式时，**我们使用向量索引 ${0, 1, 2, \cdots, n-1}$作为评估 $H$ 的域**，但也可以使用任何域。在 PLONK 中，多项式插值所使用的域由单位根组成，这是因为其能带来性能提升。某个域的 $n$ 次单位根是满足 $\omega^n = 1$ 的域元素$\omega$。 $n$ 是向量的大小，在本例中为 $4$ 。 $H$ 为 $\{\omega^1,\omega^2,\omega^3,\omega^4\}$。



### 累加器

让我们定义一个向量 $P$，计算结果如下：

![img](https://miro.medium.com/v2/resize:fit:522/1*6jHU3NrKSVRqO_3Sp5jv3g.png)

它累积了总乘积，因为 $P$ 可以递归地重写：

![img](https://miro.medium.com/v2/resize:fit:418/1*SRPbhG5c5RAwYqZEJr2aOw.png)

​								图7

如果存在这样的 $P(x)$ ，我们就知道图4中的总乘积方程成立，因为

![img](https://miro.medium.com/v2/resize:fit:684/1*ps8goDT_RXsxdreQhphUrA.png)

>因为 $\omega^n=1$

图7相当于：

![img](https://miro.medium.com/v2/resize:fit:384/1*FSH4UehBB2-rVhBQKyGBKg.png)

$P(x)、f(x)$ 和 $g(x)$ 可以像之前一样，通过在域 $H=\{\omega^1,\omega^2,\omega^3,\omega^4\}$上进行插值来求得。在域 $H$ 上，满足以下多项式方程

![img](https://miro.medium.com/v2/resize:fit:372/1*S6dcEBSJKgMJX2i4z2wRqQ.png)

>$i=x$ 和 $\omega^i=x$

其中

![img](https://miro.medium.com/v2/resize:fit:408/1*Yk_U5e2lZp0bgYYXkGurdg.png)

我们可以像第一部分那样，使用多项式承诺方案来证明这一点。

## 跨向量复制约束

不同向量/多项式之间也存在复制约束，例如 $a_2 = b_0$ 和 $a_1 = c_0$ 。我们可以通过将向量 $a、b$ 和 $c$ 合并为一个单一的大向量来扩展之前的方法，该大向量的大小 $n$ 为 $12$ 。例如， $b_0$ 的索引为 $4$ ， $c_0$ 的索引为 $8$ 。我们这个简单示例的置换函数 $σ(i)$ 如下：

![img](https://miro.medium.com/v2/resize:fit:700/1*iW1THHqZj35WQsdzgKSMJA.png)

图3变为：

![img](https://miro.medium.com/v2/resize:fit:700/1*2SWvX3p_K4nQrK-R4J3-Uw.png)

剩余的步骤与在单个向量中实施复制约束的步骤相似。



## Plonk

总结一下，给定一个需要证明的程序P，我们首先将其转换为算术电路，然后转换为一系列约束，包括门约束和复制约束，这些约束会被转化为多项式。最后，我们使用多项式承诺方案（PCS）来简洁地验证多项式恒等式。

以上就是使用PLONK来证明一项计算的所有高层次思路。为了便于说明，我们省略了无数能让PLONK在实际应用中高效运行的重要优化方法。例如，可采用标准的菲亚特-沙米尔启发法使多项式承诺方案（PCS）成为非交互式的。此外，对多个多项式恒等式的检验也可以合并为一项。更多细节，你可以阅读原始论文或以下参考文献。