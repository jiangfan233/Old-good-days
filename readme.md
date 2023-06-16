### Keep hungry, keep stupid.

##### install
```bash
cargo install wasm-pack && wasm-pack build --target web
```


##### version: 0.5

BUG: 
The states of tetris which in both setInterval closure and addEventListener closure cannot be the same state at the same time.

and if i use RefCell, there would be other issues....

Seems i have to write use_state using rust!!!

##### version: 0.6

2023.06.16: Just born
"My Tetris" has basic functions but lacks any optimization or beautification.

these [vodeos](https://www.youtube.com/playlist?list=PLtTT8p-gjGEdGzZ0ET2bwNnA6iP_mmmrv) helped me so much, thanks a lot.


