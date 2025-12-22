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



