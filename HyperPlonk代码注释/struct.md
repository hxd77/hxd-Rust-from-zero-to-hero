# struct

```rust
use arithmetic::VirtualPolynomial;
use ark_ff::PrimeField;
use ark_serialize::CanonicalSerialize;

/// IOP证明是一个集合，包含
/// - 在交互式协议的每一轮中，证明者发送给验证者的消息。
/// - 由用于评估的记录生成的一个点。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IOPProof<F: PrimeField> {
    pub point: Vec<F>,
    pub proofs: Vec<IOPProverMessage<F>>,
}
```

>#### 一、导入模块
>
>```rust
>use arithmetic::VirtualPolynomial;
>use ark_ff::PrimeField;
>use ark_serialize::CanonicalSerialize;
>```
>
>* `VirtualPolynomial` → 多项式相关类型，用于多项式操作
>  
>* `PrimeField` → Arkworks 的素数域 trait，表示域元素类型
>  
>* `CanonicalSerialize` → 用于把数据序列化（比如发送或存储）
>  
>
>* * *
>
>#### 二、注释
>
>```rust
>/// IOP证明是一个集合，包含
>/// - 在交互式协议的每一轮中，证明者发送给验证者的消息。
>/// - 由用于评估的记录生成的一个点。
>```
>
>解释：
>
>* **IOPProof** 包含两部分核心内容：
>  
>    1. **每轮消息**：证明者发送给验证者的数据（例如多项式评估值、证明碎片）
>      
>    2. **评估点**：证明者为了验证生成的点，用来检查多项式或计算正确性
>      
>
>* * *
>
>#### 三、派生宏
>
>```rust
>#[derive(Clone, Debug, Default, PartialEq, Eq)]
>```
>
>* 自动为 `IOPProof` 生成常用 trait：
>  
>    * `Clone` → 可以拷贝
>      
>    * `Debug` → 可以用 `{:?}` 打印
>      
>    * `Default` → 可以生成默认实例（空向量等）
>      
>    * `PartialEq` & `Eq` → 可以直接比较两个 IOPProof 是否相等
>      
>
>* * *
>
>#### 四、结构体定义
>
>```rust
>pub struct IOPProof<F: PrimeField> {
>    pub point: Vec<F>,
>    pub proofs: Vec<IOPProverMessage<F>>,
>}
>```
>
>* `F: PrimeField` → 泛型参数，说明这是 **在素数域 F 上的证明**
>  
>* 字段解释：
>  
>    1. `point: Vec<F>`
>      
>        * 保存 **用于评估的点**，每个元素是一个域元素
>        
>    2. `proofs: Vec<IOPProverMessage<F>>`
>      
>        * 保存 **每轮证明者发送的消息**
>          
>        * 每条消息类型是 `IOPProverMessage<F>`（可能包含多项式评估值等）
>          
>
>* * *
>
>## 五、总结理解
>
>> `IOPProof<F>` 就是一个 **交互式多项式证明的容器**：
>>
>> * `point` → 用于验证的点
>>
>> * `proofs` → 证明者在每轮协议中发送的消息
>>
>>
>> 通过这些信息，验证者可以在有限域 `F` 上高效检查证明的正确性。
>
>* * *
>
>💡 类比理解：
>
>```text
>IOPProof = {
>    point: [x1, x2, ...],      // 验证用的随机点
>    proofs: [msg_round1, msg_round2, ...]  // 每轮发送的消息
>}
>```



---

```rust
/// 证明者在特定回合向验证者发送的消息是一个评估列表。
#[derive(Clone, Debug, Default, PartialEq, Eq, CanonicalSerialize)]
pub struct IOPProverMessage<F: PrimeField> {
    pub(crate) evaluations: Vec<F>,
}
```

---



>#### 一、派生宏
>
>```rust
>#[derive(Clone, Debug, Default, PartialEq, Eq, CanonicalSerialize)]
>```
>
>* `Clone` → 可以拷贝（`let b = a.clone()`）
>  
>* `Debug` → 可以用 `{:?}` 打印结构体
>  
>* `Default` → 可以生成默认实例（空向量等）
>  
>* `PartialEq` & `Eq` → 可以直接比较两个 `IOPProverMessage` 是否相等
>  
>* `CanonicalSerialize` → 可以将消息序列化（比如存储或发送给 verifier）
>
>* * *
>
>#### 二、结构体定义
>
>```rust
>pub struct IOPProverMessage<F: PrimeField> {
>    pub(crate) evaluations: Vec<F>,
>}
>```
>
>* `F: PrimeField` → 泛型参数，说明消息中的数据是 **素数域 F 上的元素**
>  
>* 字段解释：
>  
>    * `evaluations: Vec<F>`
>      
>        * 保存证明者在当前轮生成的 **多项式评估值**
>          
>        * 例如，你有一个多项式 $P(x)$，在一些点 $x_1, x_2, ...$ 上计算 $P(x_i)$，这些值就存放在 `evaluations` 中
>    
>* `pub(crate)` → 表示这个字段 **只能在当前 crate 内访问**，外部模块无法直接访问
>  
>
>* * *
>
>#### 三、和 IOPProof 的关系
>
>* 在 `IOPProof` 中：
>  
>
>```rust
>pub proofs: Vec<IOPProverMessage<F>>,
>```
>
>* 每个 `IOPProverMessage` 就是 **一轮消息**
>  
>* 例如：
>  
>
>```text
>l轮 -> IOPProverMessage { evaluations: [P1(x1), P1(x2), ...] }
>l+1轮 -> IOPProverMessage { evaluations: [P2(x1), P2(x2), ...] }
>```
>
>* `IOPProof` 把这些消息集合起来，形成完整的证明
>  
>
>* * *
>
>#### 四、通俗理解
>
>> **IOPProverMessage = 当前轮证明者发送给验证者的消息（多项式在某些点的评估值）**
>>
>> * `evaluations` 是消息的核心数据
>>
>> * `CanonicalSerialize



---

```rust
/// PolyIOP的证明者状态。
pub struct IOPProverState<F: PrimeField> {
    /// 验证者提供的抽样随机性
    pub challenges: Vec<F>,
    /// 当前的轮次编号
    pub(crate) round: usize,
    /// 指向虚拟多项式的指针
    pub(crate) poly: VirtualPolynomial<F>,
    /// 具有预先计算的重心权重的点，用于外推更小的（物体/部分等）
    /// 将单变量多项式的次数计算到`max_degree + 1`次评估。
    pub(crate) extrapolation_aux: Vec<(Vec<F>, Vec<F>)>,
}
```

---

>#### 一、结构体整体含义
>
>```rust
>pub struct IOPProverState<F: PrimeField> {
>    ...
>}
>```
>
>* `F: PrimeField`：所有计算都在一个**素数域**上进行（例如 `Fr`）
>  
>* `IOPProverState`：  
>    👉 **证明者在 IOP 协议执行过程中维护的“工作区”**
>
>它会随着每一轮交互不断更新。
>
>* * *
>
>#### 二、字段逐个解释
>
>### 1️⃣ 验证者提供的随机挑战
>
>```rust
>/// 验证者提供的抽样随机性
>pub challenges: Vec<F>,
>```
>
>* 含义：  
>    验证者在每一轮都会发送一个 **随机挑战值**（challenge）
>    
>* 作用：
>  
>    * 用于防止证明者作弊
>      
>    * 决定下一轮多项式如何折叠 / 求值
>    
>* 在 IOP / Sumcheck / GKR 中非常常见
>  
>
>📌 示例（概念）：
>
>```text
>Verifier → r₁, r₂, r₃
>Prover   → 根据 r₁, r₂, r₃ 计算下一轮消息
>```
>
>* * *
>
>### 2️⃣ 当前轮次编号
>
>```rust
>/// 当前的轮次编号
>pub(crate) round: usize,
>```
>
>* 记录当前进行到第几轮
>  
>* `pub(crate)`：只在当前 crate 内可见
>  
>* 作用：
>  
>    * 控制协议流程
>      
>    * 决定使用哪个 challenge
>      
>    * 决定多项式剩余的变量数量
>      
>
>📌 比如：
>
>* round = 0 → 初始多项式
>  
>* round = 1 → 固定第一个变量
>  
>* round = 2 → 固定前两个变量 …
>  
>
>* * *
>
>### 3️⃣ 虚拟多项式（核心对象）
>
>```rust
>/// 指向虚拟多项式的指针
>pub(crate) poly: VirtualPolynomial<F>,
>```
>
>这是**最关键的字段**之一。
>
>#### 什么是 `VirtualPolynomial`？
>
>* 并不真的把多项式完全展开
>  
>* 而是：
>  
>    * 用一组「子多项式 + 系数」
>      
>    * 动态计算评估值
>    
>* 非常适合：
>  
>    * Sumcheck
>      
>    * PolyIOP
>      
>    * 大规模多项式
>      
>
>📌 在你的上下文里：
>
>```text
>poly = 原始多项式
>→ 每一轮用 challenge 折叠变量
>→ poly 维度越来越小
>```
>
>* * *
>
>### 4️⃣ 外推辅助信息（高级但很重要）
>
>```rust
>/// 具有预先计算的重心权重的点，用于外推更小的（物体/部分等）
>/// 将单变量多项式的次数计算到`max_degree + 1`次评估。
>pub(crate) extrapolation_aux: Vec<(Vec<F>, Vec<F>)>
>```



---

```rust
/// 多项式交互式证明的验证者状态
pub struct IOPVerifierState<F: PrimeField> {
    pub(crate) round: usize,
    pub(crate) num_vars: usize,
    pub(crate) max_degree: usize,
    pub(crate) finished: bool,
    /// 一个存储由……发送的、处于求值形式的单变量多项式的列表
    /// 每一轮的证明者
    pub(crate) polynomials_received: Vec<Vec<F>>,
    /// 一个存储验证者在每一轮采样的随机性的列表
    pub(crate) challenges: Vec<F>,
}
```

---

>### 1️⃣ 当前轮次
>
>```rust
>pub(crate) round: usize,
>```
>
>* 表示当前是 **第几轮 IOP**
>  
>* 每一轮：
>  
>    1. Verifier 接收一个单变量多项式（或其评估）
>      
>    2. Verifier 抽样一个随机 challenge
>      
>    3. 轮次 +1
>      
>
>📌 和 ProverState 中的 `round` **必须同步**
>
>* * *
>
>### 2️⃣ 多项式的变量数
>
>```rust
>pub(crate) num_vars: usize,
>```
>
>* 表示 **原始多项式的变量个数**
>  
>* 在 Sumcheck / PolyIOP 中：
>  
>    * 有 `num_vars` 轮
>      
>    * 每一轮“消掉”一个变量
>      
>
>📌 例如：
>
>```text
>f(x1, x2, x3)
>→ 3 轮交互
>```
>
>* * *
>
>### 3️⃣ 单变量多项式的最大次数
>
>```rust
>pub(crate) max_degree: usize,
>```
>
>* 限制证明者每一轮发送的 **单变量多项式的次数**
>  
>* 用来防止：
>  
>    * 证明者发送“非法高次多项式”
>      
>    * 偷偷塞入额外自由度作弊
>      
>
>📌 验证时会检查：
>
>```text
>len(polynomial) ≤ max_degree + 1
>```
>
>* * *
>
>### 4️⃣ 协议是否结束
>
>```rust
>pub(crate) finished: bool,
>```
>
>* 标记整个 IOP 是否已经完成
>  
>* 常见逻辑：
>  
>    * 当 `round == num_vars`
>      
>    * 或最终一致性检查完成
>      
>    * 设置 `finished = true`
>      
>
>* * *
>
>### 5️⃣ 收到的单变量多项式（评估形式）
>
>```rust
>/// 一个存储由……发送的、处于求值形式的单变量多项式的列表
>/// 每一轮的证明者
>pub(crate) polynomials_received: Vec<Vec<F>>,
>```
>
>这是验证者**最重要的证据存储区**。
>
>#### 具体含义
>
>* 每一轮，证明者发送一个单变量多项式
>  
>* 不是系数形式，而是：
>  
>    * 在某个定义域上的 **评估值列表**
>      
>
>📌 举例：
>
>```text
>第 i 轮：
>g_i(x) = ax + b
>→ prover 发送 [g_i(ω^0), g_i(ω^1), ..., g_i(ω^d)]
>```
>
>Verifier 存下来，用于：
>
>* 相邻轮一致性检查
>  
>* 最后一轮与声明值的核对
>  
>
>* * *
>
>### 6️⃣ 验证者采样的随机挑战
>
>```rust
>/// 一个存储验证者在每一轮采样的随机性的列表
>pub(crate) challenges: Vec<F>,
>```
>
>* 每一轮：
>  
>    * Verifier 随机采样一个 `r_i`
>      
>    * 发给 Prover
>    
>* 同时自己也要保存下来
>  
>
>📌 用途：
>
>* 验证：
>  
>    ```text
>    g_i(r_i) == g_{i+1}(0) + g_{i+1}(1)
>    ```
>    
>* 最后一轮用来检查多项式在某个点的值
>  
>
