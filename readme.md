### Keep hungry, keep stupid.

##### play on [desktop](https://jiangfan233.github.io/Old-good-days/)

##### install
```bash
cargo install wasm-pack && cargo check && cargo test && wasm-pack build --target web
```

##### version: 0.6

2023.06.16: Just born

"My Tetris" has basic functions but lacks any optimization or beautification.

##### version: 0.5

BUG: 
The states of tetris which in both setInterval closure and addEventListener closure cannot be the same state at the same time.

and if i use RefCell, there would be other issues....

Seems i have to write use_state using rust!!!

these [vodeos](https://www.youtube.com/playlist?list=PLtTT8p-gjGEdGzZ0ET2bwNnA6iP_mmmrv) helped me so much, thanks a lot.


