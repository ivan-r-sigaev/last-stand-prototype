# Last Stand \[PROTOTYPE\]
This is my attempt to create a single-file web game as a part of a learning project.
# Details
- I used [`macroquad`][1] as library backend.
- I implemented ESC ([entity component component system][2]) with [sparse set compoenent pools][3].
- I implemented basic collision detection optimized using the [uniform collision grid][4].
- I created a bulid script to injects the `macroquad`'s code with the wasm file.
# Game
The game itself is heavily based upon [Brotato][5].

[1]: https://macroquad.rs/
[2]: https://en.wikipedia.org/wiki/Entity_component_system
[3]: https://skypjack.github.io/2020-08-02-ecs-baf-part-9/
[4]: https://peerdh.com/blogs/programming-insights/efficient-grid-based-collision-detection-in-2d-games
[5]: https://store.steampowered.com/app/1942280/Brotato/
