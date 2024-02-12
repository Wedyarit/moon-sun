
![Logo](https://i.imgur.com/sFHntW1.png)

![Rust Version](https://img.shields.io/badge/Rust-1.76.0-orange) ![Version](https://img.shields.io/badge/Version-0.1.0-blue) [![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/) 

# Moon & Sun

**Moon & Sun** is a captivating game simulator where moon and sun engage in a perpetual rivalry for dominance over the game board. The essence of the game lies in the strategic interplay between the celestial bodies. However, the outcome is governed by the laws of probability, adding an element of unpredictability to each encounter.

## Demo


![Demo](https://i.imgur.com/MLEcF5m.gif)
## Features

- Random Assignment of Initial Direction Vectors: Each game session begins with the objects being assigned random initial direction vectors, adding an element of unpredictability to gameplay.

- Collision Detection and Color Composition Alteration: The game meticulously tracks collisions between objects, dynamically adjusting their color composition upon impact. 

- Boundary Tracking: The game monitors instances where objects exit the boundaries of the playing field, ensuring fair gameplay and preventing entities from venturing beyond permissible limits.

- Adjustable Game Speed: Players have the freedom to customize the game's pace according to their preferences, enabling them to control the tempo of the gameplay experience.

- Display of Captured Grid Cells: The game interface includes a visual representation of the number of grid cells captured by each player, providing valuable feedback and strategic insight into the progress of the game.

## Run Locally

Clone the project

```bash
  git clone https://github.com/wedyarit/moon-sun.git
```

Go to the project directory

```bash
  cd moon-sun
```

Install dependencies

```bash
  cargo build
```

Compile and run

```bash
  cargo run
```


## Roadmap

- Dynamic Background Shading: Implement a feature to dynamically adjust the background shading based on the number of captured grid cells. As players progress and capture more cells, the background will darken or lighten accordingly, enhancing visual feedback and immersion.

- Option for Normal Vector Impulse Calculation: Introduce an option for players to utilize normal vector calculations for impulse-based interactions. 

- Collision Detection Between Sun and Moon: Implement collision tracking between the sun and moon entities, introducing unique effects or animations upon collision. 

- Musical Accompaniment: Integrate musical accompaniment to complement gameplay, setting the mood and enhancing immersion for players
## Authors

- [@wedyarit](https://www.github.com/wedyarit) - Raw concept & developing


## License

[MIT](https://choosealicense.com/licenses/mit/)

