# RSnes
This is my mostly finished NES emulator, written in Rust.
The CPU, PPU, and mappers are original, but the APU is implemented using blargg's APU library (http://www.slack.net/~ant/libs/audio.html).
It's definitely not as accurate as other emulators are, but you can still fire it up and play some Mario or Zelda.

## Usage
rsnes [filename] [(optional) savefile]

## Keybindings
### Player 1:

Z -> A

X -> B

RShift -> Select

Enter -> Return

Arrow Keys -> Directional Buttons

### Player 2:

A -> A

S -> B

F -> Select

D -> Start

IJKL -> Directional

1...9 -> take savestate n

F1...F9 -> load savestate n
