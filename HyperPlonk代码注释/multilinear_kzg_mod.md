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





