# rust-blackjack
A CLI blackjack game built in Rust


## RULES:

This game can support up to 5 players in addition to the dealer CPU. 

At the beginning of a round, the dealer deals out two cards to each player, including itself. Before beginning the turn order, the game evaluates all hands for a natural 21, ending the round automatically if any are found.

On each players' turn they may choose to hit or stay. If a hand goes over 21, it busts. If a hand reaches 21 exactly, the turn ends. If a hand is initially 21, the game automatically moves on. 

On the dealer's turn they will automatically hit or stand based on the value of their hand. 
- less than or equal to 16 is a hit.
- greater than 16 is a stay.
- turn ends at 21.
- turn busts over 21.