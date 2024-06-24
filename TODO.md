# TODO Coragames

> A simple list of tasks

# Preprod (learning)
---

- [x] Rust basics
	- [x] Rust learning
	- [x] Rust practices (exercism)
	- [x] Learn Rust best practices (e.g., naming convention)
	- [x] Learn Rust common architectures (e.g., EmbarkStudio gpu project)
- [x] Check Rust Projects
	- [x] Bevy Engine
	- [x] Embark Studio
- [x] Find libraries and tools
	- [x] Networking library
	- [x] Agent System library
	- [x] Tilemap
	- [x] GPU / Graphics / UI library
	- [x] Logger
	- [x] Profiling
- [ ] State of the Art
	- [ ] Learn about Redis DB (to see if I should use it)
	- [ ] Read tilemap articles
	- [ ] Read redblobgames articles

# Preprod v1 (offline prototype)
---

- [ ] Tilemap
	- [ ] Tilemap grid (2dim array of cells)
	- [ ] Tilemap cell (move, cover, player)
	- [ ] Create subview of a tilemap (the player's view)
- [ ] Player
	- [x] Name
	- [x] Health
	- [x] Tile position
- [ ] Actions
	- [ ] Move
	- [ ] Attack
	- [ ] Interact
- [ ] Game
	- [ ] Game map (tilemap)
	- [ ] List Players info
	- [ ] Add player
	- [ ] Remove player
	- [ ] Create player vision
	- [ ] Apply Bulk Actions (applies a list of actions)
	- [ ] Save game
	- [ ] Load game
- [ ] Game Server
	- [ ] Start / Stop  / Pause
	- [ ] Dummy Game Server (no online)
- [ ] Game Ai Agent
	- [ ] Start / Stop  / Pause
	- [ ] Dummy AI Agent (random action)

# Preprod v2 (online prototype)
---

- [ ] Game Server
	- [ ] Connect an AI Agent
	- [ ] Disconnect an AI Agent
- [ ] Game AI Agent
	- [ ] Connect with server
	- [ ] Disconnect from server
	- [ ] Upload brain code
- [ ] Game map
	- [ ] Save tilemap (to json?)
	- [ ] Load tilemap (from json?)
- [ ] Player
	- [ ] Health
	- [ ] Tile position
	- [ ] Last action
- [ ] Actions
	- [ ] Ranged attack
	- [ ] Heal
	- [ ] Search
	- [ ] Pickup object
