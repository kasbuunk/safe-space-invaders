# Bevy

## Prerequisites

Please prepare by following the following instructions. At least take the time to install and setup your development environment.

### Rust:

Follow the instructions of your preferred installation method on: https://www.rust-lang.org/tools/install

### Editor with Plugin

- VSCode with Rust-Analyzer Plugin
- RustRover by JetBrains
- Neovim with rust-analyzer lsp

## Preparation

Clone this repository and compile and run the project. Doing this in advance saves us time. Compilation can take some time, but incremental builds run fast! 

```sh
git clone git@lab.weave.nl:rust/bevy.git
cd bevy
cargo run
```

### Reading list

- Rust Book: https://doc.rust-lang.org/book/
- Bevy: https://bevyengine.org
    - Their [Learn](https://bevyengine.org/learn/) page has many resources to keep you busy.
    - Do read their [book](https://bevyengine.org/learn/book/introduction/), it's perhaps 20 minutes to browse through.
- Entity Component System
    - FAQ: https://github.com/SanderMertens/ecs-faq
    - Introductory guide: https://www.simplilearn.com/entity-component-system-introductory-guide-article

## What to build?
### Space Invaders

Not all features can be picked up immediately, this means that you have to be creative while programming it while your feature is depending on the others.
#### Must
- [ ] Empty Entities: Main Character, Alien, Shooting Alien & Castle
- [x] Physics library - A physics library is needed. gravity will be disabled and this library is only used for collision detection (maybe other features too for fun things)
- [ ] Character Movement - Move left right (**A** & **D**) through the Physics library (probably changing the velocity) 
  - [x] Make the player move right and left 
  - [ ] Make the aliens move too (similar behaviour like the gameplay of the actual game)
- [ ] Lives System: if less or equals zero, then rust custom logic (players death or alien death)
- [ ] UI
  - [ ] Score
  - [ ] High Score
  - [ ] Lives of the player
  - [ ] Game Over Screen
  - [ ] Start Screen
- [ ] Bullet - The movement has to be done through the physics library* Rest of the behaviour can be done by us (spawning & animation).  
  - [ ] Getting hit by bullet - Any entity can get hit and execute different behaviour. (player, enemy, castle)
- [ ] Waves
  - [ ] Levels (with different kind of waves)
- [ ] Sound effects
- [ ] Animations
  - [ ] Alien
  - [ ] Alien Shooting
  - [ ] Castle
  - [ ] Player

#### If everything goes fine (doubt) 
- [ ] Local Co-op (This is the easiest to implement)
- [ ] Aliens drop weapons on death
- [ ] Bosses

## Assets

For the designers, here is a website with some assets for both sound and images: https://kenney.nl/assets

## Disclaimer

Designing a game is hard. Building a game is hard. Rust is hard. Bevy has a learning curve. In short, expect to get stuck and have a drink while finding your way.
