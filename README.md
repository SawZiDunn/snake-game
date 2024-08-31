# Snake-Game-Rust

Midterm Group Project For Elementary System Programming at KMITL, SE67

Saw Zi Dunn - Email: 67011653@kmitl.ac.th

Phone Myat Pyae Sone - Email: 67011642@kmitl.ac.th

## Usage

```
$ cargo run
```

## Game Concept (Introduction)

"Snake with Bombs" is a classic snake game with a twist. In this version, the player controls a snake that grows in length by consuming food items while navigating through the screen.

However, the snake must avoid bombs that randomly appear and can shorten its length if collided with.

The game incorporates a win condition based on the snake's length and a loss condition involving collisions with walls, bombs, or itself.

## How to Play

The snake moves in the direction of its current heading and the user controls it by pressing left, right, bottom, and up keys through the keyboard.

The snake grows in length by one segment when it eats food items that appear at random locations on the screen.

Bombs appear randomly on the screen and can cause the snake to shorten by one segment if they are collided with.

## Win/Loss Conditions

The player wins the game if the snake's length reaches 10 segments. When this happens, a "You Win!" message is displayed, and the game loop ends.

The game ends if the snake's head collides with the wall or the snake's length becomes 0.

If the snake's head collides with any other part of its body, the game ends.

If the snake's length is 1 (i.e., the snake's body has only one segment) and it collides with a bomb, the game ends immediately.

If the snake's length is more than 1, it loses one segment (the tail segment) and the bomb is hidden. The bomb will reappear again after a certain amount of time at a random location within the border (wall).
