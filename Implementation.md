# Implementation Details

## Enums

### `Direction`

-   **Description**: Represents the direction of the snake's movement. It has four possible values: `Up`, `Down`, `Left`, and `Right`.

#### Methods

-   **`opposite(&self) -> Self`**: Returns the opposite direction of the current direction. For example, `Direction::Up` will return `Direction::Down`.

## Structures

### `Item`

-   **Description**: Represents an object in the game, such as the snake's body segment, food, or other items like bombs. It contains properties for its figure (a character), color, and position (x, y coordinates).

#### Methods

-   **`new(figure: char, color: Color, x: u16, y: u16) -> Self`**: Creates a new `Item` with the specified figure, color, and position.
-   **`print_item(&self)`**: Prints the item on the terminal at its current position with its specified color.

### `Bomb`

-   **Description**: A structure representing a bomb in the game. It includes an `Item` for the bomb's appearance, a visibility flag, and a timeout value that determines how long the bomb remains visible.

#### Methods

-   **`new_bomb(figure: char, color: Color, x: u16, y: u16, timeout: u64) -> Self`**: Creates a new bomb with the specified figure, color, position, and timeout duration.
-   **`print_bomb(&self)`**: Prints the bomb on the terminal if it is visible.
-   **`hide_bomb(&mut self)`**: Hides the bomb by setting its visibility to `false`.
-   **`reset_bomb(&mut self, x: u16, y: u16)`**: Resets the bomb's position to the given coordinates and makes it visible again.

### `Snake`

-   **Description**: Represents the snake in the game. It contains a vector of `Item` objects that make up the snake's body, along with the snake's current direction of movement.

#### Methods

-   **`new(head: Item) -> Self`**: Creates a new snake with the specified head `Item`. The initial direction is set to `Up`.
-   **`get_head(&self) -> &Item`**: Returns a reference to the head `Item` of the snake.
-   **`print_snake(&self)`**: Prints all segments of the snake on the terminal.
-   **`move_snake(&mut self, will_grow: bool)`**: Moves the snake in the current direction. If `will_grow` is `true`, the snake's length increases by one segment.
-   **`change_direction(&mut self, key: KeyCode)`**: Changes the snake's direction based on the input key, provided the new direction is not opposite to the current one.

## Functions

### `check_collision`

-   **Description**: Checks if two `Item` objects occupy the same position, indicating a collision. It returns `true` if the positions match, `false` otherwise. This function is mainly used for checking whether the sanke's head has collided with its own body or food or the bomb.

### `check_hit_wall`

-   **Description**: Checks if the snake's head has collided with the walls of the game area. It returns `true` if the head's coordinates are out of bounds, `false` otherwise.

### `initialize_screen`

-   **Description**: Sets up the terminal screen for the game, enabling raw mode and entering the alternate screen buffer. It hides the cursor to prepare for game rendering.

### `terminate_screen`

-   **Description**: Restores the terminal to its original state by leaving the alternate screen and showing the cursor. It also disables raw mode.

### `clear_screen`

-   **Description**: Clears the entire terminal screen, preparing it for fresh rendering of the game state.

### `print_border`

-   **Description**: Draws the game's border (wall) around the playable area using a specified character, typically `=`. It marks the boundaries within which the snake can move. The snake will not move outside this area.

### `print_status`

-   **Description**: Displays the current status of the game, such as the snake's length (which could represent the score or health points). This is typically printed outside the game area (border).

### `win`

-   **Description**: Displays a "**You Win!**" message on the screen when the player meets the winning condition (e.g., reaching a certain snake length).

### `print_game_over_screen`

-   **Description**: Displays the "GAME OVER" screen when the player loses. It also provides options to restart by pressing '**r**' or quit the game by pressing '**Esc**'.
