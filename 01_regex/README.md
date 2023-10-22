```bash
cargo run -- ayy ayylmao # true
cargo run -- ^ayy ayylmao # true
cargo run -- ^lmao ayylmao # false
cargo run -- lmao\$ ayylmao # true
cargo run -- a.y ayylmao # true
cargo run -- a.a ayylmao # false
cargo run -- z\* ayylmao # true
cargo run -- zz\* ayylmao # false
```
