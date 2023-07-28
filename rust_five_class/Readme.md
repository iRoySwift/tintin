```title
请搜索相关文档，实现：一个简单的声明宏
并理解其代码结构，和编译过程。
可将作业内容按要求写在 github 内 此处提供 github 地址 供助教查看作业
```

## 实现 string::from 宏

```rust
#[macro_export]
macro_rules! string {
    ($a:expr) => {{
        let string = String::from($a);
        string
    }};
}

#[cfg(test)]
mod test {
    #[test]
    fn test_string() {
        assert_eq!(string!("Hello Rust!"), String::from("Hello Rust!"));
    }
}

```
