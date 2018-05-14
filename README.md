# The Traveling Ghost

A very simple example to demonstrate how **Q-learning** works (not DQL).

## The Game

Long long time ago, there was a little ghost who trapped himself in a one-dimensional space.

&#x1f3e2;&#x1f3e2;&#x1f3e2;&#x1f3e2;&#x1f3e2;&#x1f3e2;&#x1f3e2;&#x1f3e2;&#x1f3e2;&#x1f3e2;&#x1f3e2;&#x1f3e2;&#x1f3e2;

&#x1f4a3;======&#x1f47b;=========&#x1f3f0;                                 

&#x1f3e2;&#x1f3e2;&#x1f3e2;&#x1f3e2;&#x1f3e2;&#x1f3e2;&#x1f3e2;&#x1f3e2;&#x1f3e2;&#x1f3e2;&#x1f3e2;&#x1f3e2;&#x1f3e2;


## Rules

- The goal for the ghost is to reach the castle as soon as possible.
- The ghost can go either **left** or **right**, one step at a time.
- The ghost cannot see anything; he will know whether he has reached the castle or the bomb only when he gets to that location.
- The Q-learning agent should not have any hard-coded knowledge.
- The environment should only give feedback when the agent reaches the castle or the bomb.
- For each step, the environment allows the agent to choose to move either left or right.

## Run the Program

Compile
```
> cargo build --release
```

Run
```
> q_learning <map-length> <initial-index> <training-episode-count> <learning-rate> <greedy-factor>
```
