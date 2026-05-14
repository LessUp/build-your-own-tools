# 并发模型对比

本文档比较 Rust 和 Go 的并发编程模型。

## 核心理念

```mermaid
graph TB
    subgraph "Rust 并发"
        A[async/await] --> B[Future]
        B --> C[Tokio Runtime]
        C --> D[工作窃取]
        
        E[std::thread] --> F[1:1 线程]
        F --> G[OS 调度]
    end
    
    subgraph "Go 并发"
        H[goroutine] --> I[M:N 调度]
        I --> J[Go Runtime]
        J --> K[工作窃取]
        
        L[channel] --> M[CSP 模型]
        M --> N[消息传递]
    end
    
    style A fill:#3b82f6,color:#fff
    style H fill:#06b6d4,color:#fff
```

## Rust: async/await

### 基本用法

```rust
use tokio;

#[tokio::main]
async fn main() {
    // 并发执行多个任务
    let task1 = tokio::spawn(async {
        println!("Task 1");
        1
    });
    
    let task2 = tokio::spawn(async {
        println!("Task 2");
        2
    });
    
    // 等待结果
    let (r1, r2) = tokio::join!(task1, task2);
    println!("Results: {:?}, {:?}", r1, r2);
}
```

### Future 和异步函数

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// 手动实现 Future
struct Delay {
    duration: Duration,
}

impl Future for Delay {
    type Output = ();
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 实现轮询逻辑
        Poll::Ready(())
    }
}

// 使用 async fn
async fn fetch_data(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}
```

### 并发原语

```rust
use tokio::sync::{mpsc, Mutex, RwLock, Semaphore};

// Channel
let (tx, mut rx) = mpsc::channel(100);

tokio::spawn(async move {
    tx.send("hello".to_string()).await.unwrap();
});

while let Some(msg) = rx.recv().await {
    println!("Received: {}", msg);
}

// Mutex
let data = Mutex::new(vec![1, 2, 3]);
{
    let mut locked = data.lock().await;
    locked.push(4);
}

// Semaphore
let semaphore = Semaphore::new(3);
let permit = semaphore.acquire().await.unwrap();
// 限制并发数量
```

## Go: goroutine + channel

### goroutine

```go
// 启动 goroutine
go func() {
    fmt.Println("Hello from goroutine")
}()

// 带参数的 goroutine
func process(id int) {
    fmt.Printf("Processing %d\n", id)
}

for i := 0; i < 10; i++ {
    go process(i)
}
```

### Channel

```go
// 无缓冲 channel
ch := make(chan string)

// 发送和接收
go func() {
    ch <- "hello"  // 发送
}()

msg := <-ch  // 接收
fmt.Println(msg)

// 缓冲 channel
buffered := make(chan int, 10)

// 关闭 channel
close(ch)

// range 遍历
for msg := range ch {
    fmt.Println(msg)
}
```

### Select

```go
// 多路复用
select {
case msg := <-ch1:
    fmt.Println("From ch1:", msg)
case msg := <-ch2:
    fmt.Println("From ch2:", msg)
case <-time.After(time.Second):
    fmt.Println("Timeout")
default:
    fmt.Println("No data")
}
```

### Context

```go
import "context"

// 超时控制
ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
defer cancel()

select {
case <-ctx.Done():
    fmt.Println("Context cancelled:", ctx.Err())
case result := <-doWork(ctx):
    fmt.Println("Result:", result)
}

func doWork(ctx context.Context) <-chan string {
    out := make(chan string)
    go func() {
        defer close(out)
        for {
            select {
            case <-ctx.Done():
                return
            default:
                // 执行工作
                out <- "result"
            }
        }
    }()
    return out
}
```

## 对比分析

### 调度模型

| 方面 | Rust (Tokio) | Go |
|------|--------------|-----|
| 调度模型 | M:N (work stealing) | M:N (work stealing) |
| 栈大小 | 动态 (默认 2MB) | 初始 2KB，可增长 |
| 抢占式 | 是 (since 1.39+) | 是 |
| 创建成本 | 低 | 极低 |

### 数据共享

```mermaid
graph LR
    subgraph "Rust"
        A[共享状态] --> B[Mutex/RwLock]
        B --> C[安全访问]
        
        D[消息传递] --> E[mpsc channel]
        E --> F[所有权转移]
    end
    
    subgraph "Go"
        G[共享状态] --> H[sync.Mutex]
        H --> I[手动保护]
        
        J[消息传递] --> K[channel]
        K --> L[CSP 模型]
    end
    
    style C fill:#10b981,color:#fff
    style F fill:#10b981,color:#fff
```

### 错误处理

```rust
// Rust - Result 传播
async fn handle_request() -> Result<Response, Error> {
    let data = fetch_data().await?;  // 错误会传播
    let processed = process(data).await?;
    Ok(Response::new(processed))
}

// 使用 join! 处理多个结果
let (r1, r2) = tokio::join!(task1, task2);
if r1.is_err() || r2.is_err() {
    // 处理错误
}
```

```go
// Go - 显式错误检查
func handleRequest() (*Response, error) {
    data, err := fetchData()
    if err != nil {
        return nil, err
    }
    
    processed, err := process(data)
    if err != nil {
        return nil, err
    }
    
    return &Response{Data: processed}, nil
}
```

## 性能对比

### 任务创建

```mermaid
graph TB
    A[100万任务创建] --> B[Rust: ~100ms]
    A --> C[Go: ~50ms]
    
    B --> D[Tokio spawn]
    C --> E[goroutine]
    
    style B fill:#3b82f6,color:#fff
    style C fill:#06b6d4,color:#fff
```

### 上下文切换

| 场景 | Rust | Go |
|------|------|-----|
| async 切换 | ~50ns | - |
| goroutine 切换 | - | ~200ns |
| 线程切换 | ~1μs | ~1μs |

## 实际案例

### htop 并发设计

```rust
// Rust - async 数据采集
async fn collect_system_info() -> SystemInfo {
    let cpu = tokio::spawn(get_cpu_info());
    let mem = tokio::spawn(get_memory_info());
    let processes = tokio::spawn(get_process_list());
    
    let (cpu, mem, processes) = tokio::join!(cpu, mem, processes);
    
    SystemInfo {
        cpu: cpu.unwrap(),
        memory: mem.unwrap(),
        processes: processes.unwrap(),
    }
}
```

```go
// Go - goroutine 数据采集
func collectSystemInfo() SystemInfo {
    var info SystemInfo
    var wg sync.WaitGroup
    
    wg.Add(3)
    go func() {
        defer wg.Done()
        info.CPU = getCPUInfo()
    }()
    go func() {
        defer wg.Done()
        info.Memory = getMemoryInfo()
    }()
    go func() {
        defer wg.Done()
        info.Processes = getProcessList()
    }()
    
    wg.Wait()
    return info
}
```

## 最佳实践

### Rust

```rust
// 1. 使用 spawn_blocking 处理阻塞操作
let result = tokio::task::spawn_blocking(|| {
    // 阻塞操作
    heavy_computation()
}).await?;

// 2. 限制并发
let semaphore = Arc::new(Semaphore::new(10));
let mut tasks = vec![];

for url in urls {
    let permit = semaphore.clone().acquire_owned().await.unwrap();
    tasks.push(tokio::spawn(async move {
        let result = fetch(url).await;
        drop(permit);
        result
    }));
}

// 3. 优雅关闭
tokio::select! {
    result = work() => result,
    _ = shutdown_signal() => {
        println!("Shutting down");
        Ok(())
    }
}
```

### Go

```go
// 1. 使用 worker pool
func workerPool(ctx context.Context, jobs <-chan Job, results chan<- Result) {
    for i := 0; i < 10; i++ {
        go func() {
            for job := range jobs {
                select {
                case results <- process(job):
                case <-ctx.Done():
                    return
                }
            }
        }()
    }
}

// 2. 使用 errgroup
g, ctx := errgroup.WithContext(ctx)

g.Go(func() error {
    return task1(ctx)
})
g.Go(func() error {
    return task2(ctx)
})

if err := g.Wait(); err != nil {
    // 处理错误
}

// 3. 优雅关闭
server.Shutdown(ctx)
```

## 相关文档

- [对比研究概览](/comparison/) — 对比总览
- [内存模型对比](/comparison/memory) — 内存管理
- [错误处理对比](/comparison/errors) — 错误处理
