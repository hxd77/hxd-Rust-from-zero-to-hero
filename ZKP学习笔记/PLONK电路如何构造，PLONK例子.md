# PLONK电路如何构造，PLONK例子

**文章目录**

[TOC]



## 个人总结

PlonK相比起之前的zkSNARK协议来说，主要区别在于三点，

1-首先是在将电路解释为多项式的时候， [SNARK](https://blog.csdn.net/AdijeShen/article/details/122063217) 协议一般采用R1CS到QAP的做法，最后要证明的多项式形式
$$
L(x)\times R(x)-O(x)=H(x)T(x),
$$

而PLONK使用的是Gate Constraints和Copy Constraints形式，最后要证明的多项式形式

$$
Q_{L}(x) a(x)+Q_{R}(x) b(x)+Q_{O}(x) c(x)+Q_{M}(x) a(x) b(x)+Q_{C}(x)=0.
$$

个人认为这样的表达方法其实直观上更加好理解。

2-其次，zkSNARK中通过一个 $\alpha$ 来对多项式做绑定，最后验证者需要验证 $e(g^{L(x)},g^\alpha)=e(g^{L'(x)},g)$ 。而Plonk采用了Kate Polynomial Commitment的Batch版本，直接用承诺方案来代替了验证步骤，最后只需要将Commitment Open出来就行了。

3-PLONK是一个universal的SNARK协议，即它的CRS[CommonReferenceString]（文章里叫做SRS[Structured Reference String]）是可以更新并复用的，也就是说，不需要每证明一个电路就重新进行一次Trusted Setup。

## 如何理解电路

比如有一个问题，是要找到 $P(x)=x^3+x+5=35$ 的解，（解为 $x=3$ ）

现在要将其转变为电路形式，其实很简单

![](https://cdn.jsdelivr.net/gh/hxd77/BlogImage/Blog/0_1.png)

那将一个门电路分为左右两个部分，左边为 $a$ ，右边为 $b$ ，那么上述电路可以表达为以下的算式：

$$
\begin{aligned} a_0*b_0&=c_0\\ a_1*b_1&=c_1\\ a_2+b_2&=c_2\\ a_3+5&=35\to a_3=30\\ \end{aligned}
$$

上述算式主要由三种构成，分别为加法，乘法和常数赋值。这些操作都可以用gate constraints方法来表示。

但光有这四条分别表示每个门的式子是不够的，因为门之间还有联系，比如
$a_0=b_0=b_1=a_2,c_0=a_1,c_1=b_2,c_2=a_3$ 
这些需要用到Copy Constraints。

上述的 $a,b,c$ 的值为

| $wire$  | $0$  | $1$  | $2$  | $3$  |
|:---:|:---:|:---:|:---:|:---:|
| $a$  | $x$  | $x^2$  | $x$  | $x^3+x$  |
| $b$  | $x$  | $x$  | $x^3$  | $/$  |
| $c$  | $x^2$  | $x^3$  | $x^3+x$  | $/$  |

## Gate Constraints

首先看如何将上面的4条constraint

$$
\begin{aligned} a_0*b_0&=c_0\\ a_1*b_1&=c_1\\ a_2+b_2&=c_2\\ a_3&=30\\ \end{aligned}
$$

变为

$$
Q_{L}(x) a(x)+Q_{R}(x) b(x)+Q_{O}(x) c(x)+Q_{M}(x) a(x) b(x)+Q_{C}(x)=0.
$$

的形式，这是一种电路多项式形式，其中$Q_L,Q_R,Q_O,Q_C$为选择多项式，首先忽略多项式的部分，将上面的式子看为4条这种形式的式子：

$$
Q_{L_i}a_i+Q_{R_i}b_i+Q_{O_i}c_i+Q_{M_i}a_ib_i+Q_{C_i}=0
$$

$L$代表左边，$R$ 代表右边，$O$ 代表输出，$M$ 代表乘法，$C$ 代表常量，$a_i$ 代表左输入，$b_i$表示右输入，$c_i$ 代表电路输出，，$i$ 代表第$i$ 个门电路。

每个$Q_i$值都是一个常量；每个程序中每个等式中的常量（以及等式的数量）都不同。每个小写字母值都是一个变量，由用户提供：$a_i$是第$i$个门的左输入线， $b_i$是右输入线， $c_i$是第 $i$个门的输出线。对于加法门，我们设置：

$$
Q_{L_i}=0,Q_{R_i}=0,Q_{M_i}=1,Q_{O_i}=-1,Q_{C_i}=0
$$
将这些常数代入方程并化简，得到 $a_i+b_i-c_i=0$，这正是我们想要的约束条件。对于乘法门，我们设定：
$$
Q_{L_i}=0,Q_{R_i}=0,Q_{M_i}=1,Q_{O_i}=-1,Q_{C_i}=0
$$
对于恒定门控设置$a_i$到某个常数$x$ ，我们设定：
$$
Q_L=1,Q_R=0,Q_M=0,Q_O=0,Q_C=-x
$$


分别为：
$$
\begin{aligned} (0)a_0+(0)b_0+(-1)c_0+(1)a_0b_0+(0)=0\\ (0)a_1+(0)b_1+(-1)c_1+(1)a_1b_1+(0)=0\\ (1)a_2+(1)b_2+(-1)c_2+(0)a_2b_2+(0)=0\\ (1)a_3+(0)b_3+(0)c_3+(0)a_3b_3+(-30)=0 \end{aligned}
$$

可以看出这个式子是和上面的四条constraints一一对应的，将 $Q_L,Q_R,Q_M,Q_O,Q_C$ 写到一起，则变为：

$Q_L=(0,0,1,1),Q_R=(0,0,1,0),Q_O=(-1,-1,-1,0),Q_M=(1,1,0,0),Q_C=(0,0,0,-30)$ 

可以将其看做是一个多项式的点值表达形式，比如 $Q_L(x)$ 就是经过点 $(0,0),(1,0),(2,1),(3,1)$ 的一条3次多项式，使用插值法计算出来的结果为： $Q_L(x)=-\frac{1}{3}x^3+\frac{3}{2}x^2-\frac{7}{6}x
$ 。可以验证一下这条线是经过上面那些点的。**由于点值表达和系数表达可以互相转换**。也就是说
$$
Q_L(x)=(0,0,1,1)\sim Q_L(x)=-\frac{1}{3}x^3+\frac{3}{2}x^2-\frac{7}{6}
$$

这两个式子是等价的。因此下面全都使用点值表达，比较清楚。
那么就可以得到
$$
Q_L(x)=(0,0,1,1),Q_R(x)=(0,0,1,0),Q_O(x)=(-1,-1,-1,0),Q_C(x)=(0,0,0,-30)
$$


同样的，如果知道了 $x^3+x+5=35$ 的解 $x=3$ 的值，那么也可以把 $a,b,c$ 也使用多项式表示：

$$
a(x)=(3,9,3,30),b(x)=(3,3,27,/),c(x)=(9,27,30,/)\quad (根据前面的表格)
$$


**$b(x)$ 和 $c(x)$ 的最后一个点可以是任意值。**

综上所述，可以构造出了如下的式子：

$$
Q_{L}(x) a(x)+Q_{R}(x) b(x)+Q_{O}(x) c(x)+Q_{M}(x) a(x) b(x)+Q_{C}(x)=0.
$$

其中 $Q_L(x),Q_R(x),Q_M(x),Q_O(x),Q_C(x)$ 只需要知道证明的东西是什么就可以构造，也就是说可以通过ZKP中的statement构造，而 $a(x),b(x),c(x)$ 是需要知道statement的某个答案才能构造的，也就是说要掌握了witness才可以构造。

而由于上述的多项式是经过 $(0,1,2,3)$ 四个横坐标构造的，所以 $x=0,x=1,x=2,x=3$ 是上述多项式 $Q_{L}(x) a(x)+Q_{R}(x) b(x)+Q_{O}(x) c(x)+Q_{M}(x) a(x) b(x)+Q_{C}(x)=0$ 的其中4个解，那么令 $Z(x)=x(x-1)(x-2)(x-3)$ ，存在 $H(x)$ 使得：
$$
Q_{L}(x) a(x)+Q_{R}(x) b(x)+Q_{O}(x) c(x)+Q_{M}(x) a(x) b(x)+Q_{C}(x)=Z(x)H(x)
$$


这个多项式的意思是，除了(0,1,2,3)这四个横坐标要求 $Q_{L}(x) a(x)+Q_{R}(x) b(x)+Q_{O}(x) c(x)+Q_{M}(x) a(x) b(x)+Q_{C}(x)=0$ ，其他的横坐标不管，但是Verifier在做证明的时候是使用其他的某个横坐标 $s$ 来进行挑战，**虽然 $Z(s)H(s)$ 不要求等于0，但是要求和左边的表达式相等。证明了 $Z(x)$ 是左边表达式的一个因子就相当于证明了**

$$
Q_{L}(x) a(x)+Q_{R}(x) b(x)+Q_{O}(x) c(x)+Q_{M}(x) a(x) b(x)+Q_{C}(x)=0, for~i\in \{0,1,2,3\}
$$

通过上述的Constraints系统，已经可以在理想模型下构建一个证明系统了，假设 $Prover$ 和 $Verifier$ 都是诚实的，那么下面的证明系统可以成立：

现在Prover和Verifier都知道 $Q_L(x),Q_R(x),Q_M(x),Q_O(x),Q_C(x),Z(x)$ ，Prover知道 $a(x),b(x),c(x),H(x)$ 。

$$
\begin{aligned} &\quad\quad\quad\bold{Prover} &&\quad\quad\quad\bold{Verifier}\\ &&\xleftrightarrow{~~~~~~~~Q_L,Q_R,Q_M,Q_O,Q_C,Z~~~~~~}\\ &&\xleftarrow{~~~~~~~~~~~~~~~~~~~~~~~~~~~~s~~~~~~~~~~~~~~~~~~~~~~~~}&\quad random~s\\ &a=a(s),b=b(s),\\&c=c(s),h=h(s)&\xrightarrow{~~~~~~~~~~~~~~~~~~~~a,b,c,h~~~~~~~~~~~~~~~~~~~~~~}& \quad \text{Check } Q_{L}(s) a+Q_{R}(s) b+Q_{O}(s) +Q_{M}(s) a b+Q_{C}(s)=Z(s)\cdot h\\ \end{aligned}
$$



## From linear systems to polynomials
$$
y=x^3-5x^2+7x-2
$$

但我们也可以以“评估形式”查看多项式。例如，我们可以将上述内容视为 degree<4 多项式， 对 $(0,1,2,3)$ 在坐标处进行求值得 $(-2,1,0,1)$。

![img](https://raw.githubusercontent.com/hxd77/BlogImage/master/TyporaImage/20251111210143975.png)

现在这是下一步。许多相同形式的方程组可以重新解释为多项式上的单个方程。例如，假设我们有以下系统：

让我们以求值形式定义四个多项式： $L(x)$ 是在坐标处 $(0,1,2)$ 求值 $()$为 的 次数 多项式，在这些坐标处 求值为 、 到 和  至 （以这种方式直接定义多项式是可以的，您可以使用拉格朗日插值转换为系数形式）。现在，考虑以下等式：

## Copy constraints

注意到，上述的4个constraints，只是说明了第 $i$ 个 $(a_i,b_i,c_i)$ 之间的关系，没有说明相等的约束，因此这四个只是分散的式子，并不能代表 $x^3+x+5=35$ 的关系。

$$
\begin{aligned} a_0*b_0&=c_0\\ a_1*b_1&=c_1\\ a_2+b_2&=c_2\\ a_3&=30\\ \end{aligned}
$$

比如Prover可以令 $a=(1,1,1,30),b=(1,1,1,/),c=(1,1,2,/)$ ，一样可以通过上面构造的证明系统。因此要对电路的相等关系也进行描述，比如在



这条电路中， $a_0=b_0=b_1=a_2,c_0=a_1,c_1=b_2,c_2=a_3$ 有这些约束条件在。

首先来看一下在一个变量中的两个值相等如何表示。在PLONK中使用置换来表示这样的一个关系，假设存在一个置换 $\sigma(\cdot)$ 使得 $a(\sigma(i))=a'(i)$ ，记为 $a'=\sigma(a)$ ，那么只要证明 $a'(x)=a(x)$ 因为 $a'(i)=a(\sigma(i))=a(i)$ 就可以证明 $a(x)$ 中对应的置换位相等。举个例子，比如 $a_0=a_2$ ，原始的 $i=(0,1,2,3)$ 置换之后为 $\sigma(i)=(2,1,0,3)$ ，
那么假设 $a'(x)=a(x)$ 
$$
a(\sigma(i))=a'(i)\\ \begin{aligned} &i=0:&&a(0)=a'(0)=a(2)\\ &i=1:&&a(1)=a'(1)\\ &i=2:&&a(2)=a'(2)=a(0)\\ &i=3:&&a(3)=a'(3) \end{aligned}
$$

就可以得到 $a(x)$ 中 $a(0)=a(2)$ 了。

但是问题在于Verifer不可能去比较每一个点的相等关系，因为Verifier不知道 $a(x)$ 的系数，而如果一次次进行多项式的交互证明就太麻烦了，需要进行 $n$ 次，这个例子里 $n=4$ ，分别证明 $a(i)=a'(i)$ 。所以这里采用了一个range proof，一次性可以证明 $a(x)=a'(x)$ 的关系成立。

假如 $a(x)=a'(x)$ ，则 
$$
\prod_{i\in [n]}(a(i))=\prod_{i \in [n]}(a'[i])
$$


又由于 $a(\sigma(i))=a'(i)=a(i)$ ，可以得到 $\prod_{i\in [n]}a(i)+i=\prod_{i\in [n]}a'(i)+\sigma(i)$ 。

在里面再加入两个随机数 $\gamma,\beta$ （作为Challenge）
就得到了

$$
\prod_{i\in [n]}(a(i)+\beta \cdot i + \gamma)=\prod_{i \in [n]}(a'[i] + \beta \cdot \sigma(i) + \gamma)
$$

为了方便，构造一个 $P(x)$ ，使得 
$$
P(j)=\frac{\prod_{ i< j}(a(i)+\beta \cdot i + \gamma)}{\prod_{i <j }(a'[i] + \beta \cdot \sigma(i) + \gamma)}
$$


现在要证明的是 $P(x)$ 对于所有的 $x=0,1,2,3$ 时， $P(x)=1$ 。

将其转化一下，变为证明

$$
\begin{aligned} &1.&P(0)&=1,\\ &2.&P(i+1)&=P(i)\cdot \frac{a(i)+\beta \cdot i + \gamma}{a'(i)+\beta \cdot \sigma(i) + \gamma}，\\ &\text{即}&P(i+1) \cdot (a'(i)+\beta \cdot \sigma(i) + \gamma)&=P(i)\cdot (a(i)+\beta \cdot i + \gamma) \end{aligned}
$$

由于上面的部分只证明了 $a(x)$ 的复制约束，如何证明类似 $a_0=b_0$ 这样的约束？这里是比较直观的，令 $i\in[3n]=[12]$ ，对于 $a_0=a_2$ 这样的复制约束来说， $\sigma(i)=(2,1,0,3)$ ，那么对于 $a_0=b_0$ 来说，就是 $\sigma(i)=(4,1,2,3,0,5,6,7,8,9,10,11)$ 。所以在



![](https://cdn.jsdelivr.net/gh/hxd77/BlogImage/Blog/0_1.png)



示例的电路中， $a_0(i=0)=a_2(i=2)=b_0(i=4)=b_1(i=5),a_1(i=1)=c_0(i=8),b_2(i=6)=c_1(i=9),a_3(i=3)=c_2(i=10)$ (两个两个的看)，将其写为置换的形式就是：

| i: | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| $\sigma(i)$  | 2 | 8 | 4 | 10 | 5 | 0 | 9 | 7 | 1 | 6 | 3 | 11 |


然后将 $\sigma(x)$ 分为三个不同的多项式，分别为 $\sigma_a(x)=(2,8,4,10)$ , $\sigma_b(x)=(5,0,9,7)$ , $\sigma_c(x)=(1,6,3,11)$ 。

证明 $P(x)=1$ 也变为了相应的证明 $P_a(X)=1,P_b(X)=1,P_c(X)=1$ 。也可以一起做，变为证明 $P_a(X)\cdot P_b(X) \cdot P_c(X)=1$ 。

而由于引入了 $3$ 个 $P(x)$ 的证明，横坐标由 $(0,1,2,3)$ 变为了 $(0,...,11$ )。出于这个考虑，采用了单位根作为横坐标，即 $x^n=1$ 的根 $\omega$ ，他具有性质为 
$$
\omega^{n+i}=\omega^i，\omega^{n}=1，
$$
证明变为了

$$
P(x)=P_a(X)\cdot P_b(X)\cdot P_c(X) \\ \begin{aligned} &1.&P(\omega^0)&=1,\\ &2.&P(\omega^n)&=1,\\ &3.&P_a(\omega x)&=P_a(x)\cdot \frac{f_a(x)}{g_a(x)}\\ &4.&P_b(\omega x)&=P_a(x)\cdot \frac{f_b(x)}{g_c(x)}\\ &5.&P_c(\omega x)&=P_a(x)\cdot \frac{f_c(x)}{g_c(x)}\\ \end{aligned}
$$

其中 $f_a(x)=a(x)+\beta\cdot x +\gamma,g_a(x)=a'(x)+\beta \cdot \sigma_a(x)+\gamma$ 

上述式子对于 $x\in \{\omega^0,\omega^1,\omega^2,\omega^3\}$ 成立，那么可以令 $Z(x)=(x-\omega^0)(x-\omega^1)(x-\omega^2)(x-\omega^3)=x^4-1$ 。

公式3，4，5变为

$$
P_a(\omega x)\cdot g_a(x) - P_a(x)\cdot f_a(x)=H_a(x)\cdot Z(x)\\ P_b(\omega x)\cdot g_b(x) - P_b(x)\cdot f_b(x)=H_b(x)\cdot Z(x)\\ P_c(\omega x)\cdot g_c(x) - P_c(x)\cdot f_c(x)=H_c(x)\cdot Z(x)
$$

那么证明 $a(x),b(x),c(x)$ 之间的copy constraint的交互式证明为：

$$
\begin{aligned} &\quad\quad\quad\bold{Prover} &&\quad\quad\quad\bold{Verifier}\\ &&\xleftarrow{~~~~~~~~~~~~~~~~~~~~\beta,\gamma~~~~~~~~~~~~~~~~~~}&\quad random~\beta,\gamma\\ &construct~f_a(x),g_a(x) ...,H_a(x),....,P_a(x),... \\ &&\xleftarrow{~~~~~~~~~~~~~~~~~~~~~~s~~~~~~~~~~~~~~~~~~~~}&\quad random~s \\&calculate~f_a(s),...P_a(\omega s),P_a(s),...&\xrightarrow{~~~~~~~~~~~~ ~f_a(s),...~~~~~~~~~~~~~~~~~}& \quad \text{Check the validity of the above eqation 1,2,3,4,5} \\ \end{aligned}
$$

## 多项式承诺

上述证明过程还是一个交互式的证明，运用Fiat Shamir转换，可以将 $\beta,\gamma$ 变为使用哈希函数生成，而 $s$ 则是通过一个公共的参考串得到，因此很容易将上述的协议变为一个非交互式的协议。

但这样的协议还是要求双方都是诚实的，因为没有对Prover给出约束，要求他算出的所有结果都是用多项式计算出来，而不是通过其他方式伪造。因此要加入一个多项式承诺。

多项式承诺的作用在于Prover将多项式承诺 $cm_i=com(f_i,crs)$ 以及多项式计算结果 $z_i=f_i(s)$ 发送给Verifier，Verifier可以使用使用打开协议对 $z_i$ 进行验证，保证它是由 $f_i(s)$ 计算得出。

加入多项式承诺后的协议变为了

$$
\begin{aligned} &\quad\quad\quad\bold{Prover} &&\quad\quad\quad\bold{Verifier}\\ &&\xleftrightarrow{~~~~~~~~~~~~~~~~~Q_L,Q_R,Q_M,Q_O,Q_C,Z~~~~~~~~~~~~~~}\\ &&\xleftrightarrow{~~~~~~~~~~~~~~~~~~check~copy~constraint~~~~~~~~~~~~~~~}\\ &&\xrightarrow{~~~~~~~~~commitment ~ for~a(),b(),c(),h()~~~~~~~}\\ &&\xleftarrow{~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~s~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~}&\quad random~s\\ &a=a(s),b=b(s),\\&c=c(s),h=h(s)&\xrightarrow{~~~~open~commitment~a(s),b(s),c(s),h(s)~~~~}& \quad \text{Check } Q_{L}(s) a+Q_{R}(s) b+Q_{O}(s) +Q_{M}(s) a b+Q_{C}(s)=Z(s)\cdot h\\ \end{aligned}
$$

多项式承诺的方案为

---

1. ${\sf gen}(d)$ 输出 ${\rm srs}=([1]_1,[x]_1,...,[x^{d-1}]_1,[1]_2,[x]_2$ )，这里的 $[x]_1$ 指的是双线性配对中 $e(g_1,g_2)$ 中的 $g_1$ 部分，本文采用的是加法群表示，即 $[x]_1=x\cdot g_1$ 。用乘法群表示则为 $[x]_1=g_1^x$ 。

2. ${\sf com}(f,{\rm srs}): cm=[f(x)]_1$

3. ${\sf open}(cm,z,s)$ : 计算 $H(x)=\frac{f(x)-f(z)}{x-z},W=[h(x)]_1,F=cm,v=s
   $ ，发送给Verifier。
   Verifier验证 $e(F-v,[1]_2)\cdot e(-W,[x-z]_2)=1$ 是否成立。

---

这是一个单独的多项式承诺，除此之外文章中还提出了batch版本的，笔记中就省略了。

## 参考资料

1. vitalik写的 [Understanding PLONK文章](https://vitalik.ca/general/2019/09/22/plonk.html)

2. Plonk论文 PlonK : Permutations over Lagrange-bases for Oecumenical Noninteractive arguments of Knowledge

3. Plonk Tutorial (对于vitalik博客的python实现) [https://github.com/barryWhiteHat/plonk_tutorial](https://github.com/barryWhiteHat/plonk_tutorial)

​     
