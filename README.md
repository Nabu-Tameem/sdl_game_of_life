# SDL Game of Life

An implementation of Conway's game of life using Rust for backend and SDL2 for user interface.

## Requirements

- SDL2 development libraries.

Ubuntu

```shell
apt install sdl2-dev
```

Arch

```shell
pacman -S sdl2
```

Solus

```shell
eopkg install -c sdl2-devel
```

## Features to be added

- ~~The ability to pause the universe.~~
- ~~The ability to move forwards one tick.~~
- ~~Increase the size of the world~~ (possibly "infinite" size universe implementation).
- Implement GPU acceleration for rendering the cells.
- ~~The ability to scroll through the universe.~~
- ~~The ability to toggle a cell (Turn a dead cell into a live cell, and vice versa) by clicking on it.~~
- ~~The ability to drag to draw new live cells.~~
- ~~The ability to drag to replace live cells with dead cells.~~
- ~~The ability to zoom in and out of the universe (Scaling the cells).~~
- ~~Support other display sizes for the window.~~
- The ability to change resolution.

## How to use

- SPACE: Toggle pause and resume.
- ESC: Close the program.
- Right Arrow: Progress one tick.
- Middle Mouse: Drag the universe.
- Right Mouse: Kill a cell.
- Left Mouse: Revive a cell.
- Scroll Up: Zoom in to the universe.
- Scroll Down: Zoom out of the universe.