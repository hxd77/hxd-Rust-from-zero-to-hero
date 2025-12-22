```rust
[package]
name = "arithmetic"
version = "0.1.0"
edition = "2021"
# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

>#### 一、包信息
>
>```toml
>[package]
>name = "arithmetic"
>version = "0.1.0"
>edition = "2021"
>```
>
>* `name` → crate 名字，其他 crate 可以用 `arithmetic` 引用它
>  
>* `version` → crate 版本
>  
>* `edition` → Rust 语言版本标准，这里是 2021
>



```rust
[dependencies]
ark-bls12-381 = { version = "0.4.0", default-features = false, features = [ "curve" ] }
ark-ff = { version = "^0.4.0", default-features = false }
ark-poly = { version = "^0.4.0", default-features = false }
ark-serialize =  { version = "^0.4.0", default-features = false }
ark-std = { version = "^0.4.0", default-features = false }
displaydoc = { version = "0.2.3", default-features = false }
rand_chacha = { version = "0.3.0", default-features = false }
rayon = { version = "1.5.2", default-features = false, optional = true }
```

>#### 二、依赖（dependencies）
>
>```toml
>[dependencies]
>ark-bls12-381 = { version = "0.4.0", default-features = false, features = [ "curve" ] }
>ark-ff = { version = "^0.4.0", default-features = false }
>ark-poly = { version = "^0.4.0", default-features = false }
>ark-serialize =  { version = "^0.4.0", default-features = false }
>ark-std = { version = "^0.4.0", default-features = false }
>displaydoc = { version = "0.2.3", default-features = false }
>rand_chacha = { version = "0.3.0", default-features = false }
>rayon = { version = "1.5.2", default-features = false, optional = true }
>```
>
>* **arkworks 相关库**：
>  
>    * `ark-bls12-381` → BLS12-381 曲线实现
>      
>    * `ark-ff` → 域元素（PrimeField / Fp）
>      
>    * `ark-poly` → 多项式操作（DensePolynomial / MLE）
>      
>    * `ark-serialize` → 序列化功能
>      
>    * `ark-std` → arkworks 自己的 std 替代（支持 no_std / parallel）
>    
>* **其他依赖**：
>  
>    * `displaydoc` → 用于宏生成文档 / 错误显示
>      
>    * `rand_chacha` → 随机数生成器
>      
>    * `rayon` → 并行计算（可选）
>      
>
>💡 `default-features = false` → 禁用 crate 的默认 feature，以减少冗余依赖或启用自定义 feature



```rust
[dev-dependencies]
ark-ec = { version = "^0.4.0", default-features = false }
criterion = "0.5.1"
```

>#### 三、开发依赖（dev-dependencies）
>
>```toml
>[dev-dependencies]
>ark-ec = { version = "^0.4.0", default-features = false }
>criterion = "0.5.1"
>```
>
>* 仅在 **测试 / 基准** 时才用
>  
>* `ark-ec` → 曲线的通用接口
>  
>* `criterion` → Rust 的基准测试框架
>



```rust
[features]
# default = [ "parallel", "print-trace" ]
default = ["parallel"]
parallel = [ 
    "rayon",
    "ark-std/parallel", 
    "ark-ff/parallel",  
    "ark-poly/parallel" 
    ]
print-trace = [ 
    "ark-std/print-trace" 
    ] 
```



>#### 四、Feature 配置
>
>```toml
>[features]
># default = [ "parallel", "print-trace" ]
>default = ["parallel"]
>parallel = [ 
>    "rayon",
>    "ark-std/parallel", 
>    "ark-ff/parallel",  
>    "ark-poly/parallel" 
>    ]
>print-trace = [ 
>    "ark-std/print-trace" 
>    ] 
>```
>
>* `default = ["parallel"]` → 默认启用 `parallel` feature
>  
>* `parallel` → 启用多线程 / 并行计算
>  
>* `print-trace` → 启用打印调试信息
>
>> Feature 可以理解为“可选模块”，用来控制是否启用并行或调试功能
>





```rust
[[bench]]
name = "mle_eval"
path = "benches/bench.rs"
harness = false
```

>#### 五、基准测试（bench）
>
>```toml
>[[bench]]
>name = "mle_eval"
>path = "benches/bench.rs"
>harness = false
>```
>
>* 定义一个基准测试目标
>  
>* `name` → 测试名称
>  
>* `path` → 测试文件路径
>  
>* `harness = false` → 不使用 Rust 默认的测试 harness（用于 Criterion 基准测试）