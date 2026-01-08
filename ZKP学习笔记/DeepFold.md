# DeepFold

# 论文解读：DeepFold

**标题:** DeepFold: Efficient Multilinear Polynomial Commitment from Reed-Solomon Code and Its Application to Zero-knowledge Proofs **作者:** Yanpei Guo, Wenjie Qu, 等 (新加坡国立大学) **领域:** 密码学 / 零知识证明 (zk-SNARKs) / 多项式承诺方案 (PCS)

## 1. 核心概述

这篇论文提出了一种名为 **DeepFold** 的新型多线性多项式承诺方案（PCS）。该方案基于里德-所罗门码（Reed-Solomon Code）和 FRI（Fast Reed-Solomon IOPP）协议构建。

**主要成就：**

- **极致的效率：** 保持了最优的证明者时间（Prover Time）。
- **更小的证明体积：** 相比于之前的最先进方案（如 BaseFold），证明体积（Proof Size）减少了 **3倍**。
- **打破限制：** 首次将基于 FRI 的多线性 PCS 适配到了**列表解码半径（List Decoding Radius）**设置下，从而显著减少了所需的查询次数。

## 2. 背景与痛点

在零知识证明（zk-SNARKs）系统中，多项式承诺方案（PCS）是一个核心组件。

- **现有方案的问题：**
  - **KZG：** 需要可信设置（Trusted Setup），且计算开销大。
  - **BaseFold (Crypto '24)：** 虽然实现了高效的证明者时间，但受限于“唯一解码半径（Unique Decoding Radius）”，导致查询重复次数很高（>100次），从而使得证明体积很大（约 600KB+）。
  - **PolyFRIM (USENIX Security '24)：** 性能不错，但 DeepFold 在各方面都比它提升了约 2 倍。

## 3. DeepFold 的核心技术创新

### A. 引入列表解码（List Decoding）以减少查询

DeepFold 的核心突破在于突破了 BaseFold 的限制，利用了 DEEP（Domain Extending for Eliminating Pretenders）技术，使得方案可以在**列表解码半径**下工作。

- **结果：** 允许更宽松的参数设置，从而将查询重复次数从 >100 降低到 **~34** 次。
- **影响：** 直接导致证明体积大幅缩小。

### B. 任意长度输入的批处理评估 (Batch Evaluation)

传统的 PCS 通常要求多项式大小为 $2^k$（2的幂次）。如果输入数据长度不是 $2^k$，通常需要填充（padding）大量的零，这会造成计算浪费。

- **DeepFold 的创新：** 提出了一种批处理技术，可以将任意长度的输入拆分为多个不同大小的多项式进行承诺，而无需填充。
- **优势：** 对于非 $2^k$ 大小的矩阵乘法（如 GPT-2 中的 $768 \times 2304$ 矩阵），效率提升显著。

### C. 零知识性 (Zero-Knowledge)

论文还通过批处理技术，构建了 **zkDeepFold**，在不显著增加开销的情况下实现了零知识属性。

## 4. 性能评估

论文将 DeepFold 与目前主流的 PCS 方案（如 BaseFold, Virgo, Hyrax, mKZG 等）进行了对比：

| 指标           | 对比 BaseFold   | 对比 PolyFRIM | 对比 Virgo |
| -------------- | --------------- | ------------- | ---------- |
| **证明者时间** | 相似 (最优级别) | 快 2 倍       | 快 3 倍    |
| **验证者时间** | 快 3 倍         | 快 2 倍       | -          |
| **证明体积**   | **小 3 倍**     | 小 2 倍       | -          |

*注：在 22 个变量的多项式下，BaseFold 证明大小约为 619 KB，而 DeepFold 仅为 208 KB。*

## 5. 实际应用场景

DeepFold 可以作为插件替换现有的 zk-SNARK 系统中的 PCS 组件，从而提升整体性能：

1. **通用 zk-SNARKs：**
   - 替换 **Virgo**：证明 256 叶子的 Merkle Tree 时，速度提升 **2.5倍**。
   - 替换 **HyperPlonk** 中的 PCS：相比 HyperPlonk + mKZG，速度提升 **3.6倍**。
2. **可验证矩阵乘法 (zkML 关键技术)：**
   - 在验证 GPT-2 模型中使用的 $768 \times 2304$ 矩阵乘法时，利用 DeepFold 的任意长度处理能力，比传统填充方法的证明者时间快 **2.4倍**。

## 6. 总结

DeepFold 是目前基于纠删码（Error-Correcting Codes）且无需可信设置的 PCS 方案中，综合性能（证明速度、验证速度、证明大小）最优的方案之一。它解决了 BaseFold 证明体积过大的痛点，使其更适合实际部署，特别是在 zkML（零知识机器学习）等处理大规模数据的场景中。



