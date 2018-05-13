# The Travel Ghost

A very simple example to demonstrate how **Q-learning** works.

## Game Background

Long long time ago, there was a little ghost who trapped himself in an one-dimensional space, as shown below:

:office::office::office::office::office::office::office::office::office:

:bomb:======:ghost:=======:european_castle:                                 

:office::office::office::office::office::office::office::office::office:


- The goal for the ghost is to reach the castle as soon as possible.
- The ghost can go either **left** or **right**, one step at a time.
- The ghost cannot see anything; he will know whether he has reached the castle or the bomb only when he gets to that location.

## Requirements
- The Q-learning agent should not have any hard-coded knowledge.
- The environment should only give feedback when the agent reaches the castle or the bomb.
- For each step, the environment allows the agent to choose to move either left or right.

# How to run the program

```
> cargo run
```
of if you prefer to compile to binary
```
> cargo build --release
> ./target/release/q_learning
```

## Sample Output
```
============== Training Episode 99 ===============
Episode 99 State 1 Action →
Episode 99 State 2 Action →
Episode 99 State 3 Action →
Episode 99 State 4 Action →
Episode 99 State 5 Action →
Episode 99 State 6 Action →
Episode 99 State 7 Action ←
Episode 99 State 6 Action →
Episode 99 State 7 Action →
Episode 99 State 8 Action ←
Episode 99 State 7 Action →
Episode 99 State 8 Action →
Episode 99 State 9 Action →
Episode 99 State 10 Action →
Episode 99 State 11 Action →
============== Simulating with Trained Knowledge ===============
State 1 Action →
State 2 Action →
State 3 Action →
State 4 Action →
State 5 Action →
State 6 Action →
State 7 Action →
State 8 Action →
State 9 Action →
State 10 Action →
State 11 Action →
Simulation Results: Reached the castle!
```
