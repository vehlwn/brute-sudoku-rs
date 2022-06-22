# brute-sudoku-rs

Same as in [here](https://github.com/vehlwn/BruteSudoku) in Rust, but implements only recursive descent algorithm.

Client-server app with protocol defined in [here](./backend-rs/src/protocol.rs).

## Build

```bash
$ mkdir owncert
# Put your custom certificate and key from a valid CA to ./owncert/certificate.pem and
# ./owncert/key.pem.
$ docker-compose build && docker-compose up
# Now open https://your_domain:5000/ in a browser or post data via curl:
$ curl -H "content-type: application/json" https://your_domain:5000/recursive_solver -d '{"table":" . . .  . . .  . . .  . . .  . . 3  . 8 5 . . 1  2 . .  . . .  . . .  5 . 7  . . .  . . 4  . . .  1 . .  . 9 .  . . .  . . .  5 . .  . . .  . 7 3 . . 2  . 1 .  . . .  . . .  . 4 .  . . 9 ", "output_format":"Compact"}'
```
