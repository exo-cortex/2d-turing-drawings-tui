# 2d Turing Machine for the terminal.

This project runs a 2d turing machine where the (1-dimensional) tape is replaced by a 2-dimensional canvas and the tape head now moves in 4 directions (Up, Down, Left, Right). Each Symbol is represented by a different combination color/character.

This is much inspired by this project [Turing Drawings](https://maximecb.github.io/Turing-Drawings/#) (which was also forked [here](https://github.com/darius/Turing-Drawings) to make it *drastically* faster and more versatile.)

# Implementation
This uses Ratatui and was made using the event-driven template.

# ToDo
- Remove hardcoded constants for canvas-dimensions and make it resizable.
- Cleanup ... there is an unnecessary layer of encapsulation between the actual turing machine and the wrapper that makes everything very complicated and ugly.
- Lots...
