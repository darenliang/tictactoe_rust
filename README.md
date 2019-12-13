# A simple Tic Tac Toe engine written in Rust
This tictactoe engine written in Rust and uses an minimax implementation.

To build and run the program:
```
cd src
rustc main.rs
./main           <= For Linux and macOS
main.exe       <= For Windows
```

Format:
- First input (1: Player, -1: Engine) indicates who moves first
- Later inputs (0 - 8) indicate positions
- Output who wins (1: Player, -1: Engine, 0: Tie)

Positions:
```
0 1 2
3 4 5
6 7 8
```
