# multilinear_kzg_mod

## 1. 模块声明

```rust
pub(crate) mod batching;
pub(crate) mod srs;
pub(crate) mod util;
```

- `pub(crate)` - 这些模块在当前crate内可见，但对外部crate不可见
- `batching` - 批量证明和验证的实现
- `srs` - Structured Reference String（结构化参考字符串），KZG方案的公共参数
- `util` - 辅助工具函数

## 2. 导入语句详解

### 内部导入

```rust
use crate::{
    pcs::{prelude::Commitment, PCSError, PolynomialCommitmentScheme, StructuredReferenceString},
    BatchProof,
};
```
- `Commitment` - 多项式承诺的数据结构
- `PCSError` - 多项式承诺方案的错误类型
- `PolynomialCommitmentScheme` - PCS的trait接口
- `StructuredReferenceString` - SRS的trait
- `BatchProof` - 批量证明结构

### 算术模块
```rust
use arithmetic::evaluate_opt;
```
- `evaluate_opt` - 优化的多项式求值函数

### Arkworks椭圆曲线库
```rust
use ark_ec::{
    pairing::Pairing,  // 配对运算trait
    scalar_mul::{
        fixed_base::FixedBase,      // 固定基点标量乘法（预计算优化）
        variable_base::VariableBaseMSM  // 可变基点多标量乘法
    },
    AffineRepr,   // 仿射坐标表示
    CurveGroup,   // 曲线群操作
};
```

**配对（Pairing）**: 一种特殊的双线性映射 `e: G1 × G2 → GT`，满足：
- `e(aP, bQ) = e(P, Q)^(ab)`
- KZG方案的核心数学基础

**MSM（Multi-Scalar Multiplication）**: 计算 `∑ sᵢ·Pᵢ`，是密码学中的性能瓶颈操作

### 有限域
```rust
use ark_ff::PrimeField;
```
- 素数域上的运算，所有多项式系数都在这个域中

### 多项式
```rust
use ark_poly::{DenseMultilinearExtension, MultilinearExtension};
```
- `DenseMultilinearExtension` - 密集存储的多线性多项式
- **多线性多项式**: `f(x₁, x₂, ..., xₙ)`，每个变量的次数最多为1
  - 例如: `f(x₁, x₂) = a + b·x₁ + c·x₂ + d·x₁·x₂`

### 序列化
```rust
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
```
- 用于承诺、证明的序列化和反序列化

### 标准库和工具
```rust
use ark_std::{
    borrow::Borrow,           // 借用trait
    end_timer, start_timer,   // 性能计时宏
    format,                   // 字符串格式化
    marker::PhantomData,      // 零大小类型标记
    rand::Rng,                // 随机数生成
    string::ToString,         // 转字符串
    sync::Arc,                // 原子引用计数（线程安全）
    vec, vec::Vec,            // 向量
    One, Zero,                // 单位元和零元trait
};
```

```rust
use std::ops::Mul;  // 乘法运算符重载
```

## 3. 子模块导入

```rust
use srs::{MultilinearProverParam, MultilinearUniversalParams, MultilinearVerifierParam};
```

**SRS参数类型**:
- `MultilinearUniversalParams` - 通用参数（setup阶段生成）
  - 包含椭圆曲线点: `[G, τG, τ²G, ..., τⁿG]`（τ是秘密值）
- `MultilinearProverParam` - 证明者参数（从通用参数派生）
- `MultilinearVerifierParam` - 验证者参数（通常更小）

```rust
use transcript::IOPTranscript;
```
- **Transcript**: Fiat-Shamir变换的交互式证明转非交互式
- 用于生成随机挑战值，保证证明的安全性

```rust
use self::batching::{batch_verify_internal, multi_open_internal};
```
- `multi_open_internal` - 批量打开多个承诺在不同点的值
- `batch_verify_internal` - 批量验证多个证明

## 4. 核心结构体

```rust
/// 关于多线性多项式的KZG多项式承诺方案。
pub struct MultilinearKzgPCS<E: Pairing> {
    #[doc(hidden)]
    phantom: PhantomData<E>,
}
```

### 设计解析

**为什么使用PhantomData？**
```rust
// 这个结构体实际上不需要存储任何数据
// 所有功能都通过关联函数（静态方法）实现
impl<E: Pairing> MultilinearKzgPCS<E> {
    pub fn setup(max_degree: usize) -> UniversalParams { ... }
    pub fn commit(params: &ProverParam, poly: &Polynomial) -> Commitment { ... }
    pub fn open(params: &ProverParam, poly: &Polynomial, point: &Point) -> Proof { ... }
    pub fn verify(params: &VerifierParam, commitment: &Commitment, point: &Point, value: F, proof: &Proof) -> bool { ... }
}
```

**PhantomData的作用**:
1. **类型安全** - 确保不同配对方案的KZG不会混用
2. **零成本抽象** - 编译后完全消失，无运行时开销
3. **泛型约束** - 让编译器知道E参数的存在

## 5. KZG多线性承诺方案工作原理

### Setup阶段

```
生成秘密值 τ (之后销毁)
计算 SRS = {G, τG, τ²G, ..., τⁿG} 在G1上
          {H, τH, τ²H, ..., τⁿH} 在G2上
```

### Commit阶段

```
给定多线性多项式 f(x₁, ..., xₙ)
承诺 C = f(τ₁, ..., τₙ)·G (使用SRS计算)
```

### Open阶段

```
证明者想证明 f(u₁, ..., uₙ) = v
生成商多项式 q(x) = (f(x) - v) / (x - u)
证明 π = q(τ)·G
```

### Verify阶段
```
验证配对等式:
e(C - vG, H) = e(π, τH - uH)
如果等式成立，则证明有效
```



## 1. Trait 实现声明

```rust
impl<E: Pairing> PolynomialCommitmentScheme<E> for MultilinearKzgPCS<E>
```

为支持配对运算的椭圆曲线 `E` 实现多项式承诺方案接口。

## 2. 关联类型定义

### 参数类型
```rust
type ProverParam = MultilinearProverParam<E>;
type VerifierParam = MultilinearVerifierParam<E>;
type SRS = MultilinearUniversalParams<E>;
```

**作用**：
- `ProverParam` - 证明者需要的参数（包含完整的 G1 群元素）
- `VerifierParam` - 验证者参数（只需要少量 G2 群元素）
- `SRS` - 可信设置生成的通用参数

**为什么分开**：验证者参数更小，便于链上存储和传输。

### 多项式相关类型
```rust
type Polynomial = Arc<DenseMultilinearExtension<E::ScalarField>>;
type Point = Vec<E::ScalarField>;
type Evaluation = E::ScalarField;
```

**详解**：
- `Arc<DenseMultilinearExtension>` - 用引用计数共享多项式数据
  - **为什么用 Arc**：多项式可能很大（2^n 个系数），避免拷贝
  - `DenseMultilinearExtension` - 存储所有 2^n 个求值点的值
  
- `Point = Vec<E::ScalarField>` - n 维点 `(x₁, x₂, ..., xₙ)`
  
- `Evaluation = E::ScalarField` - 单个域元素值

### 承诺和证明类型
```rust
type Commitment = Commitment<E>;
type Proof = MultilinearKzgProof<E>;
type BatchProof = BatchProof<E, Self>;
```

- `Commitment<E>` - 椭圆曲线上的单个点（G1 群）
- `MultilinearKzgProof<E>` - KZG 证明（多个 G1 点）
- `BatchProof` - 批量证明结构

## 3. 核心函数实现

### 3.1 测试用 SRS 生成

```rust
fn gen_srs_for_testing<R: Rng>(rng: &mut R, log_size: usize) -> Result<Self::SRS, PCSError> {
    MultilinearUniversalParams::<E>::gen_srs_for_testing(rng, log_size)
}
```

**参数**：
- `log_size` - 变量数量（不是度数！）
  - 例如：`log_size = 10` 表示支持 10 个变量，多项式有 2^10 = 1024 个系数

**警告**：仅供测试！生产环境需要真正的可信设置（MPC ceremony）。

**工作原理**：
```rust
// 伪代码
τ = random_scalar()  // 秘密值
for i in 0..2^log_size:
    srs.g1_powers[i] = τ^i * G1
    srs.g2_powers[i] = τ^i * G2
// 销毁 τ（测试时不会真销毁）
```

### 3.2 参数裁剪

```rust
fn trim(
    srs: impl Borrow<Self::SRS>,
    supported_degree: Option<usize>,      // 必须为 None
    supported_num_vars: Option<usize>,    // 必须提供
) -> Result<(Self::ProverParam, Self::VerifierParam), PCSError>
```

**目的**：从通用 SRS 中提取特定大小的参数。

**逻辑分析**：
```rust
assert!(supported_degree.is_none());  // 多线性不用 degree
```
- 单变量多项式用 `degree`（如 x^100）
- 多线性多项式用 `num_vars`（如 f(x₁, x₂, x₃)）

```rust
let supported_num_vars = match supported_num_vars {
    Some(p) => p,
    None => return Err(PCSError::InvalidParameters(
        "multilinear should receive a num_var param".to_string()
    )),
};
```

**示例**：
```rust
// SRS 支持最多 10 个变量
let srs = gen_srs_for_testing(&mut rng, 10);

// 只需要 5 个变量的参数
let (prover_param, verifier_param) = trim(&srs, None, Some(5))?;
// prover_param 包含 2^5 = 32 个 G1 点
// verifier_param 包含 5 个 G2 点
```

### 3.3 承诺生成

```rust
fn commit(
    prover_param: impl Borrow<Self::ProverParam>,
    poly: &Self::Polynomial,
) -> Result<Self::Commitment, PCSError>
```

**核心算法**：

#### 步骤 1：参数验证
```rust
if prover_param.num_vars < poly.num_vars {
    return Err(PCSError::InvalidParameters(format!(
        "MlE length ({}) exceeds param limit ({})",
        poly.num_vars, prover_param.num_vars
    )));
}
```

**检查**：多项式变量数不能超过参数支持的变量数。

#### 步骤 2：处理变量数差异
```rust
let ignored = prover_param.num_vars - poly.num_vars;
```

**例子**：
- 参数支持 10 个变量（1024 个 G1 点）
- 多项式只有 8 个变量（256 个系数）
- `ignored = 2`，使用 SRS 的后 256 个点

#### 步骤 3：准备系数
```rust
let scalars: Vec<_> = poly.to_evaluations();
```

将多项式转换为求值形式：`[f(0,0,0), f(0,0,1), f(0,1,0), ..., f(1,1,1)]`

#### 步骤 4：多标量乘法（MSM）
```rust
let commitment = E::G1::msm_unchecked(
    &prover_param.powers_of_g[ignored].evals, 
    scalars.as_slice()
).into_affine();
```

**计算**：

```
C = c₀·G + c₁·(τG) + c₂·(τ²G) + ... + c₂ₙ₋₁·(τ^(2^n-1)·G)
  = f(τ₁, τ₂, ..., τₙ)·G
```

其中 `τ₁, τ₂, ..., τₙ` 是从 `τ` 派生的值。

**性能**：
- 复杂度：O(2^n) 个标量乘法
- 对于 n=20（100 万个系数），现代硬件约需 1-2 秒

**返回**：单个椭圆曲线点（约 48 字节，G1 压缩形式）

### 3.4 打开证明

```rust
fn open(
    prover_param: impl Borrow<Self::ProverParam>,
    polynomial: &Self::Polynomial,
    point: &Self::Point,
) -> Result<(Self::Proof, Self::Evaluation), PCSError>
```

**目标**：证明 `f(u₁, u₂, ..., uₙ) = v`

**复杂度分析**（注释）：
```
总共 2^(n+1) 次标量乘法：
- n 轮迭代
- 第 i 轮：2^(n-i+1) 次 MSM
- 总和：2^n + 2^(n-1) + ... + 2 = 2^(n+1) - 2
```

**算法概述**（递归折叠）：

假设 n=3，证明 f(u₁, u₂, u₃) = v

**Round 1**：折叠第一个变量
```
f(x₁, x₂, x₃) = (1-x₁)·f_L(x₂, x₃) + x₁·f_R(x₂, x₃)
其中：
f_L(x₂, x₃) = f(0, x₂, x₃)
f_R(x₂, x₃) = f(1, x₂, x₃)

生成承诺：
L₁ = Commit(f_L)
R₁ = Commit(f_R)

挑战值：r₁ = Hash(L₁, R₁, transcript)

折叠：
f'(x₂, x₃) = (1-r₁)·f_L + r₁·f_R
u'₂ = u₂, u'₃ = u₃
```

**Round 2**：继续折叠...

**最终**：得到常数多项式，验证配对等式。

### 3.5 批量打开

```rust
fn multi_open(
    prover_param: impl Borrow<Self::ProverParam>,
    polynomials: &[Self::Polynomial],
    points: &[Self::Point],
    evals: &[Self::Evaluation],
    transcript: &mut IOPTranscript<E::ScalarField>,
) -> Result<BatchProof<E, Self>, PCSError>
```

**场景**：同时证明多个语句：
```
f₁(u₁) = v₁
f₂(u₂) = v₂
...
fₖ(uₖ) = vₖ
```

**优化**：不是生成 k 个独立证明，而是：
1. 使用 transcript 生成随机挑战 `ρ₁, ρ₂, ..., ρₖ`
2. 构造聚合多项式：`F = ρ₁·f₁ + ρ₂·f₂ + ... + ρₖ·fₖ`
3. 对 F 生成单个证明

**优势**：
- 证明大小：O(n)（变量数）而非 O(kn)
- 验证时间：O(n)（一次配对）而非 O(kn)

### 3.6 单次验证

```rust
fn verify(
    verifier_param: &Self::VerifierParam,
    commitment: &Self::Commitment,
    point: &Self::Point,
    value: &E::ScalarField,
    proof: &Self::Proof,
) -> Result<bool, PCSError>
```

**目标**：验证 `C` 是否承诺了在 `point` 处求值为 `value` 的多项式。

**复杂度**（注释）：
- n 次配对积运算
- n 次 MSM

**验证方程**（简化版）：

对于每一轮的证明 `(Lᵢ, Rᵢ)` 和挑战 `rᵢ`：
```
e(Lᵢ, [τ-rᵢ]₂) · e(Rᵢ, [1]₂) = e(C_{i-1}, [1]₂)
```

**配对检查**：利用双线性性质验证多项式关系。

### 3.7 批量验证

```rust
fn batch_verify(
    verifier_param: &Self::VerifierParam,
    commitments: &[Self::Commitment],
    points: &[Self::Point],
    batch_proof: &Self::BatchProof,
    transcript: &mut IOPTranscript<E::ScalarField>,
) -> Result<bool, PCSError>
```

**输入**：
- k 个承诺 `C₁, ..., Cₖ`
- k 个点 `u₁, ..., uₖ`
- 批量证明（聚合后的单个证明）
- transcript（用于重现随机挑战）

**过程**：
1. 从 transcript 重新生成挑战 `ρ₁, ..., ρₖ`（确定性）
2. 计算聚合承诺：`C = ρ₁·C₁ + ... + ρₖ·Cₖ`
3. 使用聚合的点和值验证单个证明

**安全性**：基于 Fiat-Shamir 启发式，挑战值不可预测。

## 4. 完整流程示例

```rust
// === Setup Phase ===
let log_size = 10; // 支持 10 个变量
let srs = MultilinearKzgPCS::<E>::gen_srs_for_testing(&mut rng, log_size)?;

// === Prover Side ===
// 1. 创建多项式（3 个变量，8 个系数）
let poly = Arc::new(DenseMultilinearExtension::from_evaluations_vec(
    3, 
    vec![1, 2, 3, 4, 5, 6, 7, 8].iter().map(|&x| F::from(x)).collect()
));

// 2. 裁剪参数
let (prover_param, verifier_param) = MultilinearKzgPCS::trim(&srs, None, Some(3))?;

// 3. 生成承诺
let commitment = MultilinearKzgPCS::commit(&prover_param, &poly)?;
// commitment 是单个 G1 点，~48 字节

// 4. 选择求值点
let point = vec![F::from(2), F::from(3), F::from(4)];

// 5. 计算求值（线下）
let value = poly.evaluate(&point).unwrap(); // = f(2,3,4)

// 6. 生成证明
let (proof, eval) = MultilinearKzgPCS::open(&prover_param, &poly, &point)?;
assert_eq!(eval, value);

// === Verifier Side ===
// 7. 验证（只需要 commitment, point, value, proof）
let valid = MultilinearKzgPCS::verify(
    &verifier_param,
    &commitment,
    &point,
    &value,
    &proof
)?;
assert!(valid);
```

## 5. 批量操作示例

```rust
// 多个多项式和求值点
let polys = vec![poly1, poly2, poly3];
let points = vec![point1, point2, point3];
let evals = vec![
    poly1.evaluate(&point1)?,
    poly2.evaluate(&point2)?,
    poly3.evaluate(&point3)?,
];

let mut transcript = IOPTranscript::new(b"batch_opening");

// 批量打开
let batch_proof = MultilinearKzgPCS::multi_open(
    &prover_param,
    &polys,
    &points,
    &evals,
    &mut transcript,
)?;

// 批量验证
let commitments = polys.iter()
    .map(|p| MultilinearKzgPCS::commit(&prover_param, p))
    .collect::<Result<Vec<_>, _>>()?;

let mut transcript = IOPTranscript::new(b"batch_opening");
let valid = MultilinearKzgPCS::batch_verify(
    &verifier_param,
    &commitments,
    &points,
    &batch_proof,
    &mut transcript,
)?;
```



```
/// 输入多项式 `p` 和点 `point` 时，输出相应的证明。此函数无需将求值结果作为输入。
///
/// 该函数在 G1 上执行 2^num_var 次标量乘法：
/// - 它会进行 `num_var` 轮运算，
/// - 在第 i 轮，我们对 2^(num_var - i) 个 G1 元素计算一次多标量乘法（MSM）。
```

**Created:** 12/25/2025 20:45:01  
**Updated:** 12/26/2025 11:27:18  
**Exported:** 12/26/2025 11:27:48  

## Response:
我来详细解释这个 `open_internal` 函数，这是 **KZG 多线性多项式打开证明**的核心算法。

## 1. 函数签名和目的

```rust
fn open_internal<E: Pairing>(
    prover_param: &MultilinearProverParam<E>,
    polynomial: &DenseMultilinearExtension<E::ScalarField>,
    point: &[E::ScalarField],
) -> Result<(MultilinearKzgProof<E>, E::ScalarField), PCSError>
```

**目的**：证明多线性多项式 `f(x₁, x₂, ..., xₙ)` 在点 `(u₁, u₂, ..., uₙ)` 处的求值为 `v`。

**输入**：
- `prover_param` - 包含 SRS 的证明者参数
- `polynomial` - n 变量的多线性多项式（有 2^n 个系数）
- `point` - 求值点 `[u₁, u₂, ..., uₙ]`

**输出**：
- `MultilinearKzgProof` - 包含 n 个 G1 群元素的证明
- `E::ScalarField` - 求值结果 `v = f(u₁, ..., uₙ)`

## 2. 参数验证

```rust
if polynomial.num_vars() > prover_param.num_vars {
    return Err(PCSError::InvalidParameters(format!(
        "Polynomial num_vars {} exceed the limit {}",
        polynomial.num_vars, prover_param.num_vars
    )));
}
```

**检查 1**：多项式的变量数不能超过 SRS 支持的最大变量数。

```rust
if polynomial.num_vars() != point.len() {
    return Err(PCSError::InvalidParameters(format!(
        "Polynomial num_vars {} does not match point len {}",
        polynomial.num_vars, point.len()
    )));
}
```

**检查 2**：求值点的维度必须与多项式的变量数匹配。

## 3. 初始化

```rust
let nv = polynomial.num_vars();  // 变量数，例如 nv = 3

// 计算需要跳过的 SRS 前缀
let ignored = prover_param.num_vars - nv + 1;

// 将多项式转换为求值表示
let mut f = polynomial.to_evaluations();

// 存储每一轮的证明
let mut proofs = Vec::new();
```

### 为什么要 `ignored`？

**例子**：
- SRS 支持 10 个变量（prover_param.num_vars = 10）
- 当前多项式只有 3 个变量（nv = 3）
- `ignored = 10 - 3 + 1 = 8`

SRS 的结构是分层的，每层对应不同的变量组合。我们只需要对应 3 变量的那部分。

### `f` 的初始状态

假设 nv = 3，`f` 是长度为 2³ = 8 的向量：
```
f = [f(0,0,0), f(0,0,1), f(0,1,0), f(0,1,1), 
     f(1,0,0), f(1,0,1), f(1,1,0), f(1,1,1)]
```

## 4. 核心循环：递归折叠算法

```rust
for (i, (&point_at_k, gi)) in point
    .iter()
    .zip(prover_param.powers_of_g[ignored..ignored + nv].iter())
    .enumerate()
{
    let k = nv - 1 - i;      // 当前处理的变量索引（从后往前）
    let cur_dim = 1 << k;     // 当前维度 = 2^k
```

**循环结构**：
- 总共 `nv` 轮（n = 3 时，3 轮）
- 每轮处理一个变量，从 xₙ 到 x₁（逆序）
- `point_at_k` 是当前变量的求值点
- `gi` 是当前轮使用的 SRS

### 轮次对应关系（nv = 3 为例）

| 轮次 i | k    | cur_dim | 处理变量 | point_at_k |
| ------ | ---- | ------- | -------- | ---------- |
| 0      | 2    | 4       | x₃       | u₃         |
| 1      | 1    | 2       | x₂       | u₂         |
| 2      | 0    | 1       | x₁       | u₁         |

## 5. 单轮折叠详解

### 5.1 理论基础

多线性多项式的关键性质：
```
f(x₁, ..., xₖ₋₁, xₖ) = (1 - xₖ)·f(x₁, ..., xₖ₋₁, 0) 
                      + xₖ·f(x₁, ..., xₖ₋₁, 1)
```

记：
- `f_L(x₁, ..., xₖ₋₁) = f(x₁, ..., xₖ₋₁, 0)` （左半部分）
- `f_R(x₁, ..., xₖ₋₁) = f(x₁, ..., xₖ₋₁, 1)` （右半部分）

则：
```
f(x₁, ..., xₖ) = f_L + (f_R - f_L)·xₖ
               = f_L + q·xₖ
```

其中 `q = f_R - f_L` 是**商多项式**。

### 5.2 代码实现

```rust
let mut q = vec![E::ScalarField::zero(); cur_dim];
let mut r = vec![E::ScalarField::zero(); cur_dim];
```

- `q` - 商多项式的系数（差值）
- `r` - 折叠后的多项式

```rust
for b in 0..(1 << k) {
    // q[b] = f[1, b] - f[0, b]
    q[b] = f[(b << 1) + 1] - f[b << 1];
    
    // r[b] = f[0, b] + q[b] * point_at_k
    r[b] = f[b << 1] + (q[b] * point_at_k);
}
```

**索引解析**：
- `b << 1` = `2*b` → 对应 xₖ = 0 的情况
- `(b << 1) + 1` = `2*b + 1` → 对应 xₖ = 1 的情况

### 5.3 第一轮详细示例（k=2, 处理 x₃）

**初始状态**：
```
f = [f(0,0,0), f(0,0,1), f(0,1,0), f(0,1,1), 
     f(1,0,0), f(1,0,1), f(1,1,0), f(1,1,1)]
     
point_at_k = u₃
```

**循环 b = 0 到 3**：

```rust
// b = 0: 处理 f(0,0,x₃)
q[0] = f[1] - f[0]           // = f(0,0,1) - f(0,0,0)
r[0] = f[0] + q[0] * u₃      // = f(0,0,0) + (f(0,0,1)-f(0,0,0))·u₃
                             // = f(0,0,u₃)

// b = 1: 处理 f(0,1,x₃)
q[1] = f[3] - f[2]           // = f(0,1,1) - f(0,1,0)
r[1] = f[2] + q[1] * u₃      // = f(0,1,u₃)

// b = 2: 处理 f(1,0,x₃)
q[2] = f[5] - f[4]           // = f(1,0,1) - f(1,0,0)
r[2] = f[4] + q[2] * u₃      // = f(1,0,u₃)

// b = 3: 处理 f(1,1,x₃)
q[3] = f[7] - f[6]           // = f(1,1,1) - f(1,1,0)
r[3] = f[6] + q[3] * u₃      // = f(1,1,u₃)
```

**结果**：
```
q = [q₀, q₁, q₂, q₃]  // 商多项式（4个系数）
r = [f(0,0,u₃), f(0,1,u₃), f(1,0,u₃), f(1,1,u₃)]  // 折叠后的多项式
```

### 5.4 更新 f

```rust
f = r;
```

现在 `f` 变成了关于 (x₁, x₂) 的多线性多项式，x₃ 已经被"固定"为 u₃。

## 6. 生成证明

```rust
// MSM: 多标量乘法
let msm_timer = start_timer!(|| format!(
    "msm of size {} at round {}", 
    gi.evals.len(), 
    i
));

proofs.push(E::G1::msm_unchecked(&gi.evals, &q).into_affine());

end_timer!(msm_timer);
```

**计算**：
```
πᵢ = q[0]·G₀ + q[1]·G₁ + ... + q[2^k-1]·G₂ₖ₋₁
```

其中 `gi.evals = [G₀, G₁, ..., G₂ₖ₋₁]` 是从 SRS 中取出的对应层级的群元素。

**性能瓶颈**：这是最耗时的操作！
- 第 0 轮：2² = 4 个标量乘法
- 第 1 轮：2¹ = 2 个标量乘法  
- 第 2 轮：2⁰ = 1 个标量乘法
- 总计：4 + 2 + 1 = 7 次（对于 nv=3）

一般情况：`∑ᵢ 2^(nv-1-i) = 2^nv - 1` 次标量乘法。

## 7. 完整三轮示例（nv=3）

### 输入
```
f(x₁, x₂, x₃) 有 8 个系数
point = [u₁, u₂, u₃]
```

### Round 0: 折叠 x₃
```
输入：f = [f(0,0,0), ..., f(1,1,1)]  // 8 个值
计算：q₀ = [q₀₀, q₀₁, q₀₂, q₀₃]      // 4 个商
证明：π₀ = MSM(q₀, G₀)               // 1 个 G1 点
输出：f' = [f(0,0,u₃), f(0,1,u₃), f(1,0,u₃), f(1,1,u₃)]  // 4 个值
```

### Round 1: 折叠 x₂
```
输入：f' = [f(0,0,u₃), f(0,1,u₃), f(1,0,u₃), f(1,1,u₃)]  // 4 个值
计算：q₁ = [q₁₀, q₁₁]                 // 2 个商
证明：π₁ = MSM(q₁, G₁)                // 1 个 G1 点
输出：f'' = [f(0,u₂,u₃), f(1,u₂,u₃)]  // 2 个值
```

### Round 2: 折叠 x₁
```
输入：f'' = [f(0,u₂,u₃), f(1,u₂,u₃)]  // 2 个值
计算：q₂ = [q₂₀]                       // 1 个商
证明：π₂ = MSM(q₂, G₂)                 // 1 个 G1 点
输出：f''' = [f(u₁,u₂,u₃)]             // 1 个值（最终求值）
```

## 8. 最终求值

```rust
let eval = evaluate_opt(polynomial, point);
```

**注意**：这里重新计算了一次求值，作为双重检查。理论上 `f` 的最后一个元素应该等于 `eval`。

## 9. 返回结果

```rust
Ok((MultilinearKzgProof { proofs }, eval))
```

**返回**：
- `proofs = [π₀, π₁, π₂]` - 3 个 G1 群元素（每个约 48 字节）
- `eval` - 最终求值 `f(u₁, u₂, u₃)`







