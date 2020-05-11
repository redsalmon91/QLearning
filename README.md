
A simple program to demonstrate tabular **Q-learning**.

## Game Description

An elf is trapped in a forest, our goal is to help him to return to one of the castles.

🌲🌲🌲🏰🌲🌲🌲🌲🌲🌲  
🌲🌲🌲🌲🌲🌲🦁🌲🌲🌲  
🌲🌲🌲🌲🌲🌲🌲🌲🦁🌲  
🌲🌲🌲🧝🌲🌲🌲🌲🌲🌲  
🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲  
🌲🌲🌲🌲🌲🌲🌲🦁🌲🌲  
🌲🦁🌲🌲🌲🌲🌲🌲🌲🌲  
🌲🌲🌲🌲🌲🌲🌲🌲🌲🏰  
🌲🌲🌲🦁🌲🦁🌲🌲🌲🌲  
🌲🌲🌲🌲🌲🌲🌲🌲🌲🌲  

**Rules:**
- The goal for the elf(agent) is to reach a castle as soon as possible.
- The agent can go either left, right, up, or down; one step at a time.
- Game is over if the elf has reached at one of the castles or is caught by one of the lions.

## Run the Program

**Compile**
```
> cargo build --release
```

**Run**
```
> q_learning <map-file> <number-of-training-episodes> <learning-rate> <greedy-factor> <discount-factor> <max-step-allowed-in-final-simulation>

```

### Example Map File
Please checkout map file located in `examples/map.txt`
