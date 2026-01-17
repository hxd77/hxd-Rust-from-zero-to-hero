#SumCheck: ZK证明的支柱

Sum-Check 协议是密码学证明中的一个基本工具，特别是在零知识证明（ZKPs）和可验证计算中。它允许验证者高效地检查证明者是否正确地求和了多项式在布尔超立方体上的评估，而无需显式计算每个评估。

## Sum-Check 的基础构建模块 

在深入 Sum-Check 协议之前，我们需要了解一些基本的数学概念。

### 1. 多元和多线性多项式

多元多项式是指具有多个变量的多项式。例如：

$$
f(x,y)=3x^3y^4+2xy^2+5y+7
$$
多元多项式通常表示为：$f(\vec{x})$

其中$\vec{x}$是变量的向量，例如$\vec{x}=(x_1,x_2,\ldots,x_n)$。其长度对应于函数中输入变量的数量。

需要注意的关键点：

+ 每个变量的次数：每个变量的最高指数。这里， `x` 的次数是 3， `y` 的次数是 4
+ 多项式的次数：具有最高组合次数的项，在之前的例子中是 7（3+4）

其他例子：

+ $x *y^2$ 的次数为 3（因为 `x` 的次数为 1， `y` 的次数为 2）
+ $x*y*z$ 的次数也为 3
+  $x*z$ 的次数为 2

**多元多项式是一种特殊的多元多项式，其中每个变量最多以 1 次出现（即没有指数大于 1）。**

例如：

$$
f(x, y, z) = x*y*z + x*z + z + 5
$$
这个多项式是多元的，并且次数为 3。**注意，多项式的次数最多是变量的数量。**

多元多项式在 Sum-Check 中尤其重要，因为协议能高效地处理它们。

```python
F = GF(17) #大小为17的有限域
R.<x, y, z> = PolynomialRing(F, order="lex") #定义了一个多项式环,字典序

P = 6*x^3*z^3*y - 6*z^2 - x - 2*y + 6
assert(P.degree() == 7) #检查最高次数的项

P = R.random_element(degree=5) #生成一个总次数最高为5的随机多项式
print("最高degree=5的多项式: ", P)

print("\nP * z = ", P * z)

print("----------")

P = R.random_element(degree=2) #最高次数为2
print("最高degree=2的多项式:", P)
print("P(1,2,3):", P(1,2,3)) #将x=1,y=2,z=3带入运算

print("----------")

#字典序排列,谁的x指数大谁排前面,相同再比较y然后z
P = 6*x*y - 6*z^2 - x^2 + y^2 - 2*y + 6*x + z + z^3 + x*z + y*z
print(P)
print(P.dict()) #键值形式6xy:(1, 1, 0): 6

print("----------")
print("逆字典序排序: ")
# 我们可以按不同的顺序对其进行排序
# 这里是degrevlex，即“次数逆字典序”
# https://doc.sagemath.org/html/en/reference/polynomial_rings/sage/rings/polynomial/
R.<x, y, z> = PolynomialRing(F, order="degrevlex")

#使用同一个多项式
# P = 6*x*y - 6*z^2 - x^2 + y^2 - 2*y + 6*x + z + z^3 + x*z + y*z #乱序
print("P = ", R(P)) #将P转换到这个新环中

print("\n另一个例子,带有多线性多项式: ")
print(5*x*y + 7*y + x*y*z + 7)
```

运行结果:

```
最高degree=5的多项式:  6*x^3*y + 7*x^3 + 6*x*y^3 - 2*x*y*z^2 + 4*y^2*z^3

P * z =  6*x^3*y*z + 7*x^3*z + 6*x*y^3*z - 2*x*y*z^3 + 4*y^2*z^4
----------
最高degree=2的多项式: 7*x^2 - 6*y*z - 2*z^2 - 5*z
P(1,2,3): 6
----------
-x^2 + 6*x*y + x*z + 6*x + y^2 + y*z - 2*y + z^3 - 6*z^2 + z
{(2, 0, 0): 16, (1, 1, 0): 6, (1, 0, 1): 1, (1, 0, 0): 6, (0, 2, 0): 1, (0, 1, 1): 1, (0, 1, 0): 15, (0, 0, 3): 1, (0, 0, 2): 11, (0, 0, 1): 1}
----------
逆字典序排序: 
P =  z^3 - x^2 + 6*x*y + y^2 + x*z + y*z - 6*z^2 + 6*x - 2*y + z

另一个例子,带有多线性多项式: 
x*y*z + 5*x*y + 7*y + 7
```



### 2.布尔超立方体

如果你在 Google 上搜索“超立方体”，你可能会看到表示高维立方体的几何形状。

我们并不真正关心几何表示（至少我是不关心，这让我比什么都更困惑……）。但为了让你知道，这里是一些 n 维超立方体的例子：

+ 0D 超立方体：一个点
+ 1D 超立方体：一条线
+ 2D 超立方体：一个正方形
+ 3D 超立方体：一个立方体
+ 四维超立方体：一个四维体

从现在起，让我们专注于真正重要的事情：超立方体作为点集的结构。

一个 n 维超立方体由所有长度为 `n` 的元组组成，其中每个元素来自一个固定的集合。

例如，如果我们选择元素 `{a, b}` ，3D 超立方体是：

```
(a, a, a), (a, a, b), (a, b, a), (a, b, b), (b, a, a), (b, a, b), (b, b, a), (b, b, b)
```

在密码学中最常见的超立方体是布尔超立方体，其中元素总是 `{0,1}` 。

例如，3D 布尔超立方体（元素为 `{0,1}` ）是：

```
(0, 0, 0), (0, 0, 1), (0, 1, 0), (0, 1, 1), (1, 0, 0), (1, 0, 1), (1, 1, 0), (1, 1, 1)
```

二维布尔超立方体：

```
(0,0), (0,1), (1,0), (1,1)
```

**我的理解：**二进制表示。

如果你想要一个 `n` 维的布尔超立方体，只需列出从 `0` 到 `2^n - 1` 的所有数的二进制表示。

示例 `n = 3` :

```python
# 十进制
[0, 1, 2, 3, 4, 5, 6, 7]

# 二进制字符串
['000', '001', '010', '011', '100', '101', '110', '111']

# 或者元组
[[0, 0, 0], [0, 0, 1], [0, 1, 0], [0, 1, 1], [1, 0, 0], [1, 0, 1], [1, 1, 0], [1, 1, 1]]
```



### 3. 连接多元多项式和布尔超立方体

多元多项式和超立方体之间的联系很简单：

多项式中的变量数量决定了评估所需的超立方体的维度。

在超立方体上评估多项式意味着将每个元组代入多项式并求和结果。

让我们举个例子：
$$
f(x, y, z) = x*y + z + 2
$$
由于 `f` 有 3 个变量，我们使用 3D 布尔超立方体：

```python
[[0, 0, 0], [0, 0, 1], [0, 1, 0], [0, 1, 1], [1, 0, 0], [1, 0, 1], [1, 1, 0], [1, 1, 1]]
```

现在，我们在每个点上评估 `f` 并求和：

```python
f(0, 0, 0) + f(0, 0, 1)
+ f(0, 1, 0) + f(0, 1, 1)
+ f(1, 0, 0) + f(1, 0, 1)
+ f(1, 1, 0) + f(1, 1, 1)
```

这个和正是 Sum-Check 协议高效验证的内容！

我们可以清晰地将其表示为：
$$
\displaystyle\sum_{x,y,z \in \{0,1\}}{f(x,y,z)}
$$
在 Python 中生成布尔超立方体的方法：

```python
cube = [(a,b,c) for a in [0,1] for b in [0,1] for c in [0,1]]
```

使用 `itertools` 的更通用版本：

```python
import itertools

dimension = 3
bool_hypercube = list(itertools.product([0, 1], repeat=dimension))
```

这给出了相同的结果，但适用于任何维度的数字！

一个 Sage 脚本，用于在布尔超立方体上评估多项式:

```python
import itertools

dimension = 3
cube = list(itertools.product([0, 1], repeat=dimension))
print(cube)

F = GF(17)
R.<a, b, c> = F[]

#多线性
P = 6*a*b*c + 5*a*b + 4*a*c + 3*a + 2*b*c + b + 7

# 我们可以检查每个变量的最高次数
assert(P.degrees() == (1,1,1))
print("P:", P)

#我们可以在超立方体上对多项式进行求值
S = sum([P(i) for i in cube]) #[(0, 0, 0), (0, 0, 1), (0, 1, 0), (0, 1, 1), (1, 0, 0), (1, 0, 1), (1, 1, 0), (1, 1, 1)]
print("超立方体上多项式的和:", S)
```

输出:

```
[(0, 0, 0), (0, 0, 1), (0, 1, 0), (0, 1, 1), (1, 0, 0), (1, 0, 1), (1, 1, 0), (1, 1, 1)]
P: 6*a*b*c + 5*a*b + 4*a*c + 2*b*c + 3*a + b + 7
超立方体上多项式的和: 15
```



### 4.什么是"多线性拓展"(MLE)

你会在论文和文档中经常看到“多线性扩展”这个术语。一开始，这让我感到困惑，**但它只是指一种接受超过布尔输入的多线性多项式的一种花哨说法。**

要得到一个函数的多线性扩展（MLE），**我们从 $\{0,1\}^n$ 个输入中插值一个多线性多项式。**

这个插值多项式最初仅在 {0,1} 输入上定义。然而，我们可以在非布尔值（如 `2` 、 `3.5` 或 `-1` ）上评估这个多项式，从而有效地将其"扩展"到布尔超立方体之外。

这个"扩展"将在 Sumcheck 协议中发挥关键作用，使证明者能够高效地减少总和。

**如果我们从一个定义在布尔超立方体上的函数 $f(x_1,...x_n)$ 开始，我们可以找到许多在布尔输入上与 `f` 一致的多变量多项式 $\mathbb{F}$ 。**

**然而，恰好存在一个多项式，它在布尔超立方体上与 `f` 相容，并且是多元线性的（每个变量的次数至多为 1）：这个唯一的多线性多项式就是我们所称的 $f$ 的多线性扩展。**

正如拉格朗日插值法能唯一确定通过给定一组点的多项式。

函数的最大似然估计通常用 `~` 表示。因此， $f(x)$ 的最大似然估计将用 $\tilde{f}(x)$ 表示。



### 5.布尔超立方体上的多元 Lagrange 插值 

在深入 Sum-Check 协议之前，我们还需要最后一个工具——多元多项式的 Lagrange 插值。这就是我们将要构建 MLEs 的方法。

Lagrange 插值允许我们从其在特定点的值中重建一个多项式。要确定一个 `d` 次多项式，我们需要至少 `d + 1` 个点。如果你不熟悉这个概念，我强烈推荐你观看这个[视频](https://www.youtube.com/watch?v=bzp_q7NDdd4&ab_channel=Dr.WillWood)。

对于一元多项式，拉格朗日插值公式为：
$$
P(x) = \sum_{i} y_i \cdot \ell_i(x)
$$
在 $\ell_i(x)$ 处是拉格朗日基多项式，它们确保多项式在指定点处取给定值。而 $y_i$ 只是该点的 `y` 值。

对于多元多项式，我们将这一思想扩展到布尔超立方体。给定 $\{0,1\}^n$ 在所有点的值，我们可以构造一个唯一的多元线性多项式来插值这些值。这一插值步骤在 Sum-Check 中至关重要，因为它允许我们将每个中间步骤表示为变量更少的新多项式（当我们进入协议时，你就会看到具体是如何做到的 ）。

在标准拉格朗日插值中，点 $p_i$ 的基本多项式在 $p_i$ 处定义为 1，在所有其他点处定义为 0，然后乘以 $y_i$ 。

对于多元插值，我们使用布尔输入（ `0` 和 `1` ），因此我们使用 `x` 和 `1 - x` 作为"选择器"来构建基多项式。

+ `x` 在 `x = 1` 时等于 `1` ，在 `0` 时等于 `x = 0` 。
+ `(1 - x)` 在 `x = 0` 时等于 `1` ，在 `0` 时等于 `x = 1` 。

这使得我们能够为 $\{0,1\}^n$ 中的每个点构建基多项式。例如，点 `(0,1,0)` 的基多项式是：
$$
(1-x)*y*(1-z)
$$
这是一个多线性插值的 SageMath 实现：

```python
#多线性插值MLE
F = GF(101)
R.<a,b> = F[] #多项式环

def multilinear_lagrange_interpolation(points, values):
    def basis_polynomial(point):
        result = R(1) #1
        result *= a if point[0] == 1 else (1 - a)
        #如果当前点的第0位是1，就乘a,如果是0乘(1-a)
        result *= b if point[1] == 1 else (1 - b)
        #同理b
        return result

    interpolation_polynomial = R(0) # 0多项式

    for i in range(len(points)):
        #把每个点的值(value)乘以对应的基函数，然后加起来
        interpolation_polynomial += F(values[i]) * basis_polynomial(points[i]) #F mod 101防止有大数

    return interpolation_polynomial

#在(0, 0), (0, 1), (1, 0), (1, 1)取特定值点(3,5,7,11)
points = [(0, 0), (0, 1), (1, 0), (1, 1)]
values = [3, 5, 7, 11]

P = multilinear_lagrange_interpolation(points, values)
print("P: ", P)

assert(P(points[0]) == values[0])
assert(P(points[1]) == values[1])
assert(P(points[2]) == values[2])
assert(P(points[3]) == values[3])
```

输出:

```
P:  2*a*b + 4*a + 2*b + 3
```



### 6.相等函数

当研究 Sum-check 时，你经常会遇到一个叫做等式函数的东西。

这个函数接受两个输入 `x` 和 `y` ，每个输入都在 $\{0,1\}^n$ 中，并返回：

+ eq(x,y) = 1, 如果 x = y
+ eq(x,y) = 0, 如果 x ≠ y

它在域 $\mathbb{F}$ 上的多线性扩展为：
$$
eq(x,y)=\displaystyle\prod_{i=0}^{n-1}x_iy_i+(1-x_i)(1-y_i)
$$
这可能看起来有点乱，但它只是编码了这样一个概念： `x` 的每一位都必须与 `y` 的对应位相匹配。如果任何一位不同，其中一个项就会变成 0，整个乘积就会计算为 0。

让我们看看在 2 位的情况下看起来是什么样子，其中 $x=(x_1,x_2)$ :

-   $eq(x_1,x_2,(0,0))=(1-x_1)(1-x_2) \quad y=(0,0)$  
-   $eq(x_1,x_2,(0,1))=(1-x_1)x_2\quad y=(0,1)$
-   $eq(x_1,x_2,(1,0))=x_1(1-x_2)\quad y=(1,0)$
-   $eq(x_1,x_2,(1,1))=x_1x_2\quad y=(1,1)$

你可以验证这些多项式仅在匹配的输入处求值为 1，而在其他任何地方都为 0。

如果你仔细观察这个表达式，你会发现它和之前拉格朗日基函数的样子完全一样，但现在写成了一个多项式。

>基多项式: $(1-x)*y*(1-z)$

假设你有一个定义在布尔超立方体 $\{0,1\}^n$ 上的多元函数 `g(x)` ，并且你想构建它在 $\mathbb{F}^n$ 上的多元扩张 $\tilde{g}(y)$ 。

你可以使用等式函数来做这个：
$$
\tilde{g}(y)=\displaystyle\sum_{x \in \{0,1\}^n}{g(x)*eq(x,y)}
$$
这之所以可行，是因为当 x=y 时，等式函数为 1，而在其他地方为 0（只有当 `x` 和 `y` 是布尔超立方体的值时）。因此在这个和中，只有当 x=y 的项有贡献，其他的项都消失。剩下的是 `g` 在 `y` 的值。

一个sage示例:

```python
#等式函数
F=GF(101)
R.<x1,x2,x3,y1,y2,y3>=F[]

cube=[(0,0,0),(0,0,1),(0,1,0),(0,1,1),(1,0,0),(1,0,1),(1,1,0),(1,1,1)]

def eq(y): #输入000-111
    beta=F(1) #1
    beta *=x1 *y[0]+(1-x1)*(1-y[0]) 
    beta *=x2 *y[1]+(1-x2)*(1-y[1])
    beta *=x3 *y[2]+(1-x3)*(1-y[2])
    return beta

def mle(values):
    return sum(values[i] * eq(c) for i,c in enumerate(cube))

a = [83,10,50,5,19,23,90,75]
P = mle(a)
print("P: ", P)

assert P.degrees() == (1, 1, 1, 0, 0, 0)

assert P(x1=cube[0][0],x2=cube[0][1],x3=cube[0][2]) == a[0] #P(000)=81
assert P(x1=cube[1][0],x2=cube[1][1],x3=cube[1][2]) == a[1]
assert P(x1=cube[2][0],x2=cube[2][1],x3=cube[2][2]) == a[2]
assert P(x1=cube[3][0],x2=cube[3][1],x3=cube[3][2]) == a[3]
assert P(x1=cube[4][0],x2=cube[4][1],x3=cube[4][2]) == a[4]
assert P(x1=cube[5][0],x2=cube[5][1],x3=cube[5][2]) == a[5]
assert P(x1=cube[6][0],x2=cube[6][1],x3=cube[6][2]) == a[6]
```

输出:

```
P:  -47*x1*x2*x3 + 3*x1*x2 - 24*x1*x3 + 28*x2*x3 + 37*x1 - 33*x2 + 28*x3 - 18
```



## Sumcheck概述

在深入细节之前，让我们先对 Sum-check 协议的作用有一个高层次的了解。

仅从名称来看，结合我们到目前为止所讨论的内容，你可能猜到它涉及对多元多项式求和并验证这个和。而你的猜测是对的！

该协议从一个多元多项式开始：
$$
f(x_1,x_2,...,x_n)
$$
然后我们在布尔超立方体的所有点上评估这个多项式：
$$
H = \displaystyle\sum_{v_1 \in \{0,1\}}\displaystyle\sum_{v_2 \in \{0,1\}}...\displaystyle\sum_{v_n \in \{0,1\}}f(v_1,v_2,...,v_n)
$$
Sum-Check 的目标是让证明者向验证者证明这个和是正确的，而无需验证者直接计算每一个评估。



### 这为什么重要

Sum-Check 协议有许多应用，我们稍后会探讨。然而，它最重要的用途之一是在零知识（ZK）证明系统中。

如果你熟悉现代证明系统的工作原理，你可能知道它们的大部分计算成本来自于多项式插值，通常使用快速傅里叶变换（FFT）。

有了 Sumcheck，验证布尔超立方体上的多项式变得显著高效。

我们不必插值高次单变量多项式，而可以处理多线性多项式，这要容易得多。

事实上，Sum-Check 协议允许验证者在 n 步（与变量数量呈线性关系）内就确信。

例如，在 PLONK 中，Sumcheck 可以替代可除性测试（或商测试）和基于配对技术的 KZG 多项式承诺，从而提高性能。这就是 HyperPlonk 所做的事情。

### 我的多项式需要多少个变量？

假设我们需要对 $2^n$ 个值进行多项式插值。

如果我们使用一元多项式，它将需要次数 $2^n-1$

相反，对于一个多元多项式，我们只需要 `n` 个变量： $log_2(2^n)$ 。

这源于二进制表示：要编码 $2^n$ 个不同的值，我们需要一个 `n` 位的二进制数。

Example:

假设我们要在 32 个点上评估一个单变量多项式。我们不必使用一个 31 次多项式，而可以工作于一个具有 5 个变量的多线性多项式（因为 $log_2(32)=5$ ）

每个评估对应一个从 0b00000 到 0b11111 的二进制字符串，覆盖所有 32 种可能的输入。



## The SumCheck

现在我们已经有了基础，对 Sumcheck 是做什么的有了大致的了解，让我们来分解它是如何实际工作的。

我将首先描述步骤，然后我们将使用 Sage 进行一个示例。

正如预期，我们从多元多项式 `P(x1,x2)` （我们以 2 个变量开始，稍后会使用更多变量）在布尔超立方体上进行评估，得到结果 `H`
$$
H=\displaystyle\sum_{x_1,x_2 \in \{0,1\}}{P(x_1,x_2)}
$$
展开这个求和，我们得到：
$$
H=P(0,0)+P(0,1)+P(1,0)+P(1,1)
$$
协议的目标是逐步减少 P 中的变量数量，每次减少一个，同时确保每一步的正确性。

### 步骤1：求和多项式

每一轮中，我们通过构造一个依赖于一个更少变量的新多项式来消去一个变量。

为此，我们定义一个单变量多项式，它每次捕获一个变量的贡献。具体来说，我们创建一个新的多项式 `g1(x)` ，其中 `x1` 仍然是一个变量，但 `x2` 被求和出来：
$$
g_1(x_1)=P(x_1,0)+P(x_1,1)
$$
这意味着我们仍然在所有 x2 的布尔值上评估 P，但我们把和表示为 x1 的函数。

这个想法是我们在减少变量的数量的同时保持整体总和。



### 步骤2：验证者检查总和

验证者已经知道 `H` ，所以他们可以通过验证以下内容来检查 $g_1(x)$ 是否计算正确：
$$
H \stackrel{?}{=} g_1(0)+g_1(1)
$$
如果这个等式成立，我们可以继续下一步。

⚠️ 但请注意：此时，证明者仍然可能在伪造东西。仅此检查并不能保证正确性。协议的安全性来自后续步骤。



### 步骤3：选择一个随机点

一旦验证者确信 $g_1(x)$ 是正确的，他们会选择一个随机点 `r1` ，并要求证明者计算 P 在 $x_1=r_1$ 处的值，从而减少变量的数量：
$$
P(r_1,x_2)
$$
这本质上固定了 $x_1$ ，只留下 $x_2$ 作为变量。

但我们为什么要采样一个随机点呢？

这是协议的核心，它依赖于Schwartz-Zippel引理。

大致来说：
>如果你有两个不同的多项式 `P(x)` 和 `Q(x)` ，它们在随机点 `a` 意外相等的概率非常小。
>
>更精确地说：如果 `P(x)` 的次数是 `p` ，而 `Q(x)` 的次数是 `q` ，它们最多只能在 $\max(p,q)$ 个点上相等。
>
>记住， `p` 和 `q` 与整个域相比会非常小。所以如果 `a` 在 $\mathbb{F}$ 中随机选取，那么 $P(a)=Q(a)$ 的概率最多是：
>$$
>\Pr[P(a)=Q(a)] \leq \frac{max(p,q)}{|\mathbb{F}|}
>$$
>这个微小的概率是协议安全的关键：如果你事先不知道 `a` ，一旦试图作弊，几乎肯定会被发现。

因此，随机选择 `r` 可以确保如果证明者试图作弊（通过伪装假的 `P` 或伪造中间多项式 `g` ），验证者会以高概率发现它。

**简而言之：随机性迫使证明者保持诚实，否则他们几乎肯定会被发现。**

记住：这个交互式协议可以通过 Fiat–Shamir 变换变成非交互式，用哈希替换验证者的随机数。这就是我们如何获得适用于现实世界的简洁、无需信任的证明。



### 下一轮：继续过程

在第二轮，我们再次应用相同的过程。我们定义一个新的多项式：
$$
g_2(x)=P(r_1,x)
$$
验证者进行检验：
$$
g_2(0)+g_2(1) \stackrel{?}{=} g_1(r_1)
$$
此时仅剩一个变量，我们最终通过计算完成：
$$
S=P(r_1,r_2)
$$
这个最终值被发送给验证者，用于最终验证步骤：
$$
P(r_1,r_2) \stackrel{?}{=} g_2(r_2)
$$
这个过程可以推广到更多变量。每一轮消除一个变量，同时确保正确性，使协议能够高效地验证大数和，而无需显式计算它们。



## Sage Example

###2个变量

让我们通过一个具有 2 个变量的 Sumcheck 协议的具体例子来讲解。我们将定义一个多线性多项式，计算在布尔超立方体上的和，并逐步展示证明者-验证者之间的交互。

```python
#2个变量的sumcheck
F=GF(101)

#定义一个两个变量的多项式环
R.<x1,x2>=F[]

#定义一个单变量多项式环
Rx.<x>=F[]

#定义一个多元多线性多项式
P=1+x2+x1+x1*x2

#计算P在布尔超立方体上的总和
H=P(0,0)+P(0,1)+P(1,0)+P(1,1)
print(f"H = {H}")
#证明者计算g1(x)=P(x,0)+P(x,1)
g1=P(x1,0)+P(x1,1)

#将g1转换为关于x的单变量多项式（仅为清晰起见）
g1=Rx(g1(x,0))
print(f"g1 = {g1}")

# 验证器检查g1是否正确求和得到H
print(f"g1(0) + g1(1) = {g1(0)+g1(1)}")
assert g1(0)+g1(1)==H

#验证者发送一个随即挑战值r1
r1=31

#证明者构造g2(x)=P(r1,x)
g2=P(r1,x2)

#将g2转换为关于x的单变量多项式（仅为清晰起见）
g2=Rx(g2(0,x))
print(f"g2 = {g2}")

#验证器检查g2是否正确地与g1(r1)相等
print(f"g2(0) + g2(1) = {g2(0) + g2(1)} ")
assert g2(0) + g2(1) == g1(r1)

r2 = 43
assert g2(r2) == P(r1, r2)
```

输出：

```
H = 9
g1 = 3*x + 3
g1(0) + g1(1) = 9
g2 = 32*x + 32
g2(0) + g2(1) = 96 
```

`g1 = Rx(g1(x, 0))` 主要是为了清晰。它将 `g1` 从多元多项式环 ( `R` ) 转换为单变量环 ( `Rx` )。这使得 `g1` 明确地成为一个单变量多项式。

在之前展示协议时，我们遗漏了一个重要步骤：检查 `g` 的次数。如果验证者忘记检查，证明者可以轻易作弊。让我们看看这是如何发生的。



## 欺骗验证者？

乍一看，在协议的第一轮中欺骗验证者似乎很容易。让我们通过一个例子来看看这是如何发生的：

```python
F = GF(101)
R.<x1,x2> = F[]
Rx.<x> = F[]

P = 15*x1*x2 + 50*x1 + 11
print(f"P = {P}")

H = P(0, 0) + P(0, 1) + P(1, 0) + P(1, 1)
print(f"H = {H}")

g1 = P(x1,0) + P(x1,1)
g1 = Rx(g1(x, 0))
print(f"g1 = {g1}")
assert g1(0) + g1(1) == H

# Now, let's try to trick the verifier
# Fake g1 using Lagrange interpolation
fake_g1 = Rx.lagrange_polynomial([(0, 1), (1, H-1)])
print(f"fake_g1 = {fake_g1}")
# The fake g1 passes the check!
assert fake_g1(0) + fake_g1(1) == H
```

在上述代码中，我们使用拉格朗日插值法创建了一个假的 `g1` ，使其通过了验证步骤。

请记住，我们想要证明 $g(0)+g(1)=H$ ，因此我使用了 `1` 和 `H-1` 的值来使这次加法通过验证，但你可以使用任何和为 H 的值对。

然而，现在还不要惊慌，由于验证者引入了随机挑战，欺骗将在下一轮中被发现。

```python
# Verifier sends a random challenge r1
r1 = 71

g2 = P(r1, x2)
g2 = Rx(g2(0, x))
print(f"g2 = {g2}")
assert g2(0) + g2(1) == g1(r1)

# Let's check if the fake g1 still works
print(f"g2(0) + g2(1) == fake_g1(r1)? {g2(0) + g2(1) == fake_g1(r1)}")
# The fake g1 will fail at this step
assert g2(0) + g2(1) != fake_g1(r1)
```

正如你所见，假的 `g1` 在第二轮验证中失败了，因为 `g2(0) + g2(1)` 的值与 `fake_g1(r1)` 不匹配。这是 Sum-Check 协议安全性的关键原因之一：一旦引入了随机性，就很难操纵计算过程。

但还有点缺失……你能看出来吗？



### g(x)次数界限

真正的问题在于，如果没有对多项式的次数进行适当的限制，攻击者可以插值出一个能够通过测试 `g2(0) + g2(1) == fake_g1(r1)` 的假多项式。

我们知道:
$$
g_2(x)=P(r_1,x)
$$
我们工作在一个小字段 $\mathbb{F}_{101}$ 中，因此可以通过对字段中的每个值计算 $P(i, 0) + P(i, 1)$ 来创建一个符合所需条件的多项式。

这里是如何用代码展示问题的：

```python
print("\nAllow g1 to be a higher degree polynomial")
# Now, let's try to trick the verifier with a higher degree polynomial
points = [(0, 1), (1, H-1)]

# we interpolate a polynomial where the condition is verified for every point in the field
points += [(i, P(i, 0) + P(i, 1)) for i in range(2, 101)]
fake_g1 = Rx.lagrange_polynomial(points)
print(f"fake_g1 degree: {fake_g1.degree()}")

# The fake g1 passes the check!
assert g2(0) + g2(1) == fake_g1(r1)
```

为了防止此类攻击，验证者必须在每一步检查多项式 `g` 的次数。

多项式 $g_i$ 的次数必须小于或等于索引 `i` 的变量的次数。如果 `g` 的次数高于预期，验证者就知道出错了。

在 Sumcheck 协议中，我们经常使用多元多项式。在这种情况下 `g` 的次数必须小于或等于 1。

这种次数界限检查最终防止了恶意尝试欺骗系统，并确保了 Sumcheck 协议的完整性。



## 使其成为ZK证明

Sumcheck 协议本身并非零知识证明，但我们可以通过一个简单的过程使其成为零知识证明。假设我们想要证明
$$
H=\displaystyle\sum_{x\in\{0,1\}^n}{P(x)}
$$

+ 选择一个与 `P` 相同次数的随机多项式 `Q`
+ 他计算 Q 在布尔超立方体上的和 $H'$
+ 验证者发送一个随机值 $\rho \in \mathbb{F}$
+ 证明者和验证者对掩码多项式运行 Sumcheck：

$$
H + \rho * H'=\displaystyle\sum_{x \in \{0,1\}^n}{P(x)+\rho*Q(x)}
$$

由于 `Q` 是随机的且被隐藏，验证者无法得知 `P(x)` 的信息，但正确性仍然得到保证，这得益于随机的 $\rho$ 。





## 将SumCheck应用于AIR

我们之前讨论过，Sum-Check 是一种避免多项式插值的好方法。

让我们一步步查看 Sum-Check 在（简化）AIR 跟踪上的过程。

### Zerocheck

让我们介绍 Zerocheck 协议，这是一个基于 Sumcheck 的协议，用于证明多项式 `P` 在整个布尔超立方体上为零。

让我们回顾一下函数 $f: \{0,1\}^n \to \mathbb{F}$ 的多线性扩展（MLE）是如何定义的：
$$
\tilde{f}(z)=\displaystyle\sum_{x \in \{0,1\}^n}{f(x)*eq(x,z)}
$$
在这个表达式中，求和是针对布尔输入 `x` 进行的，而 `z` 可以是字段中的任意一点，因为最大似然估计（MLE）是在 $\mathbb{F}$ 上定义的。

现在，如果对于所有布尔 `x` ， $f(x)=0$ 都成立，那么 $\tilde{f}(z)$ 是零多项式（这意味着它在所有 $z \in \mathbb{F}^n$ 上都评估为零）。因此，特别是必须成立：
$$
\tilde{f}(r)=0
$$
对于任何随机选择的点 `r` （由验证者选择）。

这就是我们想要证明的。但我们如何使用 Sumcheck 来证明这一点？

我们首先将 $\tilde{f}(r)$ 的定义转换为超立方体上的和，这种格式 Sumcheck 可以处理。为此，我们“反转”MLE 表达式：固定 `r` **，并将 `eq(x, r)` 视为验证者已知的函数。**

然后定义：
$$
g(x)=f(x)*eq(x,r)
$$
在协议开始时，证明者提交 $f(x)$ （使用任何多元多项式承诺方案）。

验证者知道 `r` ，可以重新计算 `eq(x, r)` ，并且对 $f(x)$ 有承诺。因此我们可以认为验证者对 $g(x)$ 有承诺，所以证明者不能再作弊了。对 `g(x)` 的任何篡改将在最后被检测到。

现在观察：
$$
\displaystyle\sum_{x \in \{0,1\}^n}{g(x)}=\displaystyle\sum_{x \in \{0,1\}^n}{f(x)*eq(x,r)}=\tilde{f}(r)
$$
所以立方体上 `g(x)` 的和等于 $\tilde{f}(r)$ ，我们期望它为零。

此时，验证者和证明者可以在 `g(x)` 上运行 Sumcheck 协议。

在这个过程中，请记住我们使用随机性逐步固定变量，我们将这种新的随机性表示为 `r'` 。

在最后一步，验证者要求对 `f` 在 `r'` 处进行评估（记住 f 已经被提交，所以证明者不能作弊），并独立计算 `eq(r', r)` 以验证 `g(x)` 是否正确形成。

如果一切检查无误，验证者便确信 `f(x)` 在整个布尔超立方体上为零，而无需评估任何 $2^n$ 值。





### AIR

现在我们已经理解了 Zerocheck，将其应用于 AIR 几乎变得微不足道。

如果你不熟悉 AIR，可以查看 [STARK 手工教程](https://dev.risczero.com/proof-system/stark-by-hand)。

步骤是：

+ 对执行轨迹的每一列在布尔超立方体上进行插值→获得多线性多项式。
+ 构建约束多项式 `P` ，设计使其在超立方体的每个点上为 $P(x) = 0$
+ 在 P 上运行 Zerocheck 来证明所有约束都得到满足。

换句话说：Zerocheck 让我们能够高效地验证我们的轨迹满足所有 AIR 约束，而无需逐个检查每个点。

这是一个描述完整过程的脚本。

```python
F = GF(101)
R.<x1,x2,y1,y2> = F[]

cube = [(0,0),(0,1),(1,0),(1,1)]

def eq(y):
    beta = F(1)
    beta *= x1 * y[0] + (1 - x1) * (1 - y[0])
    beta *= x2 * y[1] + (1 - x2) * (1 - y[1])
    return beta

def mle(values):
    return sum(values[i] * eq(c) for i,c in enumerate(cube))

a = [2,5,11,7]
b = [3,2,8,7]
c = [6,10,88,49]

# we interpolate a polynomial for the values of a, b, c
A = mle(a)
B = mle(b)
C = mle(c)

# we construct our constraint polynomial
# it is zero for all points in the cube
P = A * B - C
print("P =", P)

assert P(x1=0,x2=0) == 0
assert P(x1=0,x2=1) == 0
assert P(x1=1,x2=0) == 0
assert P(x1=1,x2=1) == 0

# we compute the MLE of P
# since P is zero for all points in the cube, the MLE is zero
P_mle = sum(P(x1=c[0],x2=c[1]) * eq([c[0],c[1]]) for c in cube)
print("P_mle =", P_mle)
assert P_mle == 0

# pick a random point in F
# we ultimately want to prove that P_mle(r) = 0
r = [29, 43] # random

# first we compute the "equality function" at r
eq_r = eq(r)
print("eq_r =", eq_r)

# then we compute the product of P and eq_r
S = P * eq_r
print("S =", S)

# S is zero for all points in the cube
assert S(x1=0,x2=0) == 0
assert S(x1=0,x2=1) == 0
assert S(x1=1,x2=0) == 0
assert S(x1=1,x2=1) == 0

# and obviously the sum of S over the cube is zero
H = sum(S(x1=c[0],x2=c[1]) for c in cube)
print("SUM:", H)
assert H == 0

print("===== SUMCHECK =====")
# randomness needed for the sumcheck
r_prime = [41, 79]

# remember that we need to bound the degree of the polynomials g1 and g2
(g1_deg_bound, g2_deg_bound, *_) = S.degrees()
print("g1_deg_bound =", g1_deg_bound)
print("g2_deg_bound =", g2_deg_bound)

# round 1
g1 = R(S(x2=0) + S(x2=1))
print("g1 =", g1)
assert g1(x1=0) + g1(x1=1) == H
assert g1.degree() <= g1_deg_bound

# round 2
g2 = R(S(x1=r_prime[0]))
print("g2 =", g2)
assert g2(x2=0) + g2(x2=1) == g1(x1=r_prime[0])
assert g2.degree() <= g2_deg_bound

print("Verifier recomputes S")
print("we receive P(x1=r_prime[0], x2=r_prime[1]) from polynomial commitment")
print("and the verifier can easily recompute eq_r himself")
S_prime = P(x1=r_prime[0], x2=r_prime[1]) * eq_r(x1=r_prime[0], x2=r_prime[1])
assert g2(x2=r_prime[1]) == S_prime
print("sumcheck passed")
```

```bash
$ sage zerocheck-air.sage
P = -35*x1^2*x2 + 7*x1*x2^2 + 45*x1^2 + 28*x1*x2 - 3*x2^2 - 45*x1 + 3*x2
P_mle = 0
eq_r = -3*x1*x2 + 30*x1 + 44*x2 - 36
S = 4*x1^3*x2^2 - 21*x1^2*x2^3 + 27*x1^3*x2 + 14*x1*x2^3 + 37*x1^3 - 27*x1^2*x2 - 28*x1*x2^2 - 31*x2^3 - 41*x1^2 + 31*x1*x2 + 38*x2^2 + 4*x1 - 7*x2
SUM: 0
===== SUMCHECK =====
g1_deg_bound = 3
g2_deg_bound = 3
g1 = 4*x1^3 - 29*x1^2 + 25*x1
g2 = -14*x2^3 - 45*x2^2 - 44*x2 - 48
Verifier recomputes S
we receive P(x1=r_prime[0], x2=r_prime[1]) from polynomial commitment
and the verifier can easily recompute eq_r himself
sumcheck passed
```

我们已经完成了！你应该能够理解我们为什么使用 Sumcheck，以及它是如何工作的。

Sumcheck 是一种既简洁又强大的协议：它将一个复杂的全局声明（“许多点的和等于 X”）转化为一个只涉及低次多项式的小型交互协议。