Simulates *millions* of games of uno in minutes and forces different algorithms to compete with each other.
Stacking +2's on +2's and +4's on +4's are allowed.
Saying "uno" when you have one card left is not required, because let's be honest that's a dumbass rule.

Image below shows the increase in likelihood of you winning compared to your base rate (`1 / player_count`) against a strategy of just choosing the first valid move where
- Always use +4 when the next player has 1 card left.
- Save +4 unless the player is close to winning
- Choose most common color in your hand for Wild and +4

![image](image.png)