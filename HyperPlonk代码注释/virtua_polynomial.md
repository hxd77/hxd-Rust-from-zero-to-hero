# virtual_polynomial

```rust
use crate::{errors::ArithErrors, multilinear_polynomial::random_zero_mle_list, random_mle_list};
use ark_ff::PrimeField;
use ark_poly::{DenseMultilinearExtension, MultilinearExtension};
use ark_serialize::CanonicalSerialize;
use ark_std::{
    end_timer,
    rand::{Rng, RngCore},
    start_timer,
};
use rayon::prelude::*;
use std::{cmp::max, collections::HashMap, marker::PhantomData, ops::Add, sync::Arc};
```

>#### 一、crate 内部模块
>
>```rust
>use crate::{errors::ArithErrors, multilinear_polynomial::random_zero_mle_list, random_mle_list};
>```
>
>* `crate` → 当前 crate（包）根目录
>  
>* `errors::ArithErrors` → 引入你自己定义的算术错误类型
>  
>* `multilinear_polynomial::random_zero_mle_list` → 引入一个函数，用来生成多变量多项式列表，值在 0 附近
>  
>* `random_mle_list` → 同样引入函数，可以生成随机多线性扩展列表
>
>* * *
>
>#### 二、arkworks 库相关
>
>```rust
>use ark_ff::PrimeField;
>use ark_poly::{DenseMultilinearExtension, MultilinearExtension};
>use ark_serialize::CanonicalSerialize;
>```
>
>* `ark_ff::PrimeField` → 素域类型，所有算术运算都在这个域上进行
>  
>* `ark_poly::{DenseMultilinearExtension, MultilinearExtension}` →
>  
>    * `MultilinearExtension` → 抽象的多变量多项式接口
>      
>    * `DenseMultilinearExtension` → 稠密实现（保存所有系数）
>    
>* `ark_serialize::CanonicalSerialize` → 用于序列化多项式或域元素
>
>* * *
>
>#### 三、ark_std 和随机/计时相关
>
>```rust
>use ark_std::{
>    end_timer,
>    rand::{Rng, RngCore},
>    start_timer,
>};
>```
>
>* `start_timer` / `end_timer` → ark_std 提供的计时工具，用于性能测试
>  
>* `rand::{Rng, RngCore}` → 随机数生成 trait
>  
>    * `Rng` → 高级随机生成接口
>      
>    * `RngCore` → 核心随机生成接口
>
>* * *
>
>#### 四、并行计算和标准库
>
>```rust
>use rayon::prelude::*;
>use std::{cmp::max, collections::HashMap, marker::PhantomData, ops::Add, sync::Arc};
>```
>
>* `rayon::prelude::*` → 引入并行迭代器功能，用于多线程处理向量或多项式
>  
>* `std::cmp::max` → 求最大值
>  
>* `std::collections::HashMap` → 哈希表（键值对存储）
>  
>* `std::marker::PhantomData` → Rust 泛型零成本类型标记，用于编译器类型检查
>  
>* `std::ops::Add` → 加法 trait，用于自定义类型支持 `+` 运算
>  
>* `std::sync::Arc` → 原子引用计数，线程安全的共享指针



### 一、VirtualPolynomial 是什么？

```text
A virtual polynomial is a sum of products of multilinear polynomials
```

* **虚拟多项式** = 多个多线性多项式的乘积的和
  
* **重点**：它不是直接存储完整的多项式系数，而是 **存储乘积组合和系数**
  

公式化：

$$f = \sum_{i=0}^{n-1} c_i \cdot \prod_{j=0}^{m_i} P_{ij}$$

* $n$ = 乘积数量
  
* $m_i$ = 第 i 个乘积里有多少个因子多线性多项式
  
* $c_i$ = 第 i 个乘积的系数
  
* $P_{ij}$ = 每个因子多线性多项式
  

* * *

### 二、举例说明

注释里举的例子：

```text
f = c0 * f0 * f1 * f2 + c1 * f3 * f4
```

* `f0, f1, f2, f3, f4` 是多线性多项式
  
* `c0, c1` 是它们对应的系数
  

### 内部存储方式

1. `flattened_ml_extensions`
  

```text
存储 f0, f1, f2, f3, f4 的多线性扩展（DenseMultilinearExtension）
```

* 就是把每个因子多项式的多线性扩展 **平铺存储在一个数组里**
  

2. `products`
  

```text
[
    (c0, [0, 1, 2]),  // c0 * f0 * f1 * f2
    (c1, [3, 4])      // c1 * f3 * f4
]
```

* 这里的 `[0,1,2]` 和 `[3,4]` 是索引，指向 `flattened_ml_extensions`
  
* `products[i].0` → 系数 `c_i`
  
* `products[i].1` → 该乘积包含哪些多线性多项式的索引
  

3. `raw_pointers_lookup_table`

```text
把每个 f_i 映射到它在 flattened_ml_extensions 中的索引，flattened_ml_extensions=[f0,f1,f2,f3,f4]
```

* 用来快速查找某个多线性多项式在数组中的位置
  

* * *

### 三、总结

**VirtualPolynomial 的核心思想**：

1. 不直接展开完整的多项式（避免指数级系数增长）
  
2. 通过“乘积 + 系数 + 多线性因子数组”的组合表示
  
3. 可以快速：
  
    * 在某个点上求值
      
    * 折叠（fix）某个变量
      
    * 与 Sumcheck / PolyIOP 协议配合使用
      

* * *

📌 **简单类比**：

```text
完整多项式： f(x,y,z) = 3*x*y + 5*y*z + 7
VirtualPolynomial 表示：
  products = [
    (3, [f0_index, f1_index]),   // f0 = x, f1 = y
    (5, [f2_index, f3_index])    // f2 = y, f3 = z
  ]
  flattened_ml_extensions = [x, y, y, z]
```



```rust
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VirtualPolynomial<F: PrimeField> {
    /// 关于多线性多项式的辅助信息
    pub aux_info: VPAuxInfo<F>,
    /// 多线性扩展的产品引用列表（作为usize类型）
    pub products: Vec<(F, Vec<usize>)>,
    /// 存储其中乘积被乘数可引用的多线性扩展
    /// to.
    pub flattened_ml_extensions: Vec<Arc<DenseMultilinearExtension<F>>>,
    /// 指向上述多项式扩展的指针
    raw_pointers_lookup_table: HashMap<*const DenseMultilinearExtension<F>, usize>,
}
```

>#### 一、结构体声明
>
>```rust
>#[derive(Clone, Debug, Default, PartialEq)]
>pub struct VirtualPolynomial<F: PrimeField> { ... }
>```
>
>* `#[derive(...)]` → 自动实现一些 trait：
>  
>    * `Clone` → 可以克隆 VirtualPolynomial
>      
>    * `Debug` → 可以用 `{:?}` 打印
>      
>    * `Default` → 可以用 `VirtualPolynomial::default()` 初始化
>      
>    * `PartialEq` → 可以用 `==` 比较是否相等
>    
>* `F: PrimeField` → 泛型类型 F 必须是素域元素（PrimeField），所有多项式运算都在这个域上进行
>  
>
>* * *
>
>#### 二、字段解析
>
>### 1️⃣ `aux_info: VPAuxInfo<F>`
>
>```rust
>/// Aux information about the multilinear polynomial
>```
>
>* **辅助信息**
>  
>* 用来存储多线性多项式的一些预计算信息，例如：
>  
>    * 最大变量数量
>      
>    * 预计算的点值或重心权重
>    
>* 目的是 **加速多项式求值 / 折叠**
>  
>
>* * *
>
>### 2️⃣ `products: Vec<(F, Vec<usize>)>`
>
>```rust
>/// list of reference to products (as usize) of multilinear extension
>```
>
>* **存储虚拟多项式中的每个乘积及系数**
>  
>* 每个元素 `(c_i, indices)`：
>  
>    * `c_i: F` → 第 i 个乘积的系数
>      
>    * `indices: Vec<usize>` → 第 i 个乘积包含哪些多线性多项式（索引指向 `flattened_ml_extensions`）
>      
>
>**举例**：
>
>```rust
>f = c0 * f0 * f1 + c1 * f2
>products = [
>    (c0, [0, 1]),
>    (c1, [2])
>]
>flattened_ml_extensions = [f0, f1, f2]
>```
>
>* * *
>
>### 3️⃣ `flattened_ml_extensions: Vec<Arc<DenseMultilinearExtension<F>>>`
>
>```rust
>/// Stores multilinear extensions in which product multiplicand can refer to
>```
>
>* **存储所有多线性多项式的 DenseMultilinearExtension**
>  
>* `Arc<...>` → 线程安全的共享指针
>  
>    * 因为同一个多线性多项式可能被多个乘积引用
>    
>* `flattened` → 把多线性多项式平铺到一个数组中，方便索引
>  
>
>* * *
>
>### 4️⃣ `raw_pointers_lookup_table: HashMap<*const DenseMultilinearExtension<F>, usize>`
>
>```rust
>/// Pointers to the above poly extensions
>```
>
>* 哈希表，用来快速查找某个多线性多项式在 `flattened_ml_extensions` 中的索引
>  
>* key = 指向 DenseMultilinearExtension 的原始指针
>  
>* value = 索引
>  
>* 目的是 **折叠或操作虚拟多项式时，能快速找到对应多线性多项式**



```rust
#[derive(Clone, Debug, Default, PartialEq, Eq, CanonicalSerialize)]
/// 关于多线性多项式的辅助信息
pub struct VPAuxInfo<F: PrimeField> {
    /// 每个乘积中乘数的最大数量
    pub max_degree: usize,
    /// 多项式的变量数量
    pub num_variables: usize,
    /// 相关领域
    #[doc(hidden)] //不想让这个字段出现在文档里
    pub phantom: PhantomData<F>,
}
```



```rust
impl<F: PrimeField> Add for &VirtualPolynomial<F> {
    type Output = VirtualPolynomial<F>;
    fn add(self, other: &VirtualPolynomial<F>) -> Self::Output { 
        let start = start_timer!(|| "virtual poly add");
        let mut res = self.clone();
        for products in other.products.iter() {
            let cur: Vec<Arc<DenseMultilinearExtension<F>>> = products
                .1
                .iter()
                .map(|&x| other.flattened_ml_extensions[x].clone())
                .collect();

            res.add_mle_list(cur, products.0)
                .expect("add product failed");
        }
        end_timer!(start);
        res
    }
}
```

>#### 一、整体一句话先理解
>
>```rust
>impl<F: PrimeField> Add for &VirtualPolynomial<F>
>```
>
>👉 这段代码的作用是：
>
>> **定义两个虚拟多线性多项式 `VirtualPolynomial` 的“加法”规则**  
>> 即：
>>
>> $$P + Q = \sum (\text{P 的所有乘积项}) + \sum (\text{Q 的所有乘积项})$$
>
>而且：
>
>* 左右操作数都是 **引用 `&VirtualPolynomial`**
>  
>* 结果是一个 **新的 `VirtualPolynomial`**
>  
>
>* * *
>
>#### 二、`Add` trait 是什么？
>
>```rust
>impl<F: PrimeField> Add for &VirtualPolynomial<F> {
>    type Output = VirtualPolynomial<F>;
>```
>
>这是 Rust 的**运算符重载**：
>
>* 对应 `+` 运算符
>  
>* 允许你写：
>  
>
>```rust
>let r = &vp1 + &vp2;
>```
>
>### 为什么是 `for &VirtualPolynomial<F>`？
>
>这是**非常关键、非常专业的写法**：
>
>* 不消耗原多项式（避免 move）
>  
>* 可以多次复用同一个 `VirtualPolynomial`
>  
>* 符合密码学中“结构不可变、构造新对象”的习惯
>  
>
>* * *
>
>#### 三、函数签名
>
>```rust
>fn add(self, other: &VirtualPolynomial<F>) -> Self::Output
>```
>
>等价于：
>
>```rust
>fn add(&self, &other) -> VirtualPolynomial<F>
>```
>
>👉 意思是：
>
>* 输入：两个多项式的引用
>  
>* 输出：一个新的多项式
>  
>
>* * *
>
>#### 四、逐行解释（重点部分）
>
>* * *
>
>### 1️⃣ 计时器（性能分析）
>
>```rust
>let start = start_timer!(|| "virtual poly add");
>```
>
>* arkworks 的 **性能 profiling 工具**
>  
>* 用来统计“多项式加法”耗时
>  
>* 对协议复杂度分析很重要
>  
>
>* * *
>
>### 2️⃣ 复制左操作数作为结果基底
>
>```rust
>let mut res = self.clone();
>```
>
>📌 数学含义：
>
>> 先令
>>
>> $$res = P$$
>>
>> 然后把 `Q` 的所有项加进来
>
>这样避免：
>
>* 重新构造一个空多项式
>  
>* 重复拷贝 P 的内容
>  
>
>* * *
>
>### 3️⃣ 遍历 `other`（Q）的所有乘积项
>
>```rust
>for products in other.products.iter() {
>```
>
>你要知道 `VirtualPolynomial` 的内部结构大概是：
>
>```text
>VirtualPolynomial
>├── products: Vec<(coefficient, indices)>
>├── flattened_ml_extensions: Vec<DenseMultilinearExtension>
>```
>
>所以：
>
>* `products` 是：
>  
>    ```rust
>    (coeff, Vec<usize>)
>    ```
>    
>* 表示：
>  
>    $$coeff · \prod_{i \in indices} mle_i$$
>
>* * *
>
>### 4️⃣ 把索引转换成真正的多项式对象
>
>```rust
>let cur: Vec<Arc<DenseMultilinearExtension<F>>> = products
>    .1
>    .iter()
>    .map(|&x| other.flattened_ml_extensions[x].clone())
>    .collect();
>```
>
>这一段是**核心逻辑**。
>
>#### 发生了什么？
>
>* `products.1` 是：
>  
>    ```rust
>    Vec<usize>
>    ```
>    
>    👉 是索引列表
>    
>* 每个 `x` 表示：
>  
>    ```text
>    flattened_ml_extensions[x]
>    ```
>    
>* `.clone()`：
>  
>    * 复制的是 `Arc`
>      
>    * **不复制多项式本体**
>      
>    * 非常高效
>      
>
>📌 数学意义：
>
>> 取出这个乘积项中所有参与相乘的多线性多项式
>
>* * *
>
>### 5️⃣ 把这个乘积项加进结果多项式
>
>```rust
>res.add_mle_list(cur, products.0)
>    .expect("add product failed");
>```
>
>含义是：
>
>* `cur`：乘积里的多线性多项式列表
>  
>* `products.0`：该乘积项的系数
>  
>
>数学上相当于：
>
>$$res \;+=\; products.0 \cdot \prod_{f \in cur} f$$
>
>如果失败（变量数不一致等），直接 panic。
>
>* * *
>
>### 6️⃣ 结束计时
>
>```rust
>end_timer!(start);
>```
>
>* * *
>
>### 7️⃣ 返回结果
>
>```rust
>res
>```
>



```rust
impl<F: PrimeField> VirtualPolynomial<F> {
    /// 创建一个具有`num_variables`个变量的空虚拟多项式。
    pub fn new(num_variables: usize) -> Self {
        VirtualPolynomial {
            aux_info: VPAuxInfo {
                max_degree: 0,
                num_variables,
                phantom: PhantomData,
            },
            products: Vec::new(),
            flattened_ml_extensions: Vec::new(),
            raw_pointers_lookup_table: HashMap::new(),
        }
    }
```

>#### 一、`impl<F: PrimeField> VirtualPolynomial<F>`
>
>```rust
>impl<F: PrimeField> VirtualPolynomial<F> {
>    ...
>}
>```
>
>含义：
>
>* 给 **泛型类型 `VirtualPolynomial<F>`** 实现方法
>  
>* 要求 `F` 是一个 **素数域**（`PrimeField`）
>  
>* 也就是说：  
>    👉 这个实现适用于 `VirtualPolynomial<Fr>`、`VirtualPolynomial<Fp>` 等
>
>* * *
>
>#### 二、函数签名：`new`
>
>```rust
>/// 创建一个具有`num_variables`个变量的空虚拟多项式。
>pub fn new(num_variables: usize) -> Self
>```
>
>* `pub`：对外可见
>  
>* `num_variables`：
>  
>    * 表示这个多项式**理论上有多少个变量**
>      
>    * 在 Sumcheck / PolyIOP 中，这个值决定：
>      
>        * 需要跑多少轮
>          
>        * 每轮固定一个变量
>    
>* 返回值 `Self`：
>  
>    * 就是 `VirtualPolynomial<F>`
>      
>
>* * *
>
>#### 三、函数体逐字段解释
>
>```rust
>VirtualPolynomial {
>```
>
>开始构造一个 `VirtualPolynomial` 结构体。
>
>* * *
>
>### 1️⃣ `aux_info`
>
>```rust
>aux_info: VPAuxInfo {
>    max_degree: 0,
>    num_variables,
>    phantom: PhantomData,
>},
>```
>
>#### 含义
>
>这是**辅助信息（Aux Info）**，提前记录多项式的“规格”。
>
>* `max_degree: 0`
>  
>    * 当前多项式的最大次数
>      
>    * 因为这是一个 **空多项式**，所以次数先设为 0
>      
>    * 后面每添加一个乘积项，都会更新它
>    
>* `num_variables`
>  
>    * 多项式的变量数（你传进来的参数）
>      
>    * 在协议中非常重要
>    
>* `phantom: PhantomData`
>  
>    * Rust 泛型占位
>      
>    * 告诉编译器：这个结构体逻辑上依赖 `F`
>      
>    * **不占内存，仅用于类型系统**
>      
>
>* * *
>
>### 2️⃣ `products`
>
>```rust
>products: Vec::new(),
>```
>
>* 存放：
>  
>    ```text
>    (系数 c_i, 多线性多项式索引列表)
>    ```
>    
>* 现在是空的，因为：
>  
>    * 这是一个“空的虚拟多项式”
>      
>    * 还没往里面加任何项
>      
>
>* * *
>
>### 3️⃣ `flattened_ml_extensions`
>
>```rust
>flattened_ml_extensions: Vec::new(),
>```
>
>* 存放所有被引用的 **DenseMultilinearExtension**
>  
>* 现在为空：
>  
>    * 因为还没有任何多线性多项式被加入
>      
>
>* * *
>
>### 4️⃣ `raw_pointers_lookup_table`
>
>```rust
>raw_pointers_lookup_table: HashMap::new(),
>```
>
>* 从多线性多项式的**原始指针** → 索引
>  
>* 用于：
>  
>    * 防止重复存储同一个多线性多项式
>      
>    * 快速查找
>    
>* 初始化为空
>  
>





```rust
  /// 根据最大似然估计（MLE）及其系数创建一个新的虚拟多项式。
    pub fn new_from_mle(mle: &Arc<DenseMultilinearExtension<F>>, coefficient: F) -> Self {
        let mle_ptr: *const DenseMultilinearExtension<F> = Arc::as_ptr(mle);
        let mut hm = HashMap::new();
        hm.insert(mle_ptr, 0);

        VirtualPolynomial {
            aux_info: VPAuxInfo {
                // 最大次数是任何单个变量的最大次数
                max_degree: 1,
                num_variables: mle.num_vars,
                phantom: PhantomData,
            },
            //这里的`0`指向`flattened_ml_extensions`的第一个多项式
            products: vec![(coefficient, vec![0])],
            flattened_ml_extensions: vec![mle.clone()],
            raw_pointers_lookup_table: hm,
        }
    }
```

>#### 一、这个函数在“数学上”干嘛？
>
>```rust
>pub fn new_from_mle(
>    mle: &Arc<DenseMultilinearExtension<F>>,
>    coefficient: F
>) -> Self
>```
>
>**数学含义：**
>
>> 用一个多线性多项式 $P(x_1,\dots,x_n)$  
>> 构造虚拟多项式
>>
>> $$f(x_1,\dots,x_n) = c \cdot P(x_1,\dots,x_n)$$
>
>也就是：
>
>* **只有一个乘积项**
>  
>* **乘积里只有一个因子**
>  
>* 这是最简单的 VirtualPolynomial
>  
>
>* * *
>
>#### 二、为什么要用 `Arc<DenseMultilinearExtension<F>>`？
>
>* `DenseMultilinearExtension` 是真实存数据的 MLE
>  
>* `VirtualPolynomial` 只是：
>  * **引用这些 MLE**
>      
>    * **用索引组织它们的乘积结构**
>    
>* `Arc`：
>  * 允许多个虚拟多项式 / 协议轮次共享同一个 MLE
>      
>    * 避免复制指数级大小的数据
>      
>
>* * *
>
>#### 三、逐行解释代码
>
>* * *
>
>### 1️⃣ 拿到 MLE 的“唯一身份”：指针
>
>```rust
>let mle_ptr: *const DenseMultilinearExtension<F> = Arc::as_ptr(mle);
>```
>
>* `Arc::as_ptr(mle)`：
>  
>    * 拿到 **堆上那份 MLE 的原始指针**
>    
>* 用途：
>  
>    * 后面放进 `raw_pointers_lookup_table`
>      
>    * 用来判断：
>      
>        > “这个 MLE 我是不是已经存过了？”
>        
>
>⚠️ **重要**：  
>不是用值比较，而是用 **内存地址** 比较，O(1)
>
>* * *
>
>### 2️⃣ 初始化查找表
>
>```rust
>let mut hm = HashMap::new();
>hm.insert(mle_ptr, 0);
>```
>
>意思是：
>
>> “这个 MLE 在 `flattened_ml_extensions` 里的索引是 0”
>
>为后面 **复用 MLE** 做准备。
>
>* * *
>
>### 3️⃣ 构造 `VirtualPolynomial`
>
>```rust
>VirtualPolynomial {
>```
>
>* * *
>
>### 4️⃣ `aux_info`
>
>```rust
>aux_info: VPAuxInfo {
>    max_degree: 1,
>    num_variables: mle.num_vars,
>    phantom: PhantomData,
>},
>```
>
>#### 解释
>
>* `num_variables: mle.num_vars`
>  
>    * 这个多项式的变量数
>      
>    * 必须和 MLE 一致
>    
>* `max_degree: 1`
>  
>    * **非常重要**
>      
>    * 因为：
>      
>        * 多线性多项式中
>          
>        * 每个变量次数 ≤ 1
>        
>    * 这里只有一个因子，没有乘法叠加
>      
>
>📌 在 Sumcheck 中：
>
>* 每一轮得到的单变量多项式次数 ≤ `max_degree`
>  
>
>* * *
>
>### 5️⃣ `products`
>
>```rust
>products: vec![(coefficient, vec![0])],
>```
>
>这是 **VirtualPolynomial 的核心表示**。
>
>#### 数学解释
>
>```text
>(c, [0])
>```
>
>意思是：
>
>> $$c \times \text{flattened\_ml\_extensions}[0]$$
>
>即：
>
>$$f = c \cdot P_0$$
>
>* `coefficient`：系数 $c$
>  
>* `vec![0]`：
>  
>    * 乘积因子索引列表
>      
>    * 这里只有一个因子
>      
>
>* * *
>
>### 6️⃣ `flattened_ml_extensions`
>
>```rust
>flattened_ml_extensions: vec![mle.clone()],
>```
>
>* 真正存 MLE 的地方
>  
>* clone 的是 `Arc`：
>  
>    * **不复制多项式数据**
>      
>    * 只是引用计数 +1
>      
>
>* * *
>
>### 7️⃣ `raw_pointers_lookup_table`
>
>```rust
>raw_pointers_lookup_table: hm,
>```
>
>* 用于：
>  
>    * 后续添加新的乘积项时
>      
>    * 快速判断某个 MLE 是否已经存在
>    
>* 这是 VirtualPolynomial **高效的关键**
>  
>



```rust
	/// 向自身添加多个多线性扩展的乘积
    /// 若列表为空，或多线性扩展（MLE）的`num_vars`与自身不同，则返回错误。
    /// 这些多线性扩展将先相乘，然后再与标量`coefficient`相乘。
    pub fn add_mle_list(
        &mut self,
        mle_list: impl IntoIterator<Item = Arc<DenseMultilinearExtension<F>>>,
        coefficient: F,
    ) -> Result<(), ArithErrors> {
        let mle_list: Vec<Arc<DenseMultilinearExtension<F>>> = mle_list.into_iter().collect();
        let mut indexed_product = Vec::with_capacity(mle_list.len());

        if mle_list.is_empty() {
            return Err(ArithErrors::InvalidParameters(
                "input mle_list is empty".to_string(),
            ));
        }
	
        self.aux_info.max_degree = max(self.aux_info.max_degree, mle_list.len());

        for mle in mle_list {
            if mle.num_vars != self.aux_info.num_variables {
                return Err(ArithErrors::InvalidParameters(format!(
                    "product has a multiplicand with wrong number of variables {} vs {}",
                    mle.num_vars, self.aux_info.num_variables
                )));
            }

            let mle_ptr: *const DenseMultilinearExtension<F> = Arc::as_ptr(&mle);
            if let Some(index) = self.raw_pointers_lookup_table.get(&mle_ptr) {
                indexed_product.push(*index)
            } else {
                let curr_index = self.flattened_ml_extensions.len();
                self.flattened_ml_extensions.push(mle.clone());
                self.raw_pointers_lookup_table.insert(mle_ptr, curr_index);
                indexed_product.push(curr_index);
            }
        }
        self.products.push((coefficient, indexed_product));
        Ok(())
    }
```

>#### 一、函数在“数学上”做什么？
>
>### 注释已经说得很准，我用数学语言再翻译一次：
>
>```rust
>add_mle_list(mle_list, coefficient)
>```
>
>等价于在虚拟多项式 `VP` 中 **加上一项**：
>
>$$VP(x_1,\dots,x_n) \;+=\;  
>coefficient \cdot \prod_{P \in mle\_list} P(x_1,\dots,x_n)$$
>
>其中：
>
>* 每个 `P` 是一个 **多线性多项式（MLE）**
>  
>* `coefficient ∈ F`
>
>* * *
>
>#### 二、整体流程（先看大图）
>
>```text
>输入：一组 MLE + 一个系数
>│
>├─ 检查：不能为空
>├─ 更新 VP 的最大次数信息
>├─ 对每个 MLE：
>│   ├─ 检查变量数一致
>│   ├─ 是否已经存过？
>│   │   ├─ 是 → 复用索引
>│   │   └─ 否 → 存进 flattened_ml_extensions
>│   └─ 记录索引
>│
>└─ 把 (coefficient, [索引们]) 加入 products
>```
>
>* * *
>
>#### 三、逐段详细解释（逐行）
>
>* * *
>
>### ① 函数签名
>
>```rust
>pub fn add_mle_list(
>    &mut self,
>    mle_list: impl IntoIterator<Item = Arc<DenseMultilinearExtension<F>>>,
>    coefficient: F,
>) -> Result<(), ArithErrors>
>```
>
>* `&mut self`  
>    👉 修改当前 `VirtualPolynomial`
>    
>* `mle_list`  
>    👉 一组多线性多项式（共享指针）
>    
>* `coefficient`  
>    👉 这个乘积项的系数
>    
>* 返回 `Result`  
>    👉 参数不合法就报错
>    
>
>* * *
>
>### ② 把输入统一收集成 Vec
>
>```rust
>let mle_list: Vec<Arc<DenseMultilinearExtension<F>>> =
>    mle_list.into_iter().collect();
>```
>
>目的：
>
>* 外部接口灵活（IntoIterator）
>  
>* 内部实现统一（Vec）
>  
>
>* * *
>
>### ③ 用来存“当前乘积”的索引
>
>```rust
>let mut indexed_product = Vec::with_capacity(mle_list.len());
>```
>
>之后会变成：
>
>```text
>indexed_product = [i1, i2, i3, ...]
>```
>
>表示：
>
>```text
>flattened_ml_extensions[i1] *
>flattened_ml_extensions[i2] *
>...
>```
>
>* * *
>
>### ④ 空列表直接报错
>
>```rust
>if mle_list.is_empty() {
>    return Err(ArithErrors::InvalidParameters(
>        "input mle_list is empty".to_string(),
>    ));
>}
>```
>
>数学上：
>
>* 没有乘数
>  
>* 没有意义（不是常数项）
>  
>
>* * *
>
>### ⑤ 更新“最大次数”信息
>
>```rust
>self.aux_info.max_degree =
>    max(self.aux_info.max_degree, mle_list.len());
>```
>
>📌 **非常重要的一行**
>
>原因：
>
>* 每个 MLE 对任意变量的次数 ≤ 1
>  
>* k 个 MLE 相乘后：
>  
>    * 每个变量的次数 ≤ k
>    
>* 所以：
>  
>    * 当前项的“单变量最大次数” = `mle_list.len()`
>      
>
>👉 Sumcheck 需要这个上界
>
>* * *
>
>### ⑥ 遍历每个 MLE
>
>```rust
>for mle in mle_list {
>```
>
>* * *
>
>#### ⑥.1 检查变量数量一致
>
>```rust
>if mle.num_vars != self.aux_info.num_variables {
>    return Err(ArithErrors::InvalidParameters(format!(
>        "product has a multiplicand with wrong number of variables {} vs {}",
>        mle.num_vars, self.aux_info.num_variables
>    )));
>}
>```
>
>原因：
>
>* 你不能把：
>  
>    * 3 变量多项式
>      
>    * 和 5 变量多项式
>      
>    * 相乘
>      
>
>Sumcheck 的变量空间必须一致。
>
>* * *
>
>#### ⑥.2 取原始指针作为“唯一身份”
>
>```rust
>let mle_ptr: *const DenseMultilinearExtension<F> = Arc::as_ptr(&mle);
>```
>
>这是：
>
>* **去重的关键**
>  
>* Arc 指针地址 = 多项式身份 ID
>  
>
>* * *
>
>#### ⑥.3 查是否已经存过这个 MLE
>
>```rust
>if let Some(index) = self.raw_pointers_lookup_table.get(&mle_ptr) {
>    indexed_product.push(*index)
>}
>```
>
>如果已经出现过：
>
>* 不重复存
>  
>* 只复用索引
>  
>
>* * *
>
>#### ⑥.4 如果是新的 MLE
>
>```rust
>else {
>    let curr_index = self.flattened_ml_extensions.len();
>    self.flattened_ml_extensions.push(mle.clone());
>    self.raw_pointers_lookup_table.insert(mle_ptr, curr_index);
>    indexed_product.push(curr_index);
>}
>```
>
>做了三件事：
>
>1. 分配新索引
>  
>2. 存入 `flattened_ml_extensions`
>  
>3. 建立指针 → 索引的映射
>  
>4. 把索引记进当前乘积
>  
>
>* * *
>
>### ⑦ 把这一项加入虚拟多项式
>
>```rust
>self.products.push((coefficient, indexed_product));
>```
>
>此时：
>
>```rust
>products.push((
>    c,
>    vec![i1, i2, i3]
>))
>```
>
>表示数学对象：
>
>$$c \cdot P_{i1} \cdot P_{i2} \cdot P_{i3}$$
>
>* * *
>
>### ⑧ 返回成功
>
>```rust
>Ok(())
>```
>
>* * *
>
>###3 四、一个完整的例子（非常重要）
>
>### 假设你要表示：
>
>$$f = 3 \cdot A \cdot B + 5 \cdot B \cdot C$$
>
>### 内部状态会变成：
>
>```text
>flattened_ml_extensions = [A, B, C]
>
>products = [
>    (3, [0, 1]),  // A * B
>    (5, [1, 2])   // B * C
>]
>```
>
>而不会重复存 B。
>
>* * *
>
>#### 一、先设定“数学背景”
>
>假设我们在素域 **F** 上，有 **2 个变量**：
>
>$$(x_1, x_2)$$
>
>现在我们有 **3 个多线性多项式（MLE）**：
>
>* * *
>
>### 📌 多线性多项式 A
>
>$$A(x_1, x_2) = x_1$$
>
>* * *
>
>### 📌 多线性多项式 B
>
>$$B(x_1, x_2) = x_2$$
>
>* * *
>
>### 📌 多线性多项式 C
>
>$$C(x_1, x_2) = x_1 + x_2$$
>
>它们在 arkworks 中都表示为：
>
>```rust
>Arc<DenseMultilinearExtension<F>>
>```
>
>* * *
>
>#### 二、初始化一个空的 VirtualPolynomial
>
>```rust
>let mut vp = VirtualPolynomial::<F>::new(2);
>```
>
>此时内部状态是：
>
>```text
>aux_info:
>  num_variables = 2
>  max_degree    = 0
>
>flattened_ml_extensions = []
>
>raw_pointers_lookup_table = {}
>
>products = []
>```
>
>* * *
>
>#### 三、第一次调用 add_mle_list
>
>#### 🔹 代码
>
>```rust
>vp.add_mle_list(vec![A.clone(), B.clone()], F::from(3u64))?;
>```
>
>* * *
>
>#### 🔹 数学含义
>
>你在做的是：
>
>$$VP(x_1,x_2) \;+=\; 3 \cdot A(x_1,x_2) \cdot B(x_1,x_2)$$
>
>也就是：
>
>$$VP = 3 \cdot x_1 x_2$$
>
>* * *
>
>#### 🔹 代码内部发生了什么？
>
>### ① mle_list 不为空 ✔
>
>```rust
>mle_list.len() = 2
>```
>
>* * *
>
>### ② 更新 max_degree
>
>```rust
>max_degree = max(0, 2) = 2
>```
>
>* * *
>
>### ③ 处理 A
>
>* A 还没出现过
>  
>* 分配索引 `0`
>  
>
>```text
>flattened_ml_extensions = [A]
>raw_pointers_lookup_table = { ptr(A) → 0 }
>indexed_product = [0]
>```
>
>* * *
>
>### ④ 处理 B
>
>* B 还没出现过
>  
>* 分配索引 `1`
>  
>
>```text
>flattened_ml_extensions = [A, B]
>raw_pointers_lookup_table = {
>  ptr(A) → 0,
>  ptr(B) → 1
>}
>indexed_product = [0, 1]
>```
>
>* * *
>
>### ⑤ 存入 products
>
>```rust
>products.push((3, vec![0, 1]));
>```
>
>* * *
>
>#### 🔹 此时 VirtualPolynomial 的状态
>
>```text
>flattened_ml_extensions = [A, B]
>
>products = [
>  (3, [0, 1])
>]
>```
>
>👉 表示：
>
>$$3 \cdot A \cdot B$$
>
>* * *
>
>#### 四、第二次调用 add_mle_list（复用 B）
>
>## 🔹 代码
>
>```rust
>vp.add_mle_list(vec![B.clone(), C.clone()], F::from(5u64))?;
>```
>
>* * *
>
>#### 🔹 数学含义
>
>$$VP \;+=\; 5 \cdot B(x_1,x_2) \cdot C(x_1,x_2)$$
>
>* * *
>
>#### 🔹 代码内部变化
>
>### ① mle_list.len() = 2
>
>```rust
>max_degree = max(2, 2) = 2
>```
>
>* * *
>
>### ② 处理 B
>
>* B 已经存在
>  
>* 查表得到索引 `1`
>  
>
>```text
>indexed_product = [1]
>```
>
>* * *
>
>### ③ 处理 C
>
>* C 是新的
>  
>* 分配索引 `2`
>  
>
>```text
>flattened_ml_extensions = [A, B, C]
>raw_pointers_lookup_table += { ptr(C) → 2 }
>indexed_product = [1, 2]
>```
>
>* * *
>
>### ④ 加入 products
>
>```rust
>products.push((5, vec![1, 2]));
>```
>
>* * *
>
>#### 五、最终 VirtualPolynomial 的完整结构
>
>```text
>flattened_ml_extensions = [
>  0 → A(x1,x2),
>  1 → B(x1,x2),
>  2 → C(x1,x2)
>]
>
>products = [
>  (3, [0, 1]),   // 3 * A * B
>  (5, [1, 2])    // 5 * B * C
>]
>
>aux_info.max_degree = 2
>```
>
>* * *
>
>#### 六、对应的“真实数学多项式”
>
>$$
>\begin{aligned}  
>VP(x_1,x_2)  
>&= 3 \cdot A \cdot B + 5 \cdot B \cdot C \\  
>&= 3 \cdot x_1 x_2 + 5 \cdot x_2 (x_1 + x_2)  
>\end{aligned}
>$$
>
>⚠️ **注意**：  
>这个多项式 **并没有被展开**，而是“虚拟地”存着。
>
>* * *
>
>
