version: 0.5

BUG: 
The states of tetris which in both setInterval closure and addEventListener closure cannot be the same state at the same time.

and if i use RefCell, there would be other issues....

Seems i have to write use_state using rust!!!