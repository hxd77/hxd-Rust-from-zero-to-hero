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
>    * `index_info` → 多项式的辅助信息
>    
>* 输出：
>  
>    * 返回一个初始化好的验证者 struct
>    
>* 内部可能初始化：
>  
>    * 当前轮 `round = 0`
>      
>    * 存储多项式变量数、最大次数
>      
>    * 初始化 transcript
>      
>
>* * *
>
>### 2️⃣ `verify_round_and_update_state`
>
>```rust
>fn verify_round_and_update_state(
>    &mut self,
>    prover_msg: &Self::ProverMessage,
>    transcript: &mut Self::Transcript,
>) -> Result<Self::Challenge, PolyIOPErrors>;
>```
>
>* 功能：
>  
>    1. **接收 Prover 当前轮的消息**
>      
>    2. **生成本轮 challenge**（随机 field 元素）
>      
>    3. **更新内部状态**
>    
>* 注意：
>  
>    * **实际的验证并不在这里完成**
>      
>        * 只是采样 challenge 并记录状态
>          
>        * 最终验证延迟到 `check_and_generate_subclaim`
>    
>* 数学背景：
>  
>    * SumCheck 协议每轮：
>      
>        1. Prover 发送部分和多项式
>          
>        2. Verifier 采样随机 challenge
>          
>            $$r \in \mathbb{F}_q$$
>            
>            用于固定下一轮变量
>            
>
>* * *
>
>### 3️⃣ `check_and_generate_subclaim`
>
>```rust
>fn check_and_generate_subclaim(
>    &self,
>    asserted_sum: &F,
>) -> Result<Self::SumCheckSubClaim, PolyIOPErrors>;
>```
>
>* 功能：
>  
>    1. 对之前延迟的验证操作进行检查
>      
>    2. 生成一个子声明（SubClaim），包括：
>      
>        * 某个点 `point`
>          
>        * 多项式在该点的期望值 `expected_evaluation`
>        
>    3. 如果 Prover 作弊，检查失败（返回错误）
>    
>* 数学背景：
>  
>    * 延迟检查是 SumCheck 的标准优化：
>      
>        * 不每轮都验证所有计算，而是最后一次性检查
>          
>            $$f(\text{sampled point}) \stackrel{?}{=} \text{expected value}$$
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
>    let challenge = verifier.verify_round_and_update_state(&prover_msg, &mut transcript)?;
>    // challenge 发送给 Prover，用于下一轮折叠多项式
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
>
