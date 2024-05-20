
# 2D Bullet Hell Game Jam 

## Goal
Produce a 2d Bullet Hell game using the theme *Consequences* by 05/19/24 

## Details

I want to build a game using `Rust`. I have never coded in rust and have always been interested, and I have never participated in a game jam before.

I figure it could be a fun way to just get something done!


## Consequences
Consequences come in many shapes and sizes. Not all consequences are bad. 

- If an enemy reaches the bottom of the screen you will consequentially see two more enemies spawn at the top of the screen (as well as the one who made it to the bottom going back to try again).
- If you fail to clear a wave before the next wave spawns in. Consequentially you will not recieve a power up after that wave
- Power ups spawn in groups of three, only one can be selected impacting either your bullets, shapes, or health.
- Time errodes all things, and consequentially will make the game much much harder as it goes on (waves will get closer together, and more units will spawn per wave)


## Game Play Tips / Mechanics

### Controls
- `W` : Up
- `S` : Down
- `A` : Left
- `D` : Right
- `E` : Special (Shape)
- `Space` : Shoot
- `Shift` : Speed up
- `UpArrow` : Volume Up,
- `DownArrow`: Volume Down

### Game Play
There is no objective other than to survive. An Alien force has invaded your home planet in overwhelming numbers. You as the last literate and capable fighter pilot must hold off for as long as possible to allow everyone else to escape (you are too cool to run away). Try and survive as long as possible, killing as many of those alien bastards as you can.

### Bullets
Bullets are the basic offense of the player. Power ups can alter the player's gun allowing for more rounds to be shot faster from the player. The gun can be reloaded by pressing R for a slightly reduced reload time. The gun will automatically reload when empty

### Shapes
Shapes are the special weapon for the player. Shapes can also be altered via power ups. Shapes spawn bullets depending on the size (which can be increased via power up). Shapes are kinda broken.

### Power Ups
Power Ups are spawned in when no enemies are on the screen. Initially a single group of three will drop after a wave (and before the next) and then every 8 seconds another will spawn until an enemy returns to the screen. Try to get solid power ups in the earlier waves while there are less enemies and more time between waves.

#### Bullet Power Ups 
![image](https://github.com/Jimdangle/BulletHellGameJam/assets/72684566/72ddf3f2-51b8-44f5-9431-0f0f78b34406)
(in order of left to right)
1. Increased bullet ammo
2. Faster shoot speed
3. Random extra bullet
4. Increased Damage

#### Shape Power Ups
![image](https://github.com/Jimdangle/BulletHellGameJam/assets/72684566/feb8ab5c-a1cf-4521-bbd3-63f1aef3b108)
1. More Shape ammo
2. Reduced shape reload
3. Additional Random shape
4. increased size

#### Health Power Ups
![image](https://github.com/Jimdangle/BulletHellGameJam/assets/72684566/5fe1c849-b614-490a-adfd-1e0f58e40145)
1. Increase Total Shield
2. Add health
3. Regen more shield per shield tick




## Art Credits

### Music 
All songs played are from this video https://www.youtube.com/watch?v=0u1jq-6-ELk

### Sprites
Ya boi (aka Me aka Jim Dangle)

### Background image
tbh i dont fully remember some license free website

## If images on readme dont load try using the github
https://github.com/Jimdangle/BulletHellGameJam

