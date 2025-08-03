# bBreakout

[![Github Pages](https://img.shields.io/badge/github%20pages-121013?style=for-the-badge&logo=github&logoColor=white)](https://aleksuo.github.io/bBreakout/)
[![Build Status](https://github.com/Aleksuo/bBreakout/actions/workflows/on-push-main.yaml/badge.svg)](https://github.com/Aleksuo/bBreakout/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://opensource.org/licenses/MIT)


> A simple Breakout clone built in Rust using Bevy. [Playable in the browser](https://aleksuo.github.io/bBreakout/).

![Game gif](samples/bbreakout_sample.gif)

## Controls
- Move left: A
- Move right: D

## Features
- Gameplay mechanics:
  - Score based on destroyed tiles
  - Three lives that get depleted when the ball hits the bottom wall
  - Simple ball physics
  - Simple particle system that is used to spawn a trail for the ball
  - A ball speedup system that is also indicated by the color of the ball trail
  - Simple audio system for sound effects
- Multiple game states:
  - Main menu screen
  - Settings screen with global volume control
  - Game state
  - Game over screen
- Playable in the browser utilizing wasm
## Installation

1. Install Rust (via [rustup](https://rustup.rs/))
2. Install Node.js (via [nodejs.org](https://nodejs.org/))
3. Install project dependencies:
   ```bash
   npm install
   ```
## Running the Project

- To run the web application:
  ```bash
  npm run dev:webapp
  ```

- To run the Bevy game:
  ```bash
  npm run dev:game
  ```

## Building for GitHub Pages deployment

- To build the web application for GitHub Pages:
  ```bash
  npm run dist:gh-pages
  ```

## Made with
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Bevy](https://img.shields.io/badge/bevy-%23232326.svg?style=for-the-badge&logo=bevy&logoColor=white)](https://bevy.org/)
[![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?style=for-the-badge&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Angular](https://img.shields.io/badge/angular-%23DD0031.svg?style=for-the-badge&logo=angular&logoColor=white)](https://angular.dev/)
[![NPM](https://img.shields.io/badge/NPM-%23CB3837.svg?style=for-the-badge&logo=npm&logoColor=white)](https://www.npmjs.com/)


## License

TODO
