# Ten Top Tiled - Design Document

Ten Top Tiled is a tile-matching game about running a restaurant, except the restaurant is a hot mess and you have to move tiles around to make sure the customers get their food on time. 

# Objectives

Customers come into the restaurant craving a food, it's your job as the cook to keep the food coming and keep the customers happy.

# Mechanics

- Each game round has a set number of customers
- The playfield is a grid with n number of tiles.  
- Tiles represent ingredients.  
- The playfield is populated with a random mix of tiles
- The player is able to swap two tiles with each other at will
- When tiles are adjacent to other tiles that make up a dish, those tiles are removed from the playfield and the dish is ready for delivery  
- When tiles are removed from the playfield, new tiles appear at the top of the playfield and fall into place 
- New customers appear on screen periodically
- Each customer wants a specific kind of food (hot dog, hamburger, burrito, etc)
- If customers get their food quickly, they are happy and the score goes up
- If customers have to wait too long, they are unhappy and complain
- Complaints lower morale
- If morale is too low, the game ends 
