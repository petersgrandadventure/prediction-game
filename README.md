# Welcome to the Prediction Game!

This game was built during a dev camp to explore creating games on the holochain platform.

The idea behind my game is to focus on the human connection and the ability to suggest and predict various things (true/false, random number, etc). 

Player 1 initially takes the role of Suggester, they will record their suggestion as follows

```
make_move {"Suggest":{"suggestion":1}}
```

They will then focus on sending this suggestion to the Predicter, who will log their prediction as follows

```
make_move {"Predict":{"prediction":1}}
```

A record will be kept of the number of successful/attempted predictions and suggestions so players can track their abilities over time. 

The current player may also choose to swap roles using the Swap move as follows

```
make_move {"Swap":{}}
```