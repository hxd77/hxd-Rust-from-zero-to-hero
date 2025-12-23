# 双线性映射（Bilinear Mapping)

双线性映射其实它就是这样的一个双线性映射：

$e:\mathbb{G}_1\times\mathbb{G}_2\rightarrow\mathbb{G}_T$ ，其中 $\mathbb{G}_1,\mathbb{G}_2,\mathbb{G}_T$ 是三个群， $\mathbb{G}_1,\mathbb{G}_2$ 是椭圆曲线上的两个子群，这两个群的阶均为素数$p$，而 $\mathbb{G}_T$ 是有限域中的一个乘法子群。当 $\mathbb{G}_1=\mathbb{G}_2$时，配对称为对称的。否则是非对称的。

## 预备知识

+ 熟悉椭圆曲线



## 主要性质

1. 双线性：所谓“双线性”是指，对于 $g\in\mathbb{G}_1,h\in\mathbb{G}_2，a,b\in\mathbb{Z}_p$ ，有

   $$
   e(g^a,h^b)=e(g,h)^{ab}
   $$
    ，这里将 $\mathbb{G}_1,\mathbb{G}_2$ 视为乘法群，当然也可以将其视为加法群，即 
   $$
   e(ag,bh)=e(g,h)^{ab}
   $$
   
2. 非退化性：$\exist g\in \mathbb{G}_1,\exist h\in \mathbb{G}_2$，满足 
   $$
   e(g,h)\ne 1
   $$
   或者$g_1$表示$\mathbb{G}_1$的生成元，$g_2$表示$\mathbb{G}_2$的生成元，$1_{\mathbb{G}_r}$表示 $\mathbb{G}_r$的单位元，则上式等价于
   $$
   e(g_1,g_2)\ne 1_{\mathbb{G}_r}
   $$
   
3. 可计算性：$\exist g\in \mathbb{G}_1,\exist h\in \mathbb{G}_2$。总存在概率多项式时间算法计算出 $e(g,h)$。