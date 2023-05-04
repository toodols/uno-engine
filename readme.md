# Uno Engine
Simulates *millions* of games of uno in minutes and forces different algorithms to compete with each other.
- Stacking +2's on +2's and +4's on +4's are allowed.
- Saying "uno" when you have one card left is not required, because let's be honest that's a dumbass rule.
- If all players have collectively hoarded all 108 cards, the game ends with no victor.

An **advantage** is the increase in likelihood of you winning compared to your base rate (`1 / player_count`). For example, If you have a 50% win rate in a 4-player match, you have a +100.0% advantage, when compared to a base win rate, 25%.

Results show that uno is pretty much entirely luck based. In 2-3 player matches, the advantage is heavy skewed by +4% for player that starts first. Strategies that involve playing "smartly" can at most net +6% advantage.