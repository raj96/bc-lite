# bc-lite

A lite version of Basic Calculator (bc) from GNU/Linux written in Rust

## Example

```
echo "1+2-5*3/6" | cargo run
bc-lite v1.0
1+2-5*3/6 = -1
```

## Changelog

[18/05/2025]
* Initial commit
* Adds a left to right expression evaluator using `std::vec::Vec`

