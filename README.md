# CHIP-8 Emulator
The emulator comes with 3 ROMs: **Space Invaders**, **Pong** and **Tetris**.
The default ROM is Space Invaders, but you can pass your own ROM as an argument.
```
cargo run -- --rom=location/to/your_rom.ch8
```
## Debug Mode
The emulator also has a debug mode to inspect the current state of itself.
You can run in debug mode by passing the argument as follows:
```
cargo run -- --debug
```
Once you ran in debug mode, you can type ***help*** to see the available commands.
## References
[Cowgod's Chip-8 Technical Reference]=(http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
