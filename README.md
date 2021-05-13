# retrogame-rust

The idea is to implement a game similar to Atari Breakout, just to learn Rust in a fun way.

## TODO

#### UI
 - [ ] Add score
 - [ ] Add game over screen
 
#### Graphics
 - [ ] Fix precision errors causing visual glitches
 - [ ] Change color palette to be similar to Breakout

#### Gameplay
 - [ ] Limit ball initial direction
 - [ ] Add "effect" to ball
 - [ ] Add lifes?
 
 #### Sound
 - [ ] Ball collision sound
 - [ ] Background music?
  
## Learning Opportunities
- Profiling (Windows Performance Analyzer)
- Testing (Rust unit tests, commit only if all tests are ok)

# Project mini-Cloud

The main goal is to stream the game at 60 fps using a Raspberry Pi as a server. 
The client should only send input, decompress, and decode data from the server. 
