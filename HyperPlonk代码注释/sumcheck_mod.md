# sumcheck

```rust
use crate::poly_iop::{
    errors::PolyIOPErrors,
    structs::{IOPProof, IOPProverState, IOPVerifierState},
    PolyIOP,
};
use arithmetic::{VPAuxInfo, VirtualPolynomial};
use ark_ff::PrimeField;
use ark_poly::DenseMultilinearExtension;
use ark_std::{end_timer, start_timer};
use std::{fmt::Debug, sync::Arc};
use transcript::IOPTranscript;

mod prover;
mod verifier;

```



```rust
/// 用于执行和检查协议的特性。
pub trait SumCheck<F: PrimeField> {
    type VirtualPolynomial;
    type VPAuxInfo;
    type MultilinearExtension;

    type SumCheckProof: Clone + Debug + Default + PartialEq;
    type Transcript;
    type SumCheckSubClaim: Clone + Debug + Default + PartialEq;

    /// 从证明中提取总和
    fn extract_sum(proof: &Self::SumCheckProof) -> F;

    /// 用一个记录初始化系统
    ///
    /// 此函数为可选函数——当SumCheck是更复杂协议的一个构建块时，
    /// 该记录可由这个复杂协议初始化，并传递给SumCheck证明者/验证者。
    fn init_transcript() -> Self::Transcript;

    /// 生成多项式在{0,1}^`num_vars`上的和的证明
    ///
    /// 该多项式以VirtualPolynomial的形式表示。
    fn prove(
        poly: &Self::VirtualPolynomial,
        transcript: &mut Self::Transcript,
    ) -> Result<Self::SumCheckProof, PolyIOPErrors>;

    /// 用该证明验证所声称的总和
    fn verify(
        sum: F,
        proof: &Self::SumCheckProof,
        aux_info: &Self::VPAuxInfo,
        transcript: &mut Self::Transcript,
    ) -> Result<Self::SumCheckSubClaim, PolyIOPErrors>;
}
```

>## 一、整体说明
>
>```rust
>pub trait SumCheck<F: PrimeField> { ... }
>```
>
>* `SumCheck` 是一个 trait（接口），表示“可以执行 SumCheck 协议的类型集合”。
>  
>* `F: PrimeField` 表示所有操作都在一个素域 `F` 上进行。
>  
>* 这个 trait 并不是具体实现，而是规定了 **SumCheck 协议必须实现的接口**。
>  
>* 核心思想：**用多项式的虚拟表示 VirtualPolynomial 进行 sum-check**。
>  
>
>* * *
>
>## 二、关联类型（Associated Types）
>
>```rust
>type VirtualPolynomial;
>type VPAuxInfo;
>type MultilinearExtension;
>
>type SumCheckProof: Clone + Debug + Default + PartialEq;
>type Transcript;
>type SumCheckSubClaim: Clone + Debug + Default + PartialEq;
>```
>
>解释：
>
>| 类型名                 | 含义                                                       |
>| ---------------------- | ---------------------------------------------------------- |
>| `VirtualPolynomial`    | 虚拟多项式类型（你前面看到的 `VirtualPolynomial<F>`）      |
>| `VPAuxInfo`            | 辅助信息，比如变量数、最大次数等                           |
>| `MultilinearExtension` | 多线性多项式类型（DenseMultilinearExtension）              |
>| `SumCheckProof`        | 证明者生成的 SumCheck 证明，必须支持克隆、打印、默认值比较 |
>| `Transcript`           | 交互式协议的 transcript，用于 Fiat-Shamir 或记录挑战       |
>| `SumCheckSubClaim`     | 验证者在每轮可能生成的子声明，方便递归验证                 |
>
>> 关联类型就是**占位符类型**，具体实现时再指定。
>
>* * *
>
>## 三、核心函数解释
>
>### 1️⃣ extract_sum
>
>```rust
>fn extract_sum(proof: &Self::SumCheckProof) -> F;
>```
>
>* 功能：从 SumCheck 证明中提取出证明声称的 **总和**。
>  
>* 数学上：如果 SumCheck 证明了
>  
>    $$\sum_{x \in \{0,1\}^n} f(x) = S$$
>    
>    那么 `extract_sum` 就返回 `S`。
>    
>
>* * *
>
>### 2️⃣ init_transcript
>
>```rust
>fn init_transcript() -> Self::Transcript;
>```
>
>* 功能：初始化 transcript（记录交互信息）
>  
>* 用途：
>  
>    * 在非交互式 SumCheck 中做 Fiat-Shamir
>      
>    * 或者当 SumCheck 是更复杂协议的一部分时提供初始上下文
>      
>
>* * *
>
>### 3️⃣ prove
>
>```rust
>fn prove(
>    poly: &Self::VirtualPolynomial,
>    transcript: &mut Self::Transcript,
>) -> Result<Self::SumCheckProof, PolyIOPErrors>;
>```
>
>* 功能：证明给定虚拟多项式 `poly` 在 {0,1}^n 上的总和。
>  
>* 输入：
>  
>    * `poly` → 你要 sum-check 的多项式
>      
>    * `transcript` → 记录每轮挑战
>    
>* 输出：
>  
>    * `SumCheckProof` → 证明
>      
>
>> 这是 Prover 执行的核心函数。
>
>* * *
>
>### 4️⃣ verify
>
>```rust
>fn verify(
>    sum: F,
>    proof: &Self::SumCheckProof,
>    aux_info: &Self::VPAuxInfo,
>    transcript: &mut Self::Transcript,
>) -> Result<Self::SumCheckSubClaim, PolyIOPErrors>;
>```
>
>* 功能：验证证明者生成的 SumCheck 证明
>  
>* 输入：
>  
>    * `sum` → 声称的总和
>      
>    * `proof` → Prover 提交的证明
>      
>    * `aux_info` → 多项式的辅助信息（变量数、最大次数）
>      
>    * `transcript` → 交互记录
>    
>* 输出：
>  
>    * `SumCheckSubClaim` → 验证者每轮可能生成的子声明，用于递归或组合验证
>      
>
>* * *
>
>## 四、总结
>
>* 这个 trait 是 **SumCheck 协议的接口**
>  
>* **Prover 和 Verifier 都要实现这个接口**，保证方法统一
>  
>* 核心作用：
>  
>    1. Prover 构造证明 (`prove`)
>      
>    2. Verifier 验证证明 (`verify`)
>      
>    3. 记录交互 (`Transcript`)
>      
>    4. 提取总和 (`extract_sum`)
>    
>* 关联类型让 trait 可以 **适配不同的多项式实现和域类型**。
>  
>





---

```rust
/// 用于和检验协议证明方API的特性。
pub trait SumCheckProver<F: PrimeField>
where
    Self: Sized,
{
    type VirtualPolynomial;
    type ProverMessage;

    ///初始化证明器状态，以证明输入多项式在{0,1}^`num_vars`上的和。
    fn prover_init(polynomial: &Self::VirtualPolynomial) -> Result<Self, PolyIOPErrors>;

    /// 从验证者处接收消息，生成证明者消息，然后继续进行
    /// 下一轮
    ///
    /// 所使用的主要算法来自[XZZPS19]的3.2节（https://eprint.iacr.org/2019/317.pdf#subsection.3.2）。
    fn prove_round_and_update_state(
        &mut self,
        challenge: &Option<F>,
    ) -> Result<Self::ProverMessage, PolyIOPErrors>;
}

```

>## 一、trait 定义
>
>```rust
>pub trait SumCheckProver<F: PrimeField>
>where
>    Self: Sized,
>{ ... }
>```
>
>* `SumCheckProver<F: PrimeField>`：这个 trait 适用于素域 `F` 上的 SumCheck 协议。
>  
>* `Self: Sized`：这个 trait 的实现类型必须是已知大小的（通常所有普通 struct 都满足）。
>  
>* trait 的作用：**定义证明者必须提供的方法**，保证 Prover 可以参与 SumCheck 协议。
>  
>
>* * *
>
>## 二、关联类型（Associated Types）
>
>```rust
>type VirtualPolynomial;
>type ProverMessage;
>```
>
>* `VirtualPolynomial`：证明者要证明的多项式类型（之前你看到的 `VirtualPolynomial<F>`）。
>  
>* `ProverMessage`：证明者每轮发送给验证者的消息类型（例如一组评估值、折叠信息等）。
>  
>
>* * *
>
>## 三、核心方法解析
>
>### 1️⃣ `prover_init`
>
>```rust
>fn prover_init(polynomial: &Self::VirtualPolynomial) -> Result<Self, PolyIOPErrors>;
>```
>
>* 功能：**初始化证明者状态**
>  
>* 输入：
>  
>    * `polynomial` → 待证明的虚拟多项式
>    
>* 输出：
>  
>    * 返回一个初始化好的 `Self`（实现 trait 的 Prover struct）
>    
>* 作用：
>  
>    * 设置初始状态，例如：
>      
>        * 当前轮数 `round = 0`
>          
>        * 存储虚拟多项式
>          
>        * 初始化辅助信息（max_degree、num_variables 等）
>    
>* 对应数学意义：
>  
>    * 证明者要准备好在 {0,1}^n 上逐轮执行 sum-check，`prover_init` 就是这个准备阶段。
>      
>
>* * *
>
>### 2️⃣ `prove_round_and_update_state`
>
>```rust
>fn prove_round_and_update_state(
>    &mut self,
>    challenge: &Option<F>,
>) -> Result<Self::ProverMessage, PolyIOPErrors>;
>```
>
>* 功能：
>  
>    1. **接收验证者的挑战（challenge）**
>      
>    2. **生成本轮证明者消息（ProverMessage）**
>      
>    3. **更新证明者状态，为下一轮做准备**
>    
>* 输入：
>  
>    * `challenge: &Option<F>`
>      
>        * 对应 sum-check 协议中 Verifier 发送的随机值
>          
>        * 第一轮可以是 `None`（没有前一轮挑战）
>    
>* 输出：
>  
>    * `ProverMessage` → 当前轮发送给 Verifier 的消息
>    
>* 内部做了什么：
>  
>    1. 把 challenge 应用到虚拟多项式（固定变量 / 折叠多项式）
>      
>    2. 计算下一轮所需的评估值
>      
>    3. 更新内部状态（轮次 `round += 1`，更新部分折叠多项式等）
>    
>* 数学背景：
>  
>    * 对应 [XZZPS19] 论文 3.2 节
>      
>        * 每轮固定一个变量
>          
>        * 计算部分和
>          
>        * 生成多项式的单变量评估结果给 Verifier
>          
>
>* * *
>
>## 四、整体协议流程（Prover 侧）
>
>1. **初始化**
>  
>
>```rust
>let mut prover = ProverType::prover_init(&poly)?;
>```
>
>* 内部状态：
>  
>    * `round = 0`
>      
>    * 保存 `poly` 等信息
>      
>
>2. **每轮交互**
>  
>
>```rust
>let message = prover.prove_round_and_update_state(&challenge)?;
>```
>
>* 内部操作：
>  
>    1. 根据上一轮 challenge 或 `None` 折叠多项式
>      
>    2. 生成当前轮的 ProverMessage
>      
>    3. 更新 `round`，准备下一轮
>      
>
>3. **重复直到最后一轮**
>  
>
>* 最终 `ProverMessage` 包含完整的 sum-check 证明信息



```rust
/// 求和检查协议验证器端API的特性。
pub trait SumCheckVerifier<F: PrimeField> {
    type VPAuxInfo;
    type ProverMessage;
    type Challenge;
    type Transcript;
    type SumCheckSubClaim;

    /// 初始化验证者状态
    fn verifier_init(index_info: &Self::VPAuxInfo) -> Self;

   	/// 给定证明者消息，为当前轮次运行验证器。
    ///
    /// 请注意，`verify_round_and_update_state` 仅采样并存储
    /// 挑战；并相应地更新验证器的状态。实际的
    /// 验证会推迟（批量进行）到最后一步的 `check_and_generate_subclaim`。
    fn verify_round_and_update_state(
        &mut self,
        prover_msg: &Self::ProverMessage,
        transcript: &mut Self::Transcript,
    ) -> Result<Self::Challenge, PolyIOPErrors>;

    /// 此函数验证协议交互版本中的延迟检查，并生成子声明。如果证明验证失败，则返回错误。
    ///
    /// 如果断言的总和正确，那么在`subclaim.point`处计算的多线性多项式将为`subclaim.expected_evaluation`。
    /// 否则，这两个值相等的可能性极低。
    /// 更大的域大小可保证更小的可靠性错误。
    fn check_and_generate_subclaim(
        &self,
        asserted_sum: &F,
    ) -> Result<Self::SumCheckSubClaim, PolyIOPErrors>;
}
```

>## 一、trait 定义
>
>```rust
>pub trait SumCheckVerifier<F: PrimeField> { ... }
>```
>
>* `SumCheckVerifier<F: PrimeField>`：这个 trait 表示在素域 `F` 上进行 SumCheck 协议的验证者接口。
>
>* 它定义了验证者必须实现的方法，保证验证者可以正确和 Prover 交互。
>
>
>* * *
>
>## 二、关联类型（Associated Types）
>
>```rust
>type VPAuxInfo;
>type ProverMessage;
>type Challenge;
>type Transcript;
>type SumCheckSubClaim;
>```
>
>解释：
>
>| 类型               | 含义                                                     |
>| ------------------ | -------------------------------------------------------- |
>| `VPAuxInfo`        | 虚拟多项式的辅助信息（变量数、最大次数等）               |
>| `ProverMessage`    | 每轮由 Prover 发送给验证者的消息                         |
>| `Challenge`        | 验证者在每轮生成的随机挑战值（单个 field 元素）          |
>| `Transcript`       | 记录交互信息的对象（Fiat-Shamir 或轮次信息）             |
>| `SumCheckSubClaim` | 验证者延迟生成的子声明，用于最后验证多项式部分和是否正确 |
>
>* * *
>
>## 三、核心方法解析
>
>### 1️⃣ `verifier_init`
>
>```rust
>fn verifier_init(index_info: &Self::VPAuxInfo) -> Self;
>```
>
>* 功能：初始化验证者状态
>
>* 输入：
>
>   * `index_info` → 多项式的辅助信息
>
>* 输出：
>
>   * 返回一个初始化好的验证者 struct
>
>* 内部可能初始化：
>
>   * 当前轮 `round = 0`
>
>   * 存储多项式变量数、最大次数
>
>   * 初始化 transcript
>
>
>* * *
>
>### 2️⃣ `verify_round_and_update_state`
>
>```rust
>fn verify_round_and_update_state(
>   &mut self,
>   prover_msg: &Self::ProverMessage,
>   transcript: &mut Self::Transcript,
>) -> Result<Self::Challenge, PolyIOPErrors>;
>```
>
>* 功能：
>
>   1. **接收 Prover 当前轮的消息**
>
>   2. **生成本轮 challenge**（随机 field 元素）
>
>   3. **更新内部状态**
>
>* 注意：
>
>   * **实际的验证并不在这里完成**
>
>       * 只是采样 challenge 并记录状态
>
>       * 最终验证延迟到 `check_and_generate_subclaim`
>
>* 数学背景：
>
>   * SumCheck 协议每轮：
>
>       1. Prover 发送部分和多项式
>
>       2. Verifier 采样随机 challenge
>
>           $$r \in \mathbb{F}_q$$
>
>           用于固定下一轮变量
>
>
>* * *
>
>### 3️⃣ `check_and_generate_subclaim`
>
>```rust
>fn check_and_generate_subclaim(
>   &self,
>   asserted_sum: &F,
>) -> Result<Self::SumCheckSubClaim, PolyIOPErrors>;
>```
>
>* 功能：
>
>   1. 对之前延迟的验证操作进行检查
>
>   2. 生成一个子声明（SubClaim），包括：
>
>       * 某个点 `point`
>
>       * 多项式在该点的期望值 `expected_evaluation`
>
>   3. 如果 Prover 作弊，检查失败（返回错误）
>
>* 数学背景：
>
>   * 延迟检查是 SumCheck 的标准优化：
>
>       * 不每轮都验证所有计算，而是最后一次性检查
>
>           $$f(\text{sampled point}) \stackrel{?}{=} \text{expected value}$$
>* 提高效率同时保证安全性（概率上，作弊被发现的概率很高）。
>
>
>* * *
>
>## 四、整体验证者工作流程
>
>假设 Prover 要证明多项式 $f(x_1, x_2, x_3)$ 在 {0,1}^3 上的总和：
>
>1. **初始化验证者**
>
>
>```rust
>let mut verifier = VerifierType::verifier_init(&aux_info);
>```
>
>2. **每轮交互**
>
>
>```rust
>for round in 0..3 {
>   let challenge = verifier.verify_round_and_update_state(&prover_msg, &mut transcript)?;
>   // challenge 发送给 Prover，用于下一轮折叠多项式
>}
>```
>
>3. **最终延迟检查**
>
>
>```rust
>let subclaim = verifier.check_and_generate_subclaim(&asserted_sum)?;
>```
>
>* `subclaim.point` → 验证器随机选择的点
>
>* `subclaim.expected_evaluation` → 多项式在该点的正确值
>
>* 验证通过则证明总和正确，否则失败
>



```rust
/// SumCheckSubClaim是验证者在确信时于验证结束时生成的声明。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SumCheckSubClaim<F: PrimeField> {
    /// 这个多线性延拓所评估的多维点
    /// to
    pub point: Vec<F>,
    /// 预期的评估
    pub expected_evaluation: F,
}
```

>### 1️⃣ `pub point: Vec<F>`
>
>```rust
>pub point: Vec<F>,
>```
>
>#### 是什么？
>
>* 一个 **随机点**
>  
>* 维度 = 原多项式变量数
>  
>* 属于域 `F`
>  
>
>数学上是：
>
>$$\mathbf{r} = (r_1, r_2, \dots, r_n) \in F^n$$
>
>#### 在协议中怎么来的？
>
>* 每一轮验证者采样一个随机挑战
>  
>* 最终把所有挑战拼起来
>  
>
>```text
>round 1 → r1
>round 2 → r2
>...
>round n → rn
>```
>
>* * *
>
>### 2️⃣ `pub expected_evaluation: F`
>
>```rust
>pub expected_evaluation: F,
>```
>
>#### 是什么？
>
>* 验证者根据 **证明者发的多项式 + 随机挑战**
>  
>* 自己算出来的一个值
>  
>
>数学上是：
>
>$$v = f(r_1, r_2, \dots, r_n)$$
>
>⚠️ 但注意：
>
>> 验证者 **并不知道 f 的全部表达式**  
>> 这个值是通过 Sum-Check 的一致性递推得到的





```rust
impl<F: PrimeField> SumCheck<F> for PolyIOP<F> {
    type SumCheckProof = IOPProof<F>;
    type VirtualPolynomial = VirtualPolynomial<F>;
    type VPAuxInfo = VPAuxInfo<F>;
    type MultilinearExtension = Arc<DenseMultilinearExtension<F>>;
    type SumCheckSubClaim = SumCheckSubClaim<F>;
    type Transcript = IOPTranscript<F>;
```

>## 一、这段代码一句话在干嘛？
>
>```rust
>impl<F: PrimeField> SumCheck<F> for PolyIOP<F> {
>```
>
>👉 **意思是：**
>
>> **`PolyIOP<F>` 这个类型，实现了 `SumCheck<F>` 这个协议接口**
>
>换句话说：
>
>> “我这个 `PolyIOP`，就是一个完整的 Sum-Check 协议实现。”
>
>* * *
>
>## 二、先回忆 `SumCheck` trait 要求什么？
>
>你之前贴过：
>
>```rust
>pub trait SumCheck<F: PrimeField> {
>    type VirtualPolynomial;
>    type VPAuxInfo;
>    type MultilinearExtension;
>
>    type SumCheckProof;
>    type Transcript;
>    type SumCheckSubClaim;
>
>    fn extract_sum(...)
>    fn init_transcript(...)
>    fn prove(...)
>    fn verify(...)
>}
>```
>
>👉 **trait = 协议规范（接口）**  
>👉 **impl = 具体协议实例**
>
>* * *
>
>## 三、逐行解释类型绑定（这是核心）
>
>```rust
>impl<F: PrimeField> SumCheck<F> for PolyIOP<F> {
>```
>
>* `F: PrimeField`  
>    👉 底层域（如 BN254、BLS12-381）
>    
>* `PolyIOP<F>`  
>    👉 一个 **多项式 IOP 框架**
>    
>* `for PolyIOP<F>`  
>    👉 PolyIOP 是 Sum-Check 的“载体”
>    
>
>* * *
>
>### 1️⃣ `type SumCheckProof = IOPProof<F>;`
>
>```rust
>type SumCheckProof = IOPProof<F>;
>```
>
>#### 含义
>
>👉 **Sum-Check 协议中“完整证明”的数据结构**
>
>* 包含：
>  
>    * 每一轮 prover 发的消息
>      
>    * 最终信息
>      
>
>你可以理解为：
>
>```text
>IOPProof =
>  [
>    prover_msg_round_1,
>    prover_msg_round_2,
>    ...
>    prover_msg_round_n
>  ]
>```
>
>* * *
>
>### 2️⃣ `type VirtualPolynomial = VirtualPolynomial<F>;`
>
>```rust
>type VirtualPolynomial = VirtualPolynomial<F>;
>```
>
>#### 含义（非常重要）
>
>👉 **Sum-Check 操作的“多项式对象”**
>
>但它不是普通多项式，而是：
>
>* **由多个 MLE 乘积 + 系数组合而成**
>  
>* **不显式展开**
>  
>* **惰性表示**
>  
>
>你前面刚分析过：
>
>```rust
>pub struct VirtualPolynomial<F> {
>    products: Vec<(F, Vec<usize>)>,
>    flattened_ml_extensions: Vec<Arc<DenseMultilinearExtension<F>>>,
>}
>```
>
>👉 **Sum-Check 实际证明的是这个东西**
>
>* * *
>
>### 3️⃣ `type VPAuxInfo = VPAuxInfo<F>;`
>
>```rust
>type VPAuxInfo = VPAuxInfo<F>;
>```
>
>#### 含义
>
>👉 **验证者所需的“公开辅助信息”**
>
>通常包括：
>
>* `num_variables`（变量个数）
>  
>* `max_degree`（最高乘积度）
>  
>* 结构信息（不含秘密）
>  
>
>💡 你可以理解为：
>
>> “验证者不用知道多项式值，但要知道它长什么样”
>
>* * *
>
>### 4️⃣ `type MultilinearExtension = Arc<DenseMultilinearExtension<F>>;`
>
>```rust
>type MultilinearExtension = Arc<DenseMultilinearExtension<F>>;
>```
>
>#### 含义
>
>👉 **Sum-Check 里使用的“基础函数单元”**
>
>* 一个 MLE：
>  
>    $$f : \{0,1\}^n \to F$$
>* 用 `Arc`：
>  
>    * 避免拷贝
>      
>    * 多个乘积共享同一个 MLE
>      
>
>这和你之前看到的：
>
>```rust
>mle_list: impl IntoIterator<Item = Arc<DenseMultilinearExtension<F>>>
>```
>
>是 **完全对上的**。
>
>* * *
>
>### 5️⃣ `type SumCheckSubClaim = SumCheckSubClaim<F>;`
>
>```rust
>type SumCheckSubClaim = SumCheckSubClaim<F>;
>```
>
>#### 含义
>
>👉 **Sum-Check 验证结束后的“子声明”**
>
>你刚刚已经完全搞懂了：
>
>```rust
>SumCheckSubClaim {
>    point: Vec<F>,
>    expected_evaluation: F,
>}
>```
>
>这就是：
>
>> “在随机点 `r` 上，多项式值应为 `v`”
>
>* * *
>
>### 6️⃣ `type Transcript = IOPTranscript<F>;`
>
>```rust
>type Transcript = IOPTranscript<F>;
>```
>
>#### 含义
>
>👉 **Fiat–Shamir 记录器**
>
>负责：
>
>* 吸收 prover 消息
>  
>* 生成 verifier 挑战
>  
>* 保证非交互安全性
>  
>
>可以理解为：
>
>```text
>Transcript = Hash(state || prover_message)
>```
>
>* * *
>
>## 四、整体“翻译成一句人话”
>
>```text
>PolyIOP<F> 声明：
>
>- 我使用 VirtualPolynomial<F> 作为 Sum-Check 的输入多项式
>- 我的证明格式是 IOPProof<F>
>- 我的基础多项式是 Arc<DenseMultilinearExtension<F>>
>- 验证结束后输出 SumCheckSubClaim<F>
>- 随机性由 IOPTranscript<F> 提供
>
>所以：
>👉 PolyIOP<F> 是一个完整的 Sum-Check 协议实现
>```
>





```rust
  fn extract_sum(proof: &Self::SumCheckProof) -> F {
        let start = start_timer!(|| "extract sum"); //SumCheckProof=IOPProof<F>
        let res = proof.proofs[0].evaluations[0] + proof.proofs[0].evaluations[1];
        end_timer!(start);
        res
    }

    fn init_transcript() -> Self::Transcript {
        let start = start_timer!(|| "init transcript");
        let res = IOPTranscript::<F>::new(b"Initializing SumCheck transcript");
        end_timer!(start);
        res
    }
```

>```rust
>pub struct IOPProof<F: PrimeField> {
>pub point: Vec<F>,
>pub proofs: Vec<IOPProverMessage<F>>,
>}
>
>```
>
>```rust
>#[derive(Clone, Debug, Default, PartialEq, Eq, CanonicalSerialize)]
>pub struct IOPProverMessage<F: PrimeField> {
>pub(crate) evaluations: Vec<F>,
>}
>```
>
>## 一、第一段：`extract_sum`
>
>```rust
>fn extract_sum(proof: &Self::SumCheckProof) -> F {
>let start = start_timer!(|| "extract sum");
>let res = proof.proofs[0].evaluations[0] + proof.proofs[0].evaluations[1];
>end_timer!(start);
>res
>}
>```
>
>### 1️⃣ 这个函数是干嘛的？
>
>👉 **从 Sum-Check 的证明中，提取“声称的总和”**
>
>在 Sum-Check 协议中，证明者一开始就声明：
>
>$$S = \sum_{x \in \{0,1\}^n} f(x)$$
>
>这个 `S` **并不是单独存的**，而是隐含在：
>
>> **第一轮 prover 发的多项式的两个端点值里**
>
>* * *
>
>## 二、理解这句核心代码（非常重要）
>
>```rust
>proof.proofs[0].evaluations[0] + proof.proofs[0].evaluations[1]
>```
>
>我们拆开来看：
>
>### `proof`
>
>```text
>IOPProof<F>
>```
>
>里面一般是：
>
>```rust
>pub struct IOPProof<F: PrimeField> {
>pub point: Vec<F>,
>pub proofs: Vec<IOPProverMessage<F>>,
>}
>```
>
>* * *
>
>### `proof.proofs[0]`
>
>👉 **Sum-Check 第 1 轮的 prover 消息**
>
>第 1 轮 prover 会发送一个 **一元多项式**：
>
>$$g_1(X)$$
>
>* * *
>
>### `evaluations`
>
>在 Sum-Check 中：
>
>```text
>prover 不发送整个 g_1(X)
>而是发送：
>  g_1(0), g_1(1)
>```
>
>所以：
>
>```rust
>proof.proofs[0].evaluations = [ g1(0), g1(1) ]
>```
>
>* * *
>
>### 为什么要相加？
>
>Sum-Check 的**第一轮正确性条件**是：
>
>$$g_1(0) + g_1(1) = \sum_{x \in \{0,1\}^n} f(x)$$
>
>📌 **所以：**
>
>```rust
>claimed_sum = g1(0) + g1(1)
>```
>
>这正是：
>
>```rust
>let res = evaluations[0] + evaluations[1];
>```
>
>* * *
>
>### 小例子（非常直观）
>
>假设：
>
>```text
>f(x1, x2) = x1 + x2
>```
>
>那么：
>
>```text
>f(0,0)=0
>f(0,1)=1
>f(1,0)=1
>f(1,1)=2
>```
>
>总和：
>
>```text
>S = 4
>```
>
>Sum-Check 第 1 轮 prover 构造：
>
>```text
>g1(X) = ∑_{x2 ∈ {0,1}} f(X, x2)
>```
>
>```text
>g1(0) = f(0,0)+f(0,1)=1
>g1(1) = f(1,0)+f(1,1)=3
>```
>
>所以：
>
>```rust
>extract_sum = 1 + 3 = 4
>```
>
>✔️ 完全正确
>
>* * *
>
>### 计时宏（不用太在意）
>
>```rust
>start_timer!(|| "extract sum");
>end_timer!(start);
>```
>
>👉 ark-std 的性能统计  
>👉 不影响逻辑，可忽略
>
>* * *
>
>## 三、第二段：`init_transcript`
>
>```rust
>fn init_transcript() -> Self::Transcript {
>  let start = start_timer!(|| "init transcript");
>  let res = IOPTranscript::<F>::new(b"Initializing SumCheck transcript");
>  end_timer!(start);
>  res
>}
>```
>
>* * *
>
>### 1️⃣ 这个函数是干嘛的？
>
>👉 **初始化 Fiat–Shamir transcript**
>
>换句话说：
>
>> “给 Sum-Check 协议准备一个‘随机性发生器’”
>
>* * *
>
>## 四、什么是 Transcript（用人话）
>
>在交互式 Sum-Check 中：
>
>```text
>Verifier: 给我一个随机挑战 r
>Prover:   用 r 继续下一轮
>```
>
>但在**非交互（IOP）**里：
>
>```text
>challenge = Hash(之前所有 prover 消息)
>```
>
>这个“Hash 记账本”就是：
>
>```rust
>IOPTranscript<F>
>```
>
>* * *
>
>### 2️⃣ 这一行在干嘛？
>
>```rust
>IOPTranscript::<F>::new(b"Initializing SumCheck transcript");
>```
>
>* 创建一个新的 transcript
>
>* 初始状态里写入一个固定字符串（domain separation）
>
>
>👉 防止不同协议复用随机性（非常重要的密码学细节）
>
>* * *
>
>### 3️⃣ 为什么用 `b"..."`？
>
>```rust
>b"Initializing SumCheck transcript"
>```
>
>* `b"..."` 是 **字节串**
>
>* transcript 吃的是字节，不是字符串
>
>



```rust
 fn prove(
        poly: &Self::VirtualPolynomial,
        transcript: &mut Self::Transcript,
    ) -> Result<Self::SumCheckProof, PolyIOPErrors> {
        let start = start_timer!(|| "sum check prove");

        transcript.append_serializable_element(b"aux info", &poly.aux_info)?;

        let mut prover_state = IOPProverState::prover_init(poly)?;
        let mut challenge = None;
        let mut prover_msgs = Vec::with_capacity(poly.aux_info.num_variables);
        for _ in 0..poly.aux_info.num_variables {
            let prover_msg =
                IOPProverState::prove_round_and_update_state(&mut prover_state, &challenge)?;
            transcript.append_serializable_element(b"prover msg", &prover_msg)?;
            prover_msgs.push(prover_msg);
            challenge = Some(transcript.get_and_append_challenge(b"Internal round")?);
        }
        // pushing the last challenge point to the state
        if let Some(p) = challenge {
            prover_state.challenges.push(p)
        };

        end_timer!(start);
        Ok(IOPProof {
            point: prover_state.challenges,
            proofs: prover_msgs,
        })
    }
```

>### 一、先给你一个“全局直觉”
>
>这段代码做的事情可以一句话概括：
>
>> **对一个 VirtualPolynomial，在 {0,1}ⁿ 上的求和，生成一份非交互式 Sum-Check 证明**
>
>流程是：
>
>```
>Prover:
>  初始化 prover state
>  for 每一个变量 xi:
>      构造一元多项式 gi(xi)
>      发送 gi(0), gi(1)
>      从 transcript 派生随机挑战 ri
>      固定变量 xi = ri（折叠多项式）
>  输出：
>      - 所有轮次的 prover 消息
>      - 所有挑战点 r1,...,rn
>```
>
>* * *
>
>### 二、函数签名（语义先行）
>
>```rust
>fn prove(
>    poly: &Self::VirtualPolynomial,
>    transcript: &mut Self::Transcript,
>) -> Result<Self::SumCheckProof, PolyIOPErrors>
>```
>
>含义：
>
>| 参数         | 含义                                              |
>| ------------ | ------------------------------------------------- |
>| `poly`       | 要证明的多变量多项式（用 VirtualPolynomial 表示） |
>| `transcript` | Fiat–Shamir transcript                            |
>| 返回值       | Sum-Check 的完整证明                              |
>
>* * *
>
>### 三、逐行 + 协议语义解释
>
>* * *
>
>## 1️⃣ 开始计时（可忽略）
>
>```rust
>let start = start_timer!(|| "sum check prove");
>```
>
>* * *
>
>## 2️⃣ 把多项式的辅助信息写进 transcript
>
>```rust
>transcript.append_serializable_element(b"aux info", &poly.aux_info)?;
>```
>
>### 为什么要这么做？
>
>`aux_info` 包含：
>
>```text
>- num_variables
>- max_degree
>```
>
>👉 这是 **协议公共输入的一部分**
>
>### 密码学意义：
>
>> 防止 prover 在不同参数下“复用”证明
>
>* * *
>
>## 3️⃣ 初始化证明者状态
>
>```rust
>let mut prover_state = IOPProverState::prover_init(poly)?;
>```
>
>### prover_state 里有什么？
>
>你之前已经看过：
>
>```rust
>IOPProverState {
>    round: 0,
>    challenges: [],
>    poly: VirtualPolynomial,
>    ...
>}
>```
>
>📌 此时：
>
>```text
>poly = 原始 n 元多项式
>round = 0
>challenges = []
>```
>
>* * *
>
>## 4️⃣ 初始化挑战 & 消息容器
>
>```rust
>let mut challenge = None;
>let mut prover_msgs = Vec::with_capacity(poly.aux_info.num_variables);
>```
>
>### 为什么 `challenge = None`？
>
>* 第 0 轮 **没有 challenge**
>  
>* 第 1 个 challenge 是在 prover 发完第 1 条消息后才生成的
>  
>
>这正是 Sum-Check 的标准流程。
>
>* * *
>
>## 5️⃣ 核心循环：每个变量一轮
>
>```rust
>for _ in 0..poly.aux_info.num_variables {
>```
>
>如果多项式有 `n` 个变量：
>
>👉 **Sum-Check 就跑 `n` 轮**
>
>* * *
>
>### 🔁 每一轮做什么？
>
>* * *
>
>### (a) 生成本轮 prover 消息
>
>```rust
>let prover_msg =
>    IOPProverState::prove_round_and_update_state(&mut prover_state, &challenge)?;
>```
>
>#### 协议层面发生了什么？
>
>假设当前是第 `i` 轮：
>
>* prover 从当前多项式 `f_i(x_i, ..., x_n)` 构造：
>  
>
>$$g_i(X) = \sum_{x_{i+1},...,x_n \in \{0,1\}} f_i(X, x_{i+1},...,x_n)$$
>
>* 然后发送：
>  
>
>```text
>g_i(0), g_i(1)
>```
>
>📌 这就是 `prover_msg`
>
>* * *
>
>### (b) 写入 transcript（Fiat–Shamir）
>
>```rust
>transcript.append_serializable_element(b"prover msg", &prover_msg)?;
>```
>
>意义：
>
>> 把 prover 的承诺写入 transcript，用于派生挑战
>
>* * *
>
>### (c) 保存消息
>
>```rust
>prover_msgs.push(prover_msg);
>```
>
>最后会组成：
>
>```rust
>IOPProof {
>    proofs: Vec<IOPProverMessage>
>}
>```
>
>* * *
>
>### (d) 生成 verifier 的挑战（非交互）
>
>```rust
>challenge = Some(
>    transcript.get_and_append_challenge(b"Internal round")?
>);
>```
>
>📌 等价于：
>
>```text
>r_i = Hash(transcript_so_far)
>```
>
>这是 **Fiat–Shamir 变换**
>
>* * *
>
>### (e) 下一轮会发生什么？
>
>在下一次 `prove_round_and_update_state` 中：
>
>```text
>poly ← poly(x_i = r_i)
>round += 1
>```
>
>👉 多项式变量数减少 1
>
>* * *
>
>## 6️⃣ 把最后一个 challenge 也记下来
>
>```rust
>if let Some(p) = challenge {
>    prover_state.challenges.push(p)
>};
>```
>
>### 为什么要单独 push？
>
>因为：
>
>* 每一轮生成 challenge 是在 loop **末尾**
>  
>* 最后一轮生成的 challenge 没机会被“用来触发下一轮”
>  
>
>但它仍然是 **subclaim 的评估点**
>
>* * *
>
>## 7️⃣ 构造最终证明
>
>```rust
>Ok(IOPProof {
>    point: prover_state.challenges,
>    proofs: prover_msgs,
>})
>```
>
>### 含义：
>
>```text
>point  = [r1, r2, ..., rn]
>proofs = [
>    (g1(0), g1(1)),
>    (g2(0), g2(1)),
>    ...
>]
>```
>





```rust
    fn verify(
        claimed_sum: F,
        proof: &Self::SumCheckProof,
        aux_info: &Self::VPAuxInfo,
        transcript: &mut Self::Transcript,
    ) -> Result<Self::SumCheckSubClaim, PolyIOPErrors> {
        let start = start_timer!(|| "sum check verify");

        transcript.append_serializable_element(b"aux info", aux_info)?;
        let mut verifier_state = IOPVerifierState::verifier_init(aux_info);
        for i in 0..aux_info.num_variables {
            let prover_msg = proof.proofs.get(i).expect("proof is incomplete");
            transcript.append_serializable_element(b"prover msg", prover_msg)?;
            IOPVerifierState::verify_round_and_update_state(
                &mut verifier_state,
                prover_msg,
                transcript,
            )?;
        }

        let res = IOPVerifierState::check_and_generate_subclaim(&verifier_state, &claimed_sum);

        end_timer!(start);
        res
    }
}
```

>### 一、一句话总览（先给直觉）
>
>> 这段 `verify` 做的事情是：
>>
>> **逐轮重放 Sum-Check 的验证流程，检查一致性，最后生成一个子声明（subclaim）**
>
>也就是：
>
>```
>Verifier:
>  初始化状态
>  for 每一轮 i:
>      接收 prover 的 gi
>      用 transcript 派生随机 ri
>      检查 gi 的一致性约束
>  最后：
>      检查 claimed_sum 是否一致
>      输出 subclaim (r1,...,rn, f(r))
>```
>
>* * *
>
>### 二、函数签名（角色说明）
>
>```rust
>fn verify(
>    claimed_sum: F,
>    proof: &Self::SumCheckProof,
>    aux_info: &Self::VPAuxInfo,
>    transcript: &mut Self::Transcript,
>) -> Result<Self::SumCheckSubClaim, PolyIOPErrors>
>```
>
>| 参数          | 含义                        |
>| ------------- | --------------------------- |
>| `claimed_sum` | prover 声称的 ∑ f(x)        |
>| `proof`       | prover 给出的 SumCheck 证明 |
>| `aux_info`    | 多项式的公共结构信息        |
>| `transcript`  | Fiat–Shamir transcript      |
>| 返回值        | `SumCheckSubClaim`          |
>
>* * *
>
>### 三、逐行 + 协议语义解释
>
>* * *
>
>## 1️⃣ 开始计时
>
>```rust
>let start = start_timer!(|| "sum check verify");
>```
>
>纯性能统计，可忽略。
>
>* * *
>
>## 2️⃣ 将 aux_info 写入 transcript
>
>```rust
>transcript.append_serializable_element(b"aux info", aux_info)?;
>```
>
>### 为什么验证者也要 append？
>
>👉 **Fiat–Shamir 的对称性**
>
>验证者必须：
>
>* 使用 **与 prover 完全相同的 transcript 内容**
>  
>* 才能派生出 **相同的随机挑战**
>  
>
>否则挑战不一致，验证直接失败。
>
>* * *
>
>## 3️⃣ 初始化验证者状态
>
>```rust
>let mut verifier_state = IOPVerifierState::verifier_init(aux_info);
>```
>
>### verifier_state 里有什么？
>
>```rust
>IOPVerifierState {
>    round: 0,
>    num_vars,
>    max_degree,
>    polynomials_received: [],
>    challenges: [],
>    finished: false,
>}
>```
>
>📌 此时验证者什么都没检查，只是“准备好状态”。
>
>* * *
>
>## 4️⃣ 逐轮验证（核心）
>
>```rust
>for i in 0..aux_info.num_variables {
>```
>
>Sum-Check 有 `n` 个变量 ⇒ `n` 轮验证。
>
>* * *
>
>### 🔁 每一轮做什么？
>
>* * *
>
>### (a) 取出 prover 的第 i 轮消息
>
>```rust
>let prover_msg = proof.proofs.get(i).expect("proof is incomplete");
>```
>
>`prover_msg` 通常包含：
>
>```text
>[g_i(0), g_i(1)]
>```
>
>* * *
>
>### (b) 写入 transcript
>
>```rust
>transcript.append_serializable_element(b"prover msg", prover_msg)?;
>```
>
>📌 保证：
>
>```text
>Verifier 派生的 ri == Prover 使用的 ri
>```
>
>* * *
>
>### (c) 验证当前轮 & 派生挑战
>
>```rust
>IOPVerifierState::verify_round_and_update_state(
>    &mut verifier_state,
>    prover_msg,
>    transcript,
>)?;
>```
>
>这是**验证端最关键的逻辑**，内部做了三件事：
>
>* * *
>
>#### ✅ 1. 检查一致性约束
>
>对于第 i 轮：
>
>$$g_i(0) + g_i(1) \stackrel{?}{=} g_{i-1}(r_{i-1})$$
>
>第 1 轮则检查：
>
>$$g_1(0) + g_1(1) \stackrel{?}{=} claimed\_sum$$
>
>* * *
>
>#### 🎲 2. 派生随机挑战
>
>```text
>r_i = Hash(transcript)
>```
>
>* * *
>
>#### 📦 3. 更新状态
>
>```text
>round += 1
>polynomials_received.push(g_i)
>challenges.push(r_i)
>```
>
>⚠️ 注意：  
>**这里只存信息，不做最终判断**
>
>* * *
>
>## 5️⃣ 延迟检查 + 生成子声明
>
>```rust
>let res =
>    IOPVerifierState::check_and_generate_subclaim(&verifier_state, &claimed_sum);
>```
>
>这是 **Sum-Check 的最后一步**
>
>* * *
>
>### 这一步做什么？
>
>#### ✅ 1. 检查所有轮是否通过
>
>* 每一轮的一致性关系
>  
>* 多项式次数是否合法
>  
>
>#### 🎯 2. 生成 SubClaim
>
>```rust
>SumCheckSubClaim {
>    point: [r1, r2, ..., rn],
>    expected_evaluation: f(r1, r2, ..., rn)
>}
>```
>
>📌 这就是：
>
>> **“如果你能证明 f 在这个点的值是 expected_evaluation，那么 ∑ f(x) 是对的”**
>
>* * *
>
>## 6️⃣ 结束计时并返回
>
>```rust
>end_timer!(start);
>res
>```
>
>* * *
>
>### 四、把 verify 和 prove 对齐起来看
>
>| Prover        | Verifier      |
>| ------------- | ------------- |
>| 构造 gi       | 接收 gi       |
>| 写 gi         | 写 gi         |
>| 生成 ri       | 生成 ri       |
>| 折叠多项式    | 检查一致性    |
>| 输出 (gi, ri) | 输出 subclaim |
>



```rust
#[cfg(test)]
mod test {

    use super::*;
    use ark_bls12_381::Fr;
    use ark_ff::UniformRand;
    use ark_poly::{DenseMultilinearExtension, MultilinearExtension};
    use ark_std::test_rng;
    use std::sync::Arc;

    fn test_sumcheck(
        nv: usize,
        num_multiplicands_range: (usize, usize),
        num_products: usize,
    ) -> Result<(), PolyIOPErrors> {
        let mut rng = test_rng();
        let mut transcript = <PolyIOP<Fr> as SumCheck<Fr>>::init_transcript();

        let (poly, asserted_sum) =
            VirtualPolynomial::rand(nv, num_multiplicands_range, num_products, &mut rng)?;
        let proof = <PolyIOP<Fr> as SumCheck<Fr>>::prove(&poly, &mut transcript)?;
        let poly_info = poly.aux_info.clone();
        let mut transcript = <PolyIOP<Fr> as SumCheck<Fr>>::init_transcript();
        let subclaim = <PolyIOP<Fr> as SumCheck<Fr>>::verify(
            asserted_sum,
            &proof,
            &poly_info,
            &mut transcript,
        )?;
        assert!(
            poly.evaluate(&subclaim.point).unwrap() == subclaim.expected_evaluation,
            "wrong subclaim"
        );
        Ok(())
    }
```





```rust
 #[test]
    fn test_trivial_polynomial() -> Result<(), PolyIOPErrors> {
        let nv = 1;
        let num_multiplicands_range = (4, 13);
        let num_products = 5;

        test_sumcheck(nv, num_multiplicands_range, num_products)?;
        test_sumcheck_internal(nv, num_multiplicands_range, num_products)
    }
```

>### 一、整体一句话
>
>这个 `test_sumcheck` 在做的事是：
>
>> **随机生成一个 VirtualPolynomial → 运行 SumCheck → 验证 → 检查子声明是否真的成立**
>
>这是 **Sum-Check 协议正确性的最终判据**。
>
>* * *
>
>### 二、测试模块结构
>
>```rust
>#[cfg(test)]
>mod test {
>```
>
>* 只有 `cargo test` 时才编译
>  
>* 不影响正式库代码
>  
>
>* * *
>
>## 引入依赖
>
>```rust
>use super::*;
>```
>
>* 引入当前模块的所有内容（PolyIOP、SumCheck、VirtualPolynomial 等）
>  
>
>```rust
>use ark_bls12_381::Fr;
>```
>
>* 选用 BLS12-381 的 **标量域** 作为素数域
>  
>
>```rust
>use ark_ff::UniformRand;
>```
>
>* 支持 `Fr::rand(&mut rng)`
>  
>
>```rust
>use ark_poly::{DenseMultilinearExtension, MultilinearExtension};
>```
>
>* 用于：
>  
>    * 多线性多项式
>      
>    * `evaluate`
>      
>
>```rust
>use ark_std::test_rng;
>```
>
>* 确定性随机数（测试可复现）
>  
>
>```rust
>use std::sync::Arc;
>```
>
>* 多项式用 `Arc` 共享
>  
>
>* * *
>
>### 三、`test_sumcheck` 函数本体
>
>* * *
>
>## 1️⃣ 函数签名
>
>```rust
>fn test_sumcheck(
>    nv: usize,
>    num_multiplicands_range: (usize, usize),
>    num_products: usize,
>) -> Result<(), PolyIOPErrors>
>```
>
>### 三个参数控制多项式结构
>
>| 参数           | 含义                         |
>| -------------- | ---------------------------- |
>| `nv`           | 变量个数                     |
>| `(l, r)`       | 每个 product 里 MLE 数量范围 |
>| `num_products` | product 数量                 |
>
>* * *
>
>## 2️⃣ 初始化 RNG 和 transcript
>
>```rust
>let mut rng = test_rng();
>```
>
>* 测试专用 RNG
>  
>* 固定 seed → 结果可复现
>  
>
>* * *
>
>```rust
>let mut transcript = <PolyIOP<Fr> as SumCheck<Fr>>::init_transcript();
>```
>
>### 这句你现在应该很熟了
>
>👉 调用的是：
>
>```rust
>SumCheck::init_transcript()
>```
>
>为 prover 初始化 Fiat–Shamir transcript。
>
>* * *
>
>## 3️⃣ 随机生成 VirtualPolynomial + 真正的和
>
>```rust
>let (poly, asserted_sum) =
>    VirtualPolynomial::rand(nv, num_multiplicands_range, num_products, &mut rng)?;
>```
>
>### 这一步非常关键
>
>它做了两件事：
>
>1. **随机生成一个 VirtualPolynomial**
>  
>2. **真实计算**
>  
>
>$$\text{asserted\_sum} = \sum_{x \in \{0,1\}^{nv}} f(x)$$
>
>📌 所以这是 **ground truth**
>
>* * *
>
>## 4️⃣ Prover：生成 SumCheck 证明
>
>```rust
>let proof = <PolyIOP<Fr> as SumCheck<Fr>>::prove(&poly, &mut transcript)?;
>```
>
>这一步等价于：
>
>```text
>prover(poly) → proof
>```
>
>内部发生了：
>
>* 逐轮构造 `g_i`
>  
>* transcript 派生挑战
>  
>* 折叠 VirtualPolynomial
>  
>
>* * *
>
>## 5️⃣ 保存 aux_info（公共信息）
>
>```rust
>let poly_info = poly.aux_info.clone();
>```
>
>### 为什么 clone？
>
>* prover & verifier **不能共享 poly**
>  
>* verifier 只能看到：
>  
>    * `num_vars`
>      
>    * `max_degree`
>      
>
>* * *
>
>## 6️⃣ 重新初始化 transcript（验证端）
>
>```rust
>let mut transcript = <PolyIOP<Fr> as SumCheck<Fr>>::init_transcript();
>```
>
>⚠️ **非常重要的一点**
>
>> 验证者必须从 **完全相同的初始 transcript 状态** 开始
>
>否则挑战不一致，验证一定失败。
>
>* * *
>
>## 7️⃣ Verifier：验证并生成 SubClaim
>
>```rust
>let subclaim = <PolyIOP<Fr> as SumCheck<Fr>>::verify(
>    asserted_sum,
>    &proof,
>    &poly_info,
>    &mut transcript,
>)?;
>```
>
>### 验证者做了什么？
>
>* 逐轮检查：
>  
>    * `g_i(0) + g_i(1)`
>    
>* 派生随机点：
>  
>    * `r_1, ..., r_nv`
>    
>* 输出：
>  
>
>```rust
>SumCheckSubClaim {
>    point: r,
>    expected_evaluation: f(r)
>}
>```
>
>* * *
>
>## 8️⃣ 最终正确性断言（最重要的一行）
>
>```rust
>assert!(
>    poly.evaluate(&subclaim.point).unwrap() == subclaim.expected_evaluation,
>    "wrong subclaim"
>);
>```
>
>### 这句在验证什么？
>
>👉 **Sum-Check 的核心定理**
>
>> 如果验证通过，那么：
>>
>> $$f(r_1, ..., r_n) = \text{expected\_evaluation}$$
>
>而这里：
>
>* 左边：**真实多项式求值**
>  
>* 右边：**SumCheck 推导的值**
>  
>
>完全一致 ⇒ 协议正确
>
>* * *
>
>## 9️⃣ 测试通过
>
>```rust
>Ok(())
>```
>



```rust
 fn test_sumcheck_internal(
        nv: usize,
        num_multiplicands_range: (usize, usize),
        num_products: usize,
    ) -> Result<(), PolyIOPErrors> {
        let mut rng = test_rng();
        let (poly, asserted_sum) =
            VirtualPolynomial::<Fr>::rand(nv, num_multiplicands_range, num_products, &mut rng)?;
        let poly_info = poly.aux_info.clone();
        let mut prover_state = IOPProverState::prover_init(&poly)?;
        let mut verifier_state = IOPVerifierState::verifier_init(&poly_info);
        let mut challenge = None;
        let mut transcript = IOPTranscript::new(b"a test transcript");
        transcript
            .append_message(b"testing", b"initializing transcript for testing")
            .unwrap();
        for _ in 0..poly.aux_info.num_variables {
            let prover_message =
                IOPProverState::prove_round_and_update_state(&mut prover_state, &challenge)
                    .unwrap();

            challenge = Some(
                IOPVerifierState::verify_round_and_update_state(
                    &mut verifier_state,
                    &prover_message,
                    &mut transcript,
                )
                .unwrap(),
            );
        }
        let subclaim =
            IOPVerifierState::check_and_generate_subclaim(&verifier_state, &asserted_sum)
                .expect("fail to generate subclaim");
        assert!(
            poly.evaluate(&subclaim.point).unwrap() == subclaim.expected_evaluation,
            "wrong subclaim"
        );
        Ok(())
    }
```

>## 一、函数整体在干什么？
>
>```rust
>fn test_sumcheck_internal(...) -> Result<(), PolyIOPErrors>
>```
>
>**一句话总结：**
>
>> 随机生成一个多项式 → 运行完整的 Sumcheck 交互协议 →  
>> 验证最终得到的 subclaim 是否真的和原多项式一致。
>
>这是一个**协议正确性测试**（不是单元函数逻辑测试，而是密码协议测试）。
>
>* * *
>
>## 二、参数是什么意思？
>
>```rust
>nv: usize
>```
>
>* number of variables
>  
>* 多项式的**变量个数**（Sumcheck 要做 `nv` 轮）
>  
>
>```rust
>num_multiplicands_range: (usize, usize)
>```
>
>* 每个乘积项中「相乘的多项式数量」的范围
>  
>* 影响多项式的复杂度（例如 ∏ f_i(x)）
>  
>
>```rust
>num_products: usize
>```
>
>* 有多少个 **product term**
>  
>* 即虚拟多项式是多个乘积的和
>  
>
>👉 这三者共同决定了：
>
>> Sumcheck 的规模、轮数、证明难度
>
>* * *
>
>## 三、逐行解释（重点）
>
>### 1️⃣ 随机生成一个待证明的多项式
>
>```rust
>let mut rng = test_rng();
>let (poly, asserted_sum) =
>    VirtualPolynomial::<Fr>::rand(
>        nv,
>        num_multiplicands_range,
>        num_products,
>        &mut rng
>    )?;
>```
>
>含义：
>
>* `poly`：一个 **虚拟多项式**（VirtualPolynomial）
>  
>* `asserted_sum`：
>  
>    $$\sum_{x \in \{0,1\}^n} poly(x)$$
>
>👉 **Sumcheck 协议的声明目标**
>
>* * *
>
>### 2️⃣ 提取 verifier 所需的公共信息
>
>```rust
>let poly_info = poly.aux_info.clone();
>```
>
>* `aux_info`：
>  
>    * 变量数
>      
>    * 多项式结构
>      
>    * degree 上界
>    
>* **verifier 不能看到完整 poly，只能看到这些**
>  
>
>这是 IOP 的典型设定。
>
>* * *
>
>### 3️⃣ 初始化 prover / verifier 状态
>
>```rust
>let mut prover_state = IOPProverState::prover_init(&poly)?;
>let mut verifier_state = IOPVerifierState::verifier_init(&poly_info);
>```
>
>含义：
>
>| 角色     | 拥有什么       |
>| -------- | -------------- |
>| Prover   | 完整多项式     |
>| Verifier | 多项式结构信息 |
>
>* * *
>
>### 4️⃣ Fiat–Shamir Transcript
>
>```rust
>let mut transcript = IOPTranscript::new(b"a test transcript");
>transcript
>    .append_message(b"testing", b"initializing transcript for testing")
>    .unwrap();
>```
>
>* 用 transcript 模拟 **随机挑战**
>  
>* 把交互协议变成 **非交互（Fiat–Shamir）**
>  
>
>你在看 IOP / PCS / SNARK，这一步非常关键。
>
>* * *
>
>### 5️⃣ Sumcheck 的主循环（核心）
>
>```rust
>for _ in 0..poly.aux_info.num_variables {
>```
>
>👉 **一轮 = 消去一个变量**
>
>#### 🟢 Prover 发消息
>
>```rust
>let prover_message =
>    IOPProverState::prove_round_and_update_state(
>        &mut prover_state,
>        &challenge
>    )
>    .unwrap();
>```
>
>* prover 给出当前变量的 **单变量多项式**
>  
>* 同时更新 prover 内部状态
>  
>
>* * *
>
>#### 🔵 Verifier 验证并生成挑战
>
>```rust
>challenge = Some(
>    IOPVerifierState::verify_round_and_update_state(
>        &mut verifier_state,
>        &prover_message,
>        &mut transcript,
>    )
>    .unwrap(),
>);
>```
>
>* 检查：
>  
>    * 多项式次数是否正确
>      
>    * 前后轮次和是否一致
>    
>* 从 transcript 中生成随机挑战 `r_i`
>  
>
>这就是 **Sumcheck 的递归一致性检查**
>
>* * *
>
>### 6️⃣ 所有轮完成后：生成 Subclaim
>
>```rust
>let subclaim =
>    IOPVerifierState::check_and_generate_subclaim(
>        &verifier_state,
>        &asserted_sum
>    )
>    .expect("fail to generate subclaim");
>```
>
>Sumcheck 的经典结论：
>
>> 如果前面都成立，那么原始求和问题  
>> 等价于在随机点 `r` 处验证一次多项式值
>
>Subclaim 包含：
>
>```text
>point = (r₁, r₂, ..., rₙ)
>expected_evaluation = poly(r)
>```
>
>* * *
>
>### 7️⃣ 用真实多项式验证 Subclaim
>
>```rust
>assert!(
>    poly.evaluate(&subclaim.point).unwrap()
>        == subclaim.expected_evaluation,
>    "wrong subclaim"
>);
>```
>
>这是测试里最关键的一句：
>
>* 用 **真实 poly**
>  
>* 检查 verifier 推导出的 subclaim 是否正确
>  
>
>✔ 成功 → 协议实现正确  
>❌ 失败 → Sumcheck 实现有 bug
>
>* * *
>
>## 四、从协议角度总结
>
>这段代码完整模拟了：
>
>```
>Prover(poly)  ↔  Verifier(aux_info)
>      |
>      |  Sumcheck (n rounds)
>      ↓
>  Subclaim: poly(r) = v
>```
>
>它验证的是：
>
>> **Sumcheck 协议的 soundness + 实现正确性**
>



```rust
 #[test]
    fn test_trivial_polynomial() -> Result<(), PolyIOPErrors> {
        let nv = 1;
        let num_multiplicands_range = (4, 13);
        let num_products = 5;

        test_sumcheck(nv, num_multiplicands_range, num_products)?;
        test_sumcheck_internal(nv, num_multiplicands_range, num_products)
    }
    #[test]
    fn test_normal_polynomial() -> Result<(), PolyIOPErrors> {
        let nv = 12;
        let num_multiplicands_range = (4, 9);
        let num_products = 5;

        test_sumcheck(nv, num_multiplicands_range, num_products)?;
        test_sumcheck_internal(nv, num_multiplicands_range, num_products)
    }
```

>## 一、语法层面：这是 Rust 的测试函数
>
>### 1️⃣ `#[test]` 是什么？
>
>```rust
>#[test]
>fn test_trivial_polynomial() -> Result<(), PolyIOPErrors> { ... }
>```
>
>* `#[test]`：标记这是一个 **测试用例**
>  
>* `cargo test` 时会自动执行
>  
>* 返回 `Result<(), PolyIOPErrors>`：
>  
>    * `Ok(())` → 测试通过
>      
>    * `Err(_)` → 测试失败
>      
>
>👉 **比 panic 更优雅的测试方式**
>
>* * *
>
>## 二、`test_trivial_polynomial` 在测什么？
>
>```rust
>fn test_trivial_polynomial() -> Result<(), PolyIOPErrors> {
>    let nv = 1;
>    let num_multiplicands_range = (4, 13);
>    let num_products = 5;
>
>    test_sumcheck(nv, num_multiplicands_range, num_products)?;
>    test_sumcheck_internal(nv, num_multiplicands_range, num_products)
>}
>```
>
>### 1️⃣ 参数含义回顾
>
>| 参数               | 值                    | 含义                  |
>| ------------------ | --------------------- | --------------------- |
>| `nv = 1`           | 1 个变量              | **最简单的 Sumcheck** |
>| `(4, 13)`          | 每个乘积 4～13 个因子 | 结构复杂              |
>| `num_products = 5` | 5 个乘积项            | 多项式是 5 个乘积的和 |
>
>👉 **变量数最小，但结构不简单**
>
>* * *
>
>### 2️⃣ 调用两个测试函数
>
>```rust
>test_sumcheck(...)?;
>test_sumcheck_internal(...)
>```
>
>#### 🟢 `test_sumcheck(...)`
>
>通常是：
>
>* 测试 **公开 API**
>  
>* 类似“黑盒测试”
>  
>* 只关心协议跑不跑得通
>  
>
>#### 🔵 `test_sumcheck_internal(...)`
>
>你刚刚问过的那个：
>
>* 明确跑完整 **IOP Sumcheck**
>  
>* 最后验证 subclaim 是否和真实多项式一致
>  
>* **白盒 + 协议级测试**
>  
>
>👉 两个一起用，覆盖面更完整。
>
>### 3️⃣ 为什么叫 _trivial_？
>
>不是说多项式结构简单，而是：
>
>> **变量维度极小（n = 1）**
>
>这是 Sumcheck 最基础的 sanity check：
>
>* 只有 **一轮**
>  
>* 很容易暴露：
>  
>    * 边界条件错误
>      
>    * off-by-one
>      
>    * degree 处理错误
>      
>
>* * *
>
>## 三、`test_normal_polynomial` 在测什么？
>
>```rust
>fn test_normal_polynomial() -> Result<(), PolyIOPErrors> {
>    let nv = 12;
>    let num_multiplicands_range = (4, 9);
>    let num_products = 5;
>
>    test_sumcheck(nv, num_multiplicands_range, num_products)?;
>    test_sumcheck_internal(nv, num_multiplicands_range, num_products)
>}
>```
>
>### 1️⃣ 和上一个的区别
>
>| 测试    | nv   | 意义             |
>| ------- | ---- | ---------------- |
>| trivial | 1    | 边界 / 极小规模  |
>| normal  | 12   | **真实使用规模** |
>
>* `nv = 12` → 12 轮 Sumcheck
>  
>* 随机点在 `Fr^{12}`
>  
>* 状态更新复杂很多
>  
>
>* * *
>
>### 2️⃣ 为什么 `num_multiplicands_range` 变小？
>
>```rust
>(4, 13) → (4, 9)
>```
>
>原因通常是：
>
>* `nv` 大了
>  
>* 乘积结构再太复杂：
>  
>    * 测试会变慢
>      
>    * 多项式 degree 爆炸
>      
>
>👉 **性能与覆盖性的折中**
>
>* * *
>
>## 四、从协议测试设计角度看
>
>这两个测试组合在一起非常“专业”👇
>
>| 测试类型   | 覆盖点                   |
>| ---------- | ------------------------ |
>| trivial    | 边界条件、单轮 Sumcheck  |
>| normal     | 多轮状态传递、随机挑战链 |
>| public API | 用户视角                 |
>| internal   | 协议正确性               |
>
>这正是 **密码协议实现** 常见的测试模式。
>





```rust
  #[test]
    fn zero_polynomial_should_error() {
        let nv = 0;
        let num_multiplicands_range = (4, 13);
        let num_products = 5;

        assert!(test_sumcheck(nv, num_multiplicands_range, num_products).is_err());
        assert!(test_sumcheck_internal(nv, num_multiplicands_range, num_products).is_err());
    }

    #[test]
    fn test_extract_sum() -> Result<(), PolyIOPErrors> {
        let mut rng = test_rng();
        let mut transcript = <PolyIOP<Fr> as SumCheck<Fr>>::init_transcript();
        let (poly, asserted_sum) = VirtualPolynomial::<Fr>::rand(8, (3, 4), 3, &mut rng)?;

        let proof = <PolyIOP<Fr> as SumCheck<Fr>>::prove(&poly, &mut transcript)?;
        assert_eq!(
            <PolyIOP<Fr> as SumCheck<Fr>>::extract_sum(&proof),
            asserted_sum
        );
        Ok(())
    }
```

>## 一、`zero_polynomial_should_error`：非法输入必须报错
>
>### 代码
>
>```rust
>#[test]
>fn zero_polynomial_should_error() {
>    let nv = 0;
>    let num_multiplicands_range = (4, 13);
>    let num_products = 5;
>
>    assert!(test_sumcheck(nv, num_multiplicands_range, num_products).is_err());
>    assert!(test_sumcheck_internal(nv, num_multiplicands_range, num_products).is_err());
>}
>```
>
>* * *
>
>### 1️⃣ `nv = 0` 表示什么？
>
>* 多项式变量个数为 **0**
>  
>* 定义域是 `{0,1}^0` —— 只有一个点（空向量）
>  
>
>在 **Sumcheck 协议语义里**：
>
>> Sumcheck 是用来证明
>>
>> $$\sum_{x\in\{0,1\}^n} f(x) = S$$
>>
>> **前提是 `n ≥ 1`**
>
>`n = 0`：
>
>* 没有轮次
>  
>* 没有 challenge
>  
>* 协议退化，很多实现 **明确禁止**
>  
>
>* * *
>
>### 2️⃣ 为什么测试期望 `is_err()`？
>
>```rust
>assert!(test_sumcheck(...).is_err());
>```
>
>这在密码工程中非常重要：
>
>> **非法 statement 必须 fail fast**
>
>如果 `nv = 0` 还能通过：
>
>* 协议定义不严谨
>  
>* 攻击者可能构造边界 case 绕过验证
>  
>
>* * *
>
>### 3️⃣ 同时测两个接口
>
>```rust
>test_sumcheck(...)
>test_sumcheck_internal(...)
>```
>
>含义：
>
>| 接口                   | 层次     |
>| ---------------------- | -------- |
>| test_sumcheck          | 公共 API |
>| test_sumcheck_internal | 协议内部 |
>
>👉 保证 **任何路径** 都不会悄悄接受非法多项式
>
>* * *
>
>### ✅ 这个测试的本质
>
>> **健壮性（robustness）测试 + 边界条件安全性**
>
>* * *
>
>## 二、`test_extract_sum`：从 proof 中取回 asserted sum
>
>### 代码
>
>```rust
>#[test]
>fn test_extract_sum() -> Result<(), PolyIOPErrors> {
>```
>
>这是一个 **正向功能测试**。
>
>* * *
>
>### 1️⃣ 初始化随机性 & transcript
>
>```rust
>let mut rng = test_rng();
>let mut transcript = <PolyIOP<Fr> as SumCheck<Fr>>::init_transcript();
>```
>
>* `test_rng()`：测试用随机数生成器
>  
>* `init_transcript()`：
>  
>    * Sumcheck 协议专用的 Fiat–Shamir transcript
>      
>    * 绑定 hash domain separation
>      
>
>这是 **严肃密码实现** 的写法。
>
>* * *
>
>### 2️⃣ 随机生成多项式和它的真实求和值
>
>```rust
>let (poly, asserted_sum) =
>    VirtualPolynomial::<Fr>::rand(8, (3, 4), 3, &mut rng)?;
>```
>
>* `asserted_sum`：
>  
>    $$\sum_{x\in\{0,1\}^8} poly(x)$$
>
>这是 prover 想证明的 statement。
>
>* * *
>
>### 3️⃣ 生成 Sumcheck 证明
>
>```rust
>let proof =
>    <PolyIOP<Fr> as SumCheck<Fr>>::prove(&poly, &mut transcript)?;
>```
>
>这里用了 **完全限定语法**：
>
>```rust
><实现类型 as Trait>::函数
>```
>
>意思是：
>
>* `PolyIOP<Fr>` 实现了 `SumCheck<Fr>` trait
>  
>* 调用的是 **SumCheck 协议接口**
>  
>
>* * *
>
>### 4️⃣ 从 proof 中“提取” asserted sum
>
>```rust
>assert_eq!(
>    <PolyIOP<Fr> as SumCheck<Fr>>::extract_sum(&proof),
>    asserted_sum
>);
>```
>
>**这行非常关键** 👇
>
>#### 语义上：
>
>> Sumcheck proof **必须明确绑定** 它声称的求和值
>
>否则：
>
>* verifier 无法知道 prover 在证明“哪个和”
>  
>* proof 可以被复用 / 混淆
>  
>
