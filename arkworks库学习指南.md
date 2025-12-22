# ark-poly

该 crate 实现了多项式、域的 FFT 友好子集（称为“域”）以及这些域的 FFT 的特征和实现。

## Polynomials

`polynomial`模块提供了以下特性，用于定义系数形式的多项式：

+ [`Polynomial`](https://github.com/arkworks-rs/algebra/blob/master/poly/src/polynomial/mod.rs#L16) ：要求实现者支持对多项式的常见操作，例如 `Add` 、 `Sub` 、 `Zero` 、在某一点求值、次数等，并定义了与多项式的系数表示进行序列化和反序列化的方法。
+ [`DenseUVPolynomial`](https://github.com/arkworks-rs/algebra/blob/master/poly/src/polynomial/mod.rs#L43) ：指定 `Polynomial` 实际上是一*元*多项式。
+ [`DenseMVPolynomial`](https://github.com/arkworks-rs/algebra/blob/master/poly/src/polynomial/mod.rs#L59) ：指定一个 `Polynomial` 实际上是一个*多元*多项式。

该 crate 还提供了以下实现这些特性的数据结构：

+ [`univariate/DensePolynomial`](https://github.com/arkworks-rs/algebra/blob/master/poly/src/polynomial/univariate/dense.rs#L22) ：通过包含 `d + 1` 系数的列表表示 `d` 单变量多项式。此结构体实现了 [`DenseUVPolynomial`](https://github.com/arkworks-rs/algebra/blob/master/poly/src/polynomial/mod.rs#L43) 特性。
+ [`univariate/SparsePolynomial`](https://github.com/arkworks-rs/algebra/blob/master/poly/src/polynomial/univariate/sparse.rs#L15) ：通过包含所有非零单项式的列表来表示 `d` 单变量多项式。仅当多项式的大部分系数为零时才应使用此结构。此结构实现了 [`Polynomial`](https://github.com/arkworks-rs/algebra/blob/master/poly/src/polynomial/mod.rs#L16) trait（但未*实现* `DenseUVPolynomial` trait）。
+ [`multivariate/SparsePolynomial`](https://github.com/arkworks-rs/algebra/blob/master/poly/src/polynomial/multivariate/sparse.rs#L21) ：通过包含所有非零单项式的列表来表示多元多项式。

该 crate 还提供了 [`univariate/DenseOrSparsePolynomial` ](https://github.com/arkworks-rs/algebra/blob/master/poly/src/polynomial/univariate/mod.rs#L16)枚举，允许用户抽象底层单变量多项式的类型（稠密或稀疏）。

### Example

```rust
use ark_poly::{
    polynomial::multivariate::{SparsePolynomial, SparseTerm, Term},//SparsePolynomial表示多元稀疏多项式(只存在非零项)
    DenseMVPolynomial, Polynomial,
};
use ark_test_curves::bls12_381::Fq; //BLS12-381曲线的基域，一个素域元素
// 创建一个含3个变量、4项的多元多项式：
// f(x_0, x_1, x_2) = 2*x_0^3 + x_0*x_2 + x_1*x_2 + 5
let poly = SparsePolynomial::from_coefficients_vec(
    3, //变量个数x0，x1,x2
    vec![
        (Fq::from(2), SparseTerm::new(vec![(0, 3)])),//from(2)表示系数2，SparseTerm:;new(vec![(0,3)])表示一个单项式的“幂结构” x_0^3
        (Fq::from(1), SparseTerm::new(vec![(0, 1), (2, 1)])),//x_0^1,x_2^1
        (Fq::from(1), SparseTerm::new(vec![(1, 1), (2, 1)])),//x_1^1,x_2^1
        (Fq::from(5), SparseTerm::new(vec![])),//5
    ],
);
assert_eq!(poly.evaluate(&vec![Fq::from(2), Fq::from(3), Fq::from(6)]), Fq::from(51));
//2x_0^3=2*2^3=16
//x_0x_2=2*6=12
//x_1x_2=3*6=18
//常数=5
//16+12+18+5=51
```

## Evaluations

`evaluations` 模块提供数据结构来表示拉格朗日形式的单变量多项式。

+ [`univariate/Evaluations`](https://github.com/arkworks-rs/algebra/blob/master/poly/src/evaluations/univariate/mod.rs#L18) 以求值形式表示单变量多项式，可用于 FFT。

`evaluations` 模块还提供了以下特性，用于定义拉格朗日形式的多元多项式：

+ [`multivariate/multilinear/MultilinearExtension` ](https://github.com/arkworks-rs/algebra/blob/master/poly/src/evaluations/multivariate/multilinear/mod.rs#L23)指定在布尔超立方体上计算的多线性多项式。

这个 crate 提供了一些数据结构来实现这些特性。

+ [`multivariate/multilinear/DenseMultilinearExtension` ](https://github.com/arkworks-rs/algebra/blob/master/poly/src/evaluations/multivariate/multilinear/dense.rs#L17)通过对布尔超立方体进行评估的列表来表示多线性扩展。
+ [`multivariate/multilinear/SparseMultilinearExtension` ](https://github.com/arkworks-rs/algebra/blob/master/poly/src/evaluations/multivariate/multilinear/sparse.rs#L20)通过布尔超立方体上的非零评估列表来表示多线性扩展。

### Example

```rust
use ark_test_curves::bls12_381::Fq;
use ark_poly::{DenseMultilinearExtension, MultilinearExtension, SparseMultilinearExtension};
use ark_poly::{
    polynomial::multivariate::{SparsePolynomial, SparseTerm, Term},
    DenseMVPolynomial, Polynomial,
};
use ark_std::One;

// 创建一个含3个变量的多元多项式：
// f(x_0, x_1, x_2) = 2*x_0^3 + x_0*x_2 + x_1*x_2 
let f = SparsePolynomial::from_coefficients_vec(
    3,//x_0,x_1,x_2三个变量
    vec![
        (Fq::from(2), SparseTerm::new(vec![(0, 3)])),//2x_0^3
        (Fq::from(1), SparseTerm::new(vec![(0, 1), (2, 1)])),//x_0*x_2
        (Fq::from(1), SparseTerm::new(vec![(1, 1), (2, 1)])),//x_1*x_2
        (Fq::from(0), SparseTerm::new(vec![])),//0
    ],
);
// g是f的多重线性拓展，其定义基于f在布尔超立方体上的取值：
// f(0, 0, 0) = 2*0^3 + 0*0 + 0*0 = 0
// f(1, 0, 0) = 2*1^3 + 0*0 + 0*0 = 2
// ...
// f(1, 1, 1) = 2*1^3 + 1*1 + 1*1 = 4
let g: DenseMultilinearExtension<Fq> = DenseMultilinearExtension::from_evaluations_vec( //DenseMultilinearExtension<Fq>表示一个多线性多项式
    3, 
    vec![ //x_0,x_1,x_2
        Fq::from(0),//000
        Fq::from(2),//100
        Fq::from(0),//010
        Fq::from(2),//110
        Fq::from(0),//001
        Fq::from(3),//101
        Fq::from(1),//011
        Fq::from(4),//111
    ]
);//DenseMultilinearExtension=[g(0,0,0), g(1,0,0), g(0,1,0), ..., g(1,1,1)]
// 当在布尔超立方体中的任意一点进行评估时，f和g应当相等
let point_within_hypercube = &vec![Fq::from(0), Fq::from(1), Fq::from(1)];//011点
assert_eq!(f.evaluate(&point_within_hypercube), g.evaluate(&point_within_hypercube));//判断两个点是否相等

// 我们也可以通过提供非零评估的列表来定义一个最大似然估计g'(x_0, x_1, x_2)：
let g_prime: SparseMultilinearExtension<Fq> = SparseMultilinearExtension::from_evaluations(//SparseMultilinearExtension表示一个稀疏存储的多线性拓展
    3, //变量个数{0,1}^3
    &vec![ //稀疏值表,每一项是(index,value)
        (1, Fq::from(2)),//001,(x_0，x_1,x_2)=（1,0,0) g=2
        (3, Fq::from(2)),//011,(x_0，x_1,x_2)=（1,1,0) g=2
        (5, Fq::from(3)),//101,(x_0，x_1,x_2)=（1,0,1) g=3
        (6, Fq::from(1)),//110,(x_0，x_1,x_2)=（0,1,1) g=1
        (7, Fq::from(4)),//111,(x_0，x_1,x_2)=（1,1,1) g=4
    ]
);
// 在任意随机点（X0，X1，X2）处，除非g和g'是同一个函数，否则g等于g'的概率极小。
let random_point = &vec![Fq::from(123), Fq::from(456), Fq::from(789)];
assert_eq!(g_prime.evaluate(&random_point), g.evaluate(&random_point));//验证两点是否相等

```



# ark-ff

此 crate 定义了有限域特性以及遵循这些特性的实用抽象模型。一些常用椭圆曲线的具体有限域实现可以在 [`arkworks-rs/curves`](https://github.com/arkworks-rs/algebra/blob/master/curves/README.md) 下的 `arkworks-rs/curves/<your favourite curve>/src/fields/` 中找到。

这个 crate 包含两种类型的特征：

+ `Field` 特性：这些特性定义了操作字段元素的接口，例如加法、乘法、倒数、平方根等等。
+ 字段 `Config` ：指定定义相关字段的参数。对于扩展字段，它还提供字段所需的其他功能，例如涉及用于构造字段的（三次或二次）非残差的操作（ `NONRESIDUE` ）。

可能的 field traits 有:

+ [`AdditiveGroup`](https://github.com/arkworks-rs/algebra/blob/master/ff/src/fields/mod.rs#L46) 接口用于定义具有 `Scalar` 关联类型的“标量乘法”运算的加性群。这适用于素数阶域、域扩张以及密码学中使用的椭圆曲线群。
+ [`Field`](https://github.com/arkworks-rs/algebra/blob/master/ff/src/fields/mod.rs#L161) - 通用有限域的接口。
+ [`FftField`](https://github.com/arkworks-rs/algebra/blob/master/ff/src/fields/fft_friendly.rs#L2) - 公开了允许对场元素执行高效 FFT 的方法。
+ [`PrimeField`](https://github.com/arkworks-rs/algebra/blob/master/ff/src/fields/prime.rs#L27) - 元素个数为素数 `p` 的域，也称为 `Fp` 。

所实施的模型如下：

+ `Quadratic Extension`
  - [`QuadExtField`](https://github.com/arkworks-rs/algebra/blob/master/ff/src/fields/models/quadratic_extension.rs#L113) - 表示二次扩展域的结构体，在本例中包含两个基域元素
  - [`QuadExtConfig`](https://github.com/arkworks-rs/algebra/blob/master/ff/src/fields/models/quadratic_extension.rs#L29) - 定义实例化二次扩展场所需参数的特性

+ `Cubic Extension`
  - [`CubicExtField`](https://github.com/arkworks-rs/algebra/blob/master/ff/src/fields/models/cubic_extension.rs#L97) - 表示三次扩展场的结构体，包含三个基本场元素
  - [`CubicExtConfig`](https://github.com/arkworks-rs/algebra/blob/master/ff/src/fields/models/cubic_extension.rs#L28) - 定义实例化 Cubic 扩展字段所需参数的特性

上述两个模型可作为抽象概念，用于直接构造扩展场 `Fp^m` （即 `m` 等于 2 或 3），或用于创建扩展塔以获得更大的 `m` 值。后者是通过迭代应用扩展来实现的，例如在二次扩展场上进行三次扩展。

+ [`Fp2`](https://github.com/arkworks-rs/algebra/blob/master/ff/src/fields/models/fp2.rs#L99) - 直接在素数域上进行二次扩展，即 `BaseField == BasePrimeField`
+ [`Fp3`](https://github.com/arkworks-rs/algebra/blob/master/ff/src/fields/models/fp3.rs#L72) - 直接在素数域上进行三次扩展，即 `BaseField == BasePrimeField`
+ [`Fp6_2over3`](https://github.com/arkworks-rs/algebra/blob/master/ff/src/fields/models/fp6_2over3.rs#L59) - 扩展塔：三次扩展域上的二次扩展，即 `BaseField = Fp3` ，但 `BasePrimeField = Fp` 。
+ [`Fp6_3over2`](https://github.com/arkworks-rs/algebra/blob/master/ff/src/fields/models/fp6_3over2.rs#L64) - 扩展塔，与上述类似，只是塔的顺序相反：它是二次扩展域上的三次扩展，即 `BaseField = Fp2` ，但 `BasePrimeField = Fp` 。默认情况下，只有后者被导出为 `Fp6` 。
+ [`Fp12_2over3over2`](https://github.com/arkworks-rs/algebra/blob/master/ff/src/fields/models/fp12_2over3over2.rs#L66) - 扩展塔： `Fp6_3over2` 的二次扩展，即 `BaseField = Fp6` 。

## Usage

处理有限域时有两个重要的特性：[ `Field` ] 和 [ `PrimeField` ]。让我们通过例子来探讨它们。

### [`AdditiveGroup`](https://docs.rs/ark-ff/latest/ark_ff/fields/trait.AdditiveGroup.html)

[`AdditiveGroup`](https://docs.rs/ark-ff/latest/ark_ff/fields/trait.AdditiveGroup.html) 特性为具有关联标量乘法运算的加法组提供了一个通用接口。实现此特性的类型支持常见的组运算，例如加法、减法、取反，以及与关联 [`Scalar`](https://docs.rs/ark-ff/latest/ark_ff/fields/trait.AdditiveGroup.html#associatedtype.Scalar) 类型的标量乘法。

```rust
use ark_ff::AdditiveGroup; //加法群
// 在这个示例中，我们将使用与BLS12-381配对友好群相关联的字段。
use ark_test_curves::bls12_381::Fq2 as F; //Fq^2=Fq[u]/(u^2−β)
use ark_std::{One, UniformRand}; //引入trait
// `ark-std` 是一个实用工具 crate，它能让 `arkworks` 库轻松支持 `std` 和 `no_std` 工作负载，同时还会重新导出整个生态系统中应通用的有用 crate，例如 `rand`。

let mut rng = ark_std::test_rng(); //创建一个用于测试的随机数生成器(RNG)
// 让我们均匀随机地采样域元素：
let a = F::rand(&mut rng); 
let b = F::rand(&mut rng);
let c = <F as AdditiveGroup>::Scalar::rand(&mut rng); //把F看成一个加法群，随机生成一个加法群的标量 
//对不同类型，Scalar不一样 F 是一个域（比如 Fq2）,<F as AdditiveGroup>::Scalar == F
//F 是椭圆曲线点（比如 G1Projective）,Scalar = Fr

// 加法
let c = a + b;
// 减法
let d = a - b;
// 双元素
assert_eq!(c + d, a.double()); //c+d=a+a
// ... negate them ...
assert_ne!(d, -d); //d不等于-d

// 标量乘法，应该相当于是椭圆曲线上的一个点吧
let e = d * c;
```

### [`Field`](https://docs.rs/ark-ff/latest/ark_ff/fields/trait.Field.html)

[`Field`](https://docs.rs/ark-ff/latest/ark_ff/fields/trait.Field.html) 特性为任何有限域提供了一个通用接口。实现 [`Field`](https://docs.rs/ark-ff/latest/ark_ff/fields/trait.Field.html) 类型支持常见的域运算，例如加法、减法、乘法和逆运算，并且必须是 [`AdditiveGroup`](https://docs.rs/ark-ff/latest/ark_ff/fields/trait.AdditiveGroup.html) 类型。

```rust
use ark_ff::{AdditiveGroup, Field};
// 在本示例中，我们将使用与BLS12-381配对友好型群相关联的字段。
use ark_test_curves::bls12_381::Fq2 as F;
// `ark-std` 是一个工具库，它能让 `arkworks` 库轻松支持 `std` 和 `no_std` 工作负载，并且还会重新导出整个生态系统中应通用的有用库，例如 `rand`。
use ark_std::{One, UniformRand};

let mut rng = ark_std::test_rng();
// 让我们对均匀随机的域元素进行采样：
let a = F::rand(&mut rng);
let b = F::rand(&mut rng);

// 我们可以执行`AdditiveGroup` trait中的所有操作：
// 我们可以进行加法运算……
let c = a + b;
// 减法
let d = a - b;
// 双元素
assert_eq!(c + d, a.double());

//乘法
let e = c * d;
// 平方元素
assert_eq!(e, a.square() - b.square());

// 计算逆元.
assert_eq!(a.inverse().unwrap() * a, F::one()); //必须进行解包，因为`a`可能为零。(只有非零元素才有乘法逆元)
```

在某些情况下，计算域元素的平方根非常有用（例如：椭圆曲线元素的点压缩）。为了支持此功能，用户可以为其域类型实现与 `sqrt` 相关的方法。该方法已针对素数域（见下文）以及二次扩展域实现。
`sqrt` 相关的方法可以按如下方式使用：

```rust
use ark_ff::Field;
// 和之前一样，在这个示例中，我们将使用与BLS12-381配对友好群相关联的字段。
use ark_test_curves::bls12_381::Fq2 as F;
use ark_std::{One, UniformRand};

let mut rng = ark_std::test_rng();
let a = F::rand(&mut rng);

// 我们可以通过计算一个域元素的勒让德符号来判断它是否为平方数……
if a.legendre().is_qr() { //判断是否是二次剩余,如果b^2=a，则称a是二次剩余
    /*
    Zero	a = 0
	QuadraticResidue	a 是平方数
	QuadraticNonResidue	a 不是平方数
    */
    // 而且如果是这样的话，我们就能计算它的平方根。
    let b = a.sqrt().unwrap();//如果a是平方数，则存在b^2=a
    assert_eq!(b.square(), a);
} else {
    // 否则，我们可以检查平方根是否为`None`。
    assert_eq!(a.sqrt(), None);
}
```



### [`PrimeField`](https://docs.rs/ark-ff/latest/ark_ff/fields/trait.PrimeField.html)

如果字段为素数，则用户可以选择为其实现 [`PrimeField`](https://docs.rs/ark-ff/latest/ark_ff/fields/trait.PrimeField.html) 特性。这将提供对以下附加 API 的访问：

```rust
use ark_ff::{Field, PrimeField, FpConfig, BigInteger, Zero}; 
// 现在我们将使用BLS12-381 G1曲线所基于的素数域。
use ark_test_curves::bls12_381::Fq as F;//Fq是一维的，Fq^2是二次拓展域，类似于复数
use ark_std::{One, UniformRand};

let mut rng = ark_std::test_rng();
let a = F::rand(&mut rng);
//我们可以获取与`F`相关联的素数模：
let modulus = <F as PrimeField>::MODULUS;//MODULUS是素域的素数p=modulus,F=F_p
assert_eq!(a.pow(&modulus), a); //a^p=a.根据费马小定理

// 我们可以将域元素转换为范围在[0, MODULUS - 1]内的整数：
let one: num_bigint::BigUint = F::one().into(); //F::one()返回整数1对应的域元素，.into()转化为普通大整数
assert_eq!(one, num_bigint::BigUint::one());//num_bigint库里类型为BigUnit的大整数1

// 我们可以从任意字节序列构建域元素：
let n = F::from_le_bytes_mod_order(&modulus.to_bytes_le()); //modulus是素域F中的模数p，类型是BigInteger（大整数），转化为小端字节数组，返回Vec<u8>,从字节数组生成一个域元素
assert_eq!(n, F::zero()); //n=modulus mod p=p mod p=0
```
