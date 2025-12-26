# SRS



//！为多线性多项式KZG实现结构化引用字符串

```rust
use crate::pcs::{
    multilinear_kzg::util::{eq_eval, eq_extension},
    prelude::PCSError,
    StructuredReferenceString,
};
use ark_ec::{pairing::Pairing, scalar_mul::fixed_base::FixedBase, AffineRepr, CurveGroup};
use ark_ff::{Field, PrimeField, Zero};
use ark_poly::DenseMultilinearExtension;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::{
    collections::LinkedList, end_timer, format, rand::Rng, start_timer, string::ToString, vec::Vec,
    UniformRand,
};
use core::iter::FromIterator;

```

>## 一、`crate::pcs::...` —— **你自己项目里的模块**
>
>```rust
>use crate::pcs::{
>    multilinear_kzg::util::{eq_eval, eq_extension},
>    prelude::PCSError,
>    StructuredReferenceString,
>};
>```
>
>### 1️⃣ `crate::pcs::...`
>
>* `crate`：当前 Rust crate（也就是你这个项目）
>  
>* `pcs`：多项式承诺系统（Polynomial Commitment Scheme）
>  
>
>### 2️⃣ `multilinear_kzg::util::{eq_eval, eq_extension}`
>
>这是 **多线性 KZG PCS** 里的工具函数：
>
>* `eq_eval`
>  
>    * 计算 **eq 多项式在某个点的值**
>      
>    * 通常是
>      
>        $$eq(r, x) = \prod_i (r_i x_i + (1-r_i)(1-x_i))$$
>    * 在 **Sumcheck / MLE / IOP** 中用于：
>      
>        * selector
>          
>        * 约束某一条路径
>    
>* `eq_extension`
>  
>    * 构造 **eq 多项式的 MLE 表示**
>      
>    * 常用于把一个点约束扩展成一个多线性多项式
>      
>
>👉 你之前问过 Sumcheck，这俩函数是 **标准配套工具**
>
>* * *
>
>### 3️⃣ `prelude::PCSError`
>
>```rust
>PCSError
>```
>
>* 这是 PCS 相关的 **统一错误类型**
>  
>* 用于：
>  
>    * SRS 不匹配
>      
>    * 多项式维度错误
>      
>    * 验证失败等
>      
>
>* * *
>
>### 4️⃣ `StructuredReferenceString`
>
>```rust
>StructuredReferenceString
>```
>
>* **结构化参考字符串（SRS）**
>  
>* KZG 的核心公共参数：
>  
>    * $$(g, g^τ, g^{τ^2}, ...)$$
>    
>* 在 commit / open / verify 中反复用到
>  
>
>* * *
>
>## 二、`ark_ec` —— **椭圆曲线 & 配对**
>
>```rust
>use ark_ec::{
>    pairing::Pairing,
>    scalar_mul::fixed_base::FixedBase,
>    AffineRepr,
>    CurveGroup
>};
>```
>
>### 1️⃣ `pairing::Pairing`
>
>* 表示 **双线性配对接口**
>  
>* 支持：
>  
>    ```rust
>    e: G1 × G2 → GT
>    ```
>    
>* KZG 验证本质靠 pairing
>  
>
>* * *
>
>### 2️⃣ `scalar_mul::fixed_base::FixedBase`
>
>* **固定基点标量乘**
>  
>* 用于：
>  
>    * 批量计算 `g^{a_i}`
>      
>    * 加速 MSM（Multi-Scalar Multiplication）
>      
>
>👉 在 **承诺生成阶段性能关键**
>
>* * *
>
>### 3️⃣ `AffineRepr`
>
>* 仿射坐标表示（Affine）
>  
>* 比 Projective：
>  
>    * 占用更小
>      
>    * 适合序列化、存储、pairing
>      
>
>* * *
>
>### 4️⃣ `CurveGroup`
>
>* 椭圆曲线群的抽象 trait
>  
>* 支持：
>  
>    * `+`
>      
>    * `* scalar`
>      
>    * `zero()`
>      
>
>* * *
>
>## 三、`ark_ff` —— **有限域**
>
>```rust
>use ark_ff::{Field, PrimeField, Zero};
>```
>
>### 1️⃣ `Field`
>
>* 一般有限域 trait
>  
>* 支持：
>  
>    * `+ - * /`
>      
>
>### 2️⃣ `PrimeField`
>
>* 素数域（如 BLS12-381 的 Fr）
>  
>* 额外支持：
>  
>    * 转换为 big integer
>      
>    * 位分解
>      
>
>### 3️⃣ `Zero`
>
>* 提供：
>  
>    ```rust
>    F::zero()
>    ```
>    
>
>* * *
>
>## 四、`ark_poly` —— **多项式**
>
>```rust
>use ark_poly::DenseMultilinearExtension;
>```
>
>* **多线性扩展（MLE）**
>  
>* 表示：
>  
>    ```text
>    f: {0,1}^n → F
>    ```
>    
>* 在：
>  
>    * Sumcheck
>      
>    * PCS
>      
>    * IOPP  
>        中是核心数据结构
>        
>
>* * *
>
>## 五、`ark_serialize` —— **序列化**
>
>```rust
>use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
>```
>
>* 标准、确定性的序列化
>  
>* 用于：
>  
>    * proof
>      
>    * commitment
>      
>    * SRS 存盘 / 网络传输
>      
>
>* * *
>
>## 六、`ark_std` —— **arkworks 的 std 封装**
>
>```rust
>use ark_std::{
>    collections::LinkedList,
>    end_timer,
>    format,
>    rand::Rng,
>    start_timer,
>    string::ToString,
>    vec::Vec,
>    UniformRand,
>};
>```
>
>### 常用的几个：
>
>| 项目                      | 含义                          |
>| ------------------------- | ----------------------------- |
>| `LinkedList`              | 链表                          |
>| `start_timer / end_timer` | 性能 profiling                |
>| `rand::Rng`               | 随机数生成器                  |
>| `UniformRand`             | 均匀随机抽样                  |
>| `Vec`                     | ark_std 自己 re-export 的 Vec |
>| `ToString`                | 字符串转换                    |
>
>👉 ark_std 是为了支持 `no_std` 环境
>
>* * *
>
>## 七、`core::iter::FromIterator`
>
>```rust
>use core::iter::FromIterator;
>```
>
>* 支持：
>  
>    ```rust
>    let v = Vec::from_iter(iter);
>    ```
>    
>* 常见于：
>  
>    * `collect()`
>      
>    * 自定义容器构造
>      
>
>* * *



```rust
//对G1或G2在{0,1}^n上的评估
#[derive(CanonicalSerialize, CanonicalDeserialize, Clone, Debug)]
pub struct Evaluations<C: AffineRepr> {
    /// 评估。
    pub evals: Vec<C>,
}

/// 通用参数
#[derive(CanonicalSerialize, CanonicalDeserialize, Clone, Debug)]
pub struct MultilinearUniversalParams<E: Pairing> {
    /// 证明者参数
    pub prover_param: MultilinearProverParam<E>,
    /// h^randomness: h^t1, h^t2, ..., **h^{t_nv}**
    pub h_mask: Vec<E::G2Affine>, //G2Affine表示椭圆曲线的点的坐标形式
}

/// 证明者参数
#[derive(CanonicalSerialize, CanonicalDeserialize, Clone, Debug)]
pub struct MultilinearProverParam<E: Pairing> {
    /// 变量个数
    pub num_vars: usize,
    /// `pp_{0}`, `pp_{1}`, ...,pp_{nu_vars} 定义
    /// by XZZPD19 where pp_{nv-0}=g and
    /// pp_{nv-i}=g^{eq((t_1,..t_i),(X_1,..X_i))}
    pub powers_of_g: Vec<Evaluations<E::G1Affine>>,
    /// generator for G1
    pub g: E::G1Affine,
    /// generator for G2
    pub h: E::G2Affine,
}

/// Verifier Parameters
#[derive(CanonicalSerialize, CanonicalDeserialize, Clone, Debug)]
pub struct MultilinearVerifierParam<E: Pairing> {
    /// number of variables
    pub num_vars: usize,
    /// generator of G1
    pub g: E::G1Affine,
    /// generator of G2
    pub h: E::G2Affine,
    /// h^randomness: h^t1, h^t2, ..., **h^{t_nv}**
    pub h_mask: Vec<E::G2Affine>,
}
```

>## 一、`Evaluations<C>` —— 在 `{0,1}^n` 上的群元素评估表
>
>```rust
>// 对 G1 或 G2 在 {0,1}^n 上的评估
>#[derive(CanonicalSerialize, CanonicalDeserialize, Clone, Debug)]
>pub struct Evaluations<C: AffineRepr> {
>    /// 评估
>    pub evals: Vec<C>,
>}
>```
>
>### 1️⃣ 这是什么？
>
>* 一个 **向量形式的评估表**
>  
>* `C: AffineRepr`：
>  
>    * 可以是 `G1Affine`
>      
>    * 也可以是 `G2Affine`
>      
>
>### 2️⃣ `{0,1}^n` 的含义
>
>* `{0,1}^n` 一共有 `2^n` 个点
>  
>* `evals.len() = 2^n`
>  
>* 第 `i` 个元素代表：
>  
>    $$f(b_1,\dots,b_n) \in G$$
>
>### 3️⃣ 为什么评估值是「群元素」？
>
>在多线性 KZG 里：
>
>* 我们不是存 `f(x) ∈ F`
>  
>* 而是存：
>  
>    $$g^{f(x)} \in G_1 \quad\text{或}\quad h^{f(x)} \in G_2$$
>
>👉 `Evaluations<G1Affine>` = `g^{f(·)}` 的全集
>
>* * *
>
>## 二、`MultilinearUniversalParams` —— 通用（公开）参数
>
>```rust
>#[derive(CanonicalSerialize, CanonicalDeserialize, Clone, Debug)]
>pub struct MultilinearUniversalParams<E: Pairing> {
>    /// 证明者参数
>    pub prover_param: MultilinearProverParam<E>,
>    /// h^randomness: h^t1, h^t2, ..., h^{t_nv}
>    pub h_mask: Vec<E::G2Affine>,
>}
>```
>
>### 1️⃣ Universal Params 是什么？
>
>* PCS 的 **SRS（结构化参考字符串）**
>  
>* 对 prover / verifier 都是公开的
>  
>
>* * *
>
>### 2️⃣ `h_mask`
>
>```text
>h^{t1}, h^{t2}, ..., h^{t_nv}
>```
>
>数学上：
>
>* `t1, t2, ..., t_nv` 是秘密随机数
>  
>* 用来：
>  
>    * blind 证明
>      
>    * 防止线性关系攻击
>      
>    * 支持多轮 Sumcheck / opening
>      
>
>👉 在 verifier 那边也会用到（后面你能看到）
>
>* * *
>
>## 三、`MultilinearProverParam` —— 证明者参数（核心）
>
>```rust
>#[derive(CanonicalSerialize, CanonicalDeserialize, Clone, Debug)]
>pub struct MultilinearProverParam<E: Pairing> {
>    /// 变量个数
>    pub num_vars: usize,
>```
>
>表示多线性多项式：
>
>$$f(X_1,\dots,X_n)$$
>
>* * *
>
>### 1️⃣ `powers_of_g` —— 多线性 KZG 的精髓 ⭐
>
>```rust
>pub powers_of_g: Vec<Evaluations<E::G1Affine>>,
>```
>
>注释非常关键：
>
>```rust
>/// pp_{0}, pp_{1}, ..., pp_{num_vars}
>/// by XZZPD19 where pp_{nv-0}=g and
>/// pp_{nv-i}=g^{eq((t_1,..t_i),(X_1,..X_i))}
>```
>
>#### 数学含义
>
>* 对每个 `i = 0..num_vars`
>  
>* 预计算：
>  
>    $$pp_{nv-i} = g^{ eq((t_1,\dots,t_i),(X_1,\dots,X_i)) }$$
>
>其中：
>
>* `eq` 是你之前看到的 `eq_extension`
>  
>* `(X_1,..,X_i) ∈ {0,1}^i`
>  
>
>* * *
>
>#### 直观理解
>
>* `powers_of_g[i]` 是一个 **表**
>  
>* 表中每一项是：
>  
>    ```text
>    g^{eq(t_prefix, x_prefix)}
>    ```
>    
>* 在 Sumcheck 的第 `i` 轮：
>  
>    * 用来“消掉”一个变量
>      
>    * 做承诺折叠（folding）
>      
>
>👉 这是 **多线性 KZG 能高效 open 的根本原因**
>
>* * *
>
>### 2️⃣ `g` / `h`
>
>```rust
>pub g: E::G1Affine,
>pub h: E::G2Affine,
>```
>
>* 椭圆曲线生成元
>  
>* 满足：
>  
>    ```text
>    e(g, h) ≠ 1
>    ```
>    
>* 用于 pairing 校验
>  
>
>* * *
>
>## 四、`MultilinearVerifierParam` —— 验证者参数
>
>```rust
>#[derive(CanonicalSerialize, CanonicalDeserialize, Clone, Debug)]
>pub struct MultilinearVerifierParam<E: Pairing> {
>    pub num_vars: usize,
>    pub g: E::G1Affine,
>    pub h: E::G2Affine,
>    pub h_mask: Vec<E::G2Affine>,
>}
>```
>
>### 1️⃣ 和 Prover Param 的区别
>
>| Prover             | Verifier          |
>| ------------------ | ----------------- |
>| 有 `powers_of_g`   | ❌ 没有            |
>| 大                 | 小                |
>| 用于生成承诺和证明 | 用于 pairing 验证 |
>
>👉 verifier **不需要** `{0,1}^n` 的整张表  
>👉 只要：
>
>* `g`
>  
>* `h`
>  
>* `h^{t_i}`
>





```rust
impl<E: Pairing> StructuredReferenceString<E> for MultilinearUniversalParams<E> {
    type ProverParam = MultilinearProverParam<E>;
    type VerifierParam = MultilinearVerifierParam<E>;

    /// 从公共参数中提取证明者参数。
    fn extract_prover_param(&self, supported_num_vars: usize) -> Self::ProverParam {
        let to_reduce = self.prover_param.num_vars - supported_num_vars;

        Self::ProverParam {
            powers_of_g: self.prover_param.powers_of_g[to_reduce..].to_vec(),
            g: self.prover_param.g,
            h: self.prover_param.h,
            num_vars: supported_num_vars,
        }
    }

    /// 从公共参数中提取验证者参数。
    fn extract_verifier_param(&self, supported_num_vars: usize) -> Self::VerifierParam {
        let to_reduce = self.prover_param.num_vars - supported_num_vars;
        Self::VerifierParam {
            num_vars: supported_num_vars,
            g: self.prover_param.g,
            h: self.prover_param.h,
            h_mask: self.h_mask[to_reduce..].to_vec(),
        }
    }

   //调整通用参数，将用于多线性多项式的公共参数专门化到给定的`supported_num_vars`
   //并返回承诺者密钥和验证者密钥。
   //`supported_num_vars`应在`1..=params.num_vars`范围内。
    fn trim(
        &self,
        supported_num_vars: usize,
    ) -> Result<(Self::ProverParam, Self::VerifierParam), PCSError> {
        if supported_num_vars > self.prover_param.num_vars {
            return Err(PCSError::InvalidParameters(format!(
                "SRS does not support target number of vars {}",
                supported_num_vars
            )));
        }

        let to_reduce = self.prover_param.num_vars - supported_num_vars;
        let ck = Self::ProverParam {
            powers_of_g: self.prover_param.powers_of_g[to_reduce..].to_vec(),
            g: self.prover_param.g,
            h: self.prover_param.h,
            num_vars: supported_num_vars,
        };
        let vk = Self::VerifierParam {
            num_vars: supported_num_vars,
            g: self.prover_param.g,
            h: self.prover_param.h,
            h_mask: self.h_mask[to_reduce..].to_vec(),
        };
        Ok((ck, vk))
    }

```



>## 一、整体在干什么？
>
>```rust
>impl<E: Pairing> StructuredReferenceString<E> for MultilinearUniversalParams<E>
>```
>
>意思是：
>
>* `MultilinearUniversalParams<E>`  
>    👉 **是一个“通用 SRS”**（支持最多 `num_vars` 个变量）
>    
>* 通过实现 `StructuredReferenceString`：  
>    👉 它可以被 **裁剪（trim）** 成：
>    
>    * `MultilinearProverParam<E>`（证明者用）
>      
>    * `MultilinearVerifierParam<E>`（验证者用）
>      
>
>* * *
>
>## 二、关联类型（Associated Types）
>
>```rust
>type ProverParam = MultilinearProverParam<E>;
>type VerifierParam = MultilinearVerifierParam<E>;
>```
>
>说明这个 SRS：
>
>* 生成的 **CK（committer / prover key）**
>  
>* 和 **VK（verifier key）**  
>    分别是什么类型。
>    
>
>* * *
>
>## 三、核心概念：为什么要“裁剪（trim）”？🤔
>
>Universal SRS 一般是：
>
>```text
>支持 n_max 个变量
>```
>
>但实际使用时：
>
>```text
>这个多项式只有 n < n_max 个变量
>```
>
>👉 **没必要用完整 SRS**
>
>* 不安全（暴露更多结构）
>  
>* 不高效（参数更大）
>  
>
>所以要：
>
>```text
>把 SRS 裁到只支持 n 个变量
>```
>
>* * *
>
>## 四、`to_reduce` 是什么意思（关键变量）
>
>```rust
>let to_reduce = self.prover_param.num_vars - supported_num_vars;
>```
>
>* `self.prover_param.num_vars`：SRS 支持的最大变量数（n_max）
>  
>* `supported_num_vars`：你现在要用的变量数（n）
>  
>
>所以：
>
>```text
>to_reduce = n_max - n
>```
>
>意思是：
>
>> **前面多出来的那几层 SRS 要丢掉**
>
>* * *
>
>## 五、`extract_prover_param` —— 只取证明者参数
>
>```rust
>fn extract_prover_param(&self, supported_num_vars: usize) -> Self::ProverParam
>```
>
>### 干了什么？
>
>```rust
>powers_of_g: self.prover_param.powers_of_g[to_reduce..].to_vec()
>```
>
>* `powers_of_g` 原本是：
>  
>    ```text
>    [pp_0, pp_1, ..., pp_{n_max}]
>    ```
>    
>* 裁剪后变成：
>  
>    ```text
>    [pp_{n_max-n}, ..., pp_{n_max}]
>    ```
>    
>
>### 数学含义
>
>只保留：
>
>* **后 `supported_num_vars` 轮 Sumcheck / folding 需要的 pp**
>  
>
>* * *
>
>## 六、`extract_verifier_param` —— 只取验证者参数
>
>```rust
>fn extract_verifier_param(&self, supported_num_vars: usize) -> Self::VerifierParam
>```
>
>### 核心代码
>
>```rust
>h_mask: self.h_mask[to_reduce..].to_vec()
>```
>
>* `h_mask` 原来是：
>  
>    ```text
>    [h^{t1}, h^{t2}, ..., h^{t_{n_max}}]
>    ```
>    
>* 裁剪后是：
>  
>    ```text
>    [h^{t_{n_max-n+1}}, ..., h^{t_{n_max}}]
>    ```
>    
>
>👉 verifier 只需要后 `n` 个随机性
>
>* * *
>
>## 七、`trim` —— 一次性生成 CK 和 VK（最常用）
>
>```rust
>fn trim(
>    &self,
>    supported_num_vars: usize,
>) -> Result<(Self::ProverParam, Self::VerifierParam), PCSError>
>```
>
>### 1️⃣ 参数合法性检查
>
>```rust
>if supported_num_vars > self.prover_param.num_vars {
>    return Err(PCSError::InvalidParameters(...));
>}
>```
>
>防止：
>
>* 用一个只支持 10 变量的 SRS
>  
>* 去 commit 12 变量的多项式
>  
>
>* * *
>
>### 2️⃣ 构造 Prover Key（CK）
>
>```rust
>let ck = Self::ProverParam {
>    powers_of_g: self.prover_param.powers_of_g[to_reduce..].to_vec(),
>    g: self.prover_param.g,
>    h: self.prover_param.h,
>    num_vars: supported_num_vars,
>};
>```
>
>* * *
>
>### 3️⃣ 构造 Verifier Key（VK）
>
>```rust
>let vk = Self::VerifierParam {
>    num_vars: supported_num_vars,
>    g: self.prover_param.g,
>    h: self.prover_param.h,
>    h_mask: self.h_mask[to_reduce..].to_vec(),
>};
>```
>
>* * *
>
>### 4️⃣ 返回 `(ck, vk)`
>
>```rust
>Ok((ck, vk))
>```
>
>* * *
>
>## 八、把这段代码翻译成“协议语言”
>
>> 给定一个 **支持最多 `n_max` 个变量的多线性 KZG 通用 SRS**，  
>> 我们可以：
>>
>> * 裁剪出只支持 `n` 个变量的 **证明者参数（CK）**
>>
>> * 裁剪出只支持 `n` 个变量的 **验证者参数（VK）**
>>
>>
>> 裁剪方式是：
>>
>> * 丢弃前 `n_max - n` 层的 `powers_of_g`
>>
>> * 丢弃前 `n_max - n` 个 `h^{t_i}`
>>
>
>* * *
>
>## 九、和你在看的 Sumcheck / Multilinear KZG 的关系
>
>你现在看到的流程是：
>
>```text
>Universal SRS
>   ↓ trim(n)
>(ProverParam, VerifierParam)
>   ↓
>Commit → Sumcheck → Open → Verify
>```
>
>每一轮 Sumcheck：
>
>* 用一个 `powers_of_g[i]`
>  
>* verifier 用一个 `h_mask[i]`
>  
>





```rust
 fn gen_srs_for_testing<R: Rng>(rng: &mut R, num_vars: usize) -> Result<Self, PCSError> {
        if num_vars == 0 {
            return Err(PCSError::InvalidParameters(
                "constant polynomial not supported".to_string(),
            ));
        }

        let total_timer = start_timer!(|| "SRS generation");

        let pp_generation_timer = start_timer!(|| "Prover Param generation");

        let g = E::G1::rand(rng);
        let h = E::G2::rand(rng);

        let mut powers_of_g = Vec::new();

        let t: Vec<_> = (0..num_vars).map(|_| E::ScalarField::rand(rng)).collect();
        let scalar_bits = E::ScalarField::MODULUS_BIT_SIZE as usize;

        let mut eq: LinkedList<DenseMultilinearExtension<E::ScalarField>> =
            LinkedList::from_iter(eq_extension(&t));
        let mut eq_arr = LinkedList::new();
        let mut base = eq.pop_back().unwrap().evaluations;

        for i in (0..num_vars).rev() {
            eq_arr.push_front(remove_dummy_variable(&base, i)?);
            if i != 0 {
                let mul = eq.pop_back().unwrap().evaluations;
                base = base
                    .into_iter()
                    .zip(mul.into_iter())
                    .map(|(a, b)| a * b)
                    .collect();
            }
        }

        let mut pp_powers = Vec::new();
        let mut total_scalars = 0;
        for i in 0..num_vars {
            let eq = eq_arr.pop_front().unwrap();
            let pp_k_powers = (0..(1 << (num_vars - i))).map(|x| eq[x]);
            pp_powers.extend(pp_k_powers);
            total_scalars += 1 << (num_vars - i);
        }
        let window_size = FixedBase::get_mul_window_size(total_scalars);
        let g_table = FixedBase::get_window_table(scalar_bits, window_size, g);

        let pp_g = E::G1::normalize_batch(&FixedBase::msm(
            scalar_bits,
            window_size,
            &g_table,
            &pp_powers,
        ));

        let mut start = 0;
        for i in 0..num_vars {
            let size = 1 << (num_vars - i);
            let pp_k_g = Evaluations {
                evals: pp_g[start..(start + size)].to_vec(),
            };
            // check correctness of pp_k_g
            let t_eval_0 = eq_eval(&vec![E::ScalarField::zero(); num_vars - i], &t[i..num_vars])?;
            assert_eq!((g * t_eval_0).into(), pp_k_g.evals[0]);
            powers_of_g.push(pp_k_g);
            start += size;
        }
        let gg = Evaluations {
            evals: [g.into_affine()].to_vec(),
        };
        powers_of_g.push(gg);

        let pp = Self::ProverParam {
            num_vars,
            g: g.into_affine(),
            h: h.into_affine(),
            powers_of_g,
        };

        end_timer!(pp_generation_timer);

        let vp_generation_timer = start_timer!(|| "VP generation");
        let h_mask = {
            let window_size = FixedBase::get_mul_window_size(num_vars);
            let h_table = FixedBase::get_window_table(scalar_bits, window_size, h);
            E::G2::normalize_batch(&FixedBase::msm(scalar_bits, window_size, &h_table, &t))
        };
        end_timer!(vp_generation_timer);
        end_timer!(total_timer);
        Ok(Self {
            prover_param: pp,
            h_mask,
        })
    }
}

/// fix first `pad` variables of `poly` represented in evaluation form to zero
fn remove_dummy_variable<F: Field>(poly: &[F], pad: usize) -> Result<Vec<F>, PCSError> {
    if pad == 0 {
        return Ok(poly.to_vec());
    }
    if !poly.len().is_power_of_two() {
        return Err(PCSError::InvalidParameters(
            "Size of polynomial should be power of two.".to_string(),
        ));
    }
    let nv = ark_std::log2(poly.len()) as usize - pad;
    Ok((0..(1 << nv)).map(|x| poly[x << pad]).collect())
}

```

>## 一、整体一句话先给你结论
>
>> `gen_srs_for_testing` 做的事是：  
>> **随机生成秘密点 `t = (t₁,…,tₙ)`，然后构造多线性 KZG 所需的 SRS：**
>>
>> * 证明者用的
>>
>>     $$\{ g^{eq((t_1..t_i),(X_1..X_i))} \}_{i=0..n}$$
>> * 验证者用的
>>
>>     $$\{ h^{t_1}, \dots, h^{t_n} \}$$
>>
>> 并且用 **固定基 MSM** 高效算出来。
>
>这正是 **XZZPD19 / Multilinear KZG** 的标准 SRS 生成方式。
>
>* * *
>
>## 二、函数入口 & 参数检查
>
>```rust
>fn gen_srs_for_testing<R: Rng>(rng: &mut R, num_vars: usize)
>```
>
>* `num_vars`：多线性多项式的变量个数 `n`
>  
>* 仅用于 **测试**（不安全生成方式）
>  
>
>```rust
>if num_vars == 0 {
>    return Err(PCSError::InvalidParameters(
>        "constant polynomial not supported".to_string(),
>    ));
>}
>```
>
>👉 不支持常数多项式（`{0,1}^0`）
>
>* * *
>
>## 三、随机生成基础群元素
>
>```rust
>let g = E::G1::rand(rng);
>let h = E::G2::rand(rng);
>```
>
>数学上：
>
>* `g ∈ G1`
>  
>* `h ∈ G2`
>  
>* 是整个 KZG 的生成元
>  
>
>* * *
>
>## 四、生成秘密点 `t = (t₁,…,tₙ)`
>
>```rust
>let t: Vec<_> = (0..num_vars).map(|_| E::ScalarField::rand(rng)).collect();
>```
>
>这就是：
>
>$$t \leftarrow \mathbb{F}^n$$
>
>👉 **这是 SRS 的“毒素（toxic waste）”**
>
>* * *
>
>## 五、构造 eq 多项式（最核心的部分 ⭐）
>
>```rust
>let mut eq: LinkedList<DenseMultilinearExtension<E::ScalarField>> =
>    LinkedList::from_iter(eq_extension(&t));
>```
>
>### 含义
>
>* `eq_extension(&t)` 生成：
>  
>    ```text
>    eq(t1,..,tn)(X1,..,Xn)
>    eq(t2,..,tn)(X2,..,Xn)
>    ...
>    eq(tn)(Xn)
>    ```
>    
>* 每一个都是一个 **MLE**
>  
>
>* * *
>
>### 接下来这段是「逐层折叠 eq」
>
>```rust
>let mut base = eq.pop_back().unwrap().evaluations;
>```
>
>* 从最底层开始（`eq(t_n)(X_n)`）
>  
>
>```rust
>for i in (0..num_vars).rev() {
>    eq_arr.push_front(remove_dummy_variable(&base, i)?);
>    if i != 0 {
>        let mul = eq.pop_back().unwrap().evaluations;
>        base = base
>            .into_iter()
>            .zip(mul.into_iter())
>            .map(|(a, b)| a * b)
>            .collect();
>    }
>}
>```
>
>### 数学意义（非常重要）
>
>这一步在构造：
>
>$$eq((t_1..t_i),(X_1..X_i))$$
>
>也就是 **Sumcheck / Multilinear KZG 每一轮用的 eq 前缀多项式**。
>
>* `remove_dummy_variable`：固定前面的变量为 0
>  
>* `base *= next eq`：逐轮乘起来
>  
>
>最终：
>
>```text
>eq_arr[i] = eq((t1..ti),(X1..Xi))
>```
>
>* * *
>
>## 六、把所有 eq 系数摊平成一个大 MSM 向量
>
>```rust
>for i in 0..num_vars {
>    let eq = eq_arr.pop_front().unwrap();
>    let pp_k_powers = (0..(1 << (num_vars - i))).map(|x| eq[x]);
>    pp_powers.extend(pp_k_powers);
>}
>```
>
>这里在干嘛？
>
>👉 把所有：
>
>$$eq((t_1..t_i),(X_1..X_i))(x)$$
>
>的 **标量值** 收集起来
>
>后面一次性算：
>
>$$g^{eq(\cdot)}$$
>
>* * *
>
>## 七、固定基 MSM：算所有 `g^{eq}`（性能关键）
>
>```rust
>let window_size = FixedBase::get_mul_window_size(total_scalars);
>let g_table = FixedBase::get_window_table(scalar_bits, window_size, g);
>
>let pp_g = E::G1::normalize_batch(&FixedBase::msm(
>    scalar_bits,
>    window_size,
>    &g_table,
>    &pp_powers,
>));
>```
>
>数学上就是：
>
>$$\{ g^{s_1}, g^{s_2}, \dots \}$$
>
>其中：
>
>* `s_i` = eq 多项式在 `{0,1}^k` 上的值
>  
>
>* * *
>
>## 八、按层切分，得到 `powers_of_g`
>
>```rust
>for i in 0..num_vars {
>    let size = 1 << (num_vars - i);
>    let pp_k_g = Evaluations {
>        evals: pp_g[start..(start + size)].to_vec(),
>    };
>```
>
>每一层：
>
>```text
>pp_i = { g^{eq((t1..ti),(X1..Xi))(x)} }_{x ∈ {0,1}^{n-i}}
>```
>
>* * *
>
>### 正确性断言（非常重要）
>
>```rust
>let t_eval_0 = eq_eval(&vec![0; num_vars - i], &t[i..num_vars])?;
>assert_eq!((g * t_eval_0).into(), pp_k_g.evals[0]);
>```
>
>验证：
>
>$$g^{eq(t_{i+1}..t_n, 0)} = g^{∏(1 - t_j)}$$
>
>👉 确保 SRS 构造是对的
>
>* * *
>
>### 最后一项：`pp_0 = g`
>
>```rust
>let gg = Evaluations {
>    evals: [g.into_affine()].to_vec(),
>};
>powers_of_g.push(gg);
>```
>
>* * *
>
>## 九、构造 ProverParam（CK）
>
>```rust
>let pp = Self::ProverParam {
>    num_vars,
>    g: g.into_affine(),
>    h: h.into_affine(),
>    powers_of_g,
>};
>```
>
>这就是：
>
>* 多线性 KZG 的 **承诺密钥**
>  
>
>* * *
>
>## 十、生成 Verifier 需要的 `h_mask`
>
>```rust
>let h_mask = {
>    E::G2::normalize_batch(&FixedBase::msm(
>        scalar_bits,
>        window_size,
>        &h_table,
>        &t,
>    ))
>};
>```
>
>数学含义：
>
>$$h\_mask = (h^{t_1}, \dots, h^{t_n})$$
>
>👉 verifier 用于 pairing 校验
>
>* * *
>
>## 十一、最终返回 Universal SRS
>
>```rust
>Ok(Self {
>    prover_param: pp,
>    h_mask,
>})
>```
>
>* * *
>
>## 十二、辅助函数 `remove_dummy_variable`
>
>```rust
>fn remove_dummy_variable<F: Field>(poly: &[F], pad: usize)
>```
>
>### 它干嘛？
>
>> **把一个 MLE 的前 `pad` 个变量固定为 0**
>
>数学上：
>
>$$f(X_1,\dots,X_n) \;\Rightarrow\; f(0,\dots,0,X_{pad+1},\dots)$$
>
>* * *
>
>### 实现方式
>
>```rust
>poly[x << pad]
>```
>
>因为：
>
>* MLE 的 evaluation table 按 bit 编码
>  
>* `x << pad` = 在低位补 `pad` 个 0
>  
>
>* * *
>
>## 十三、把整件事和协议连起来（关键）
>
>```text
>t = (t1,..,tn)
>↓
>eq 多项式族
>↓
>g^{eq(·)} → powers_of_g   ← prover
>h^{t_i}   → h_mask       ← verifier
>```
>
>👉 这正是 **Multilinear KZG + Sumcheck** 能工作的原因
>
>* * *