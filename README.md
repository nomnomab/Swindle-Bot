# Swindle Bot
*A simple bot made with Serenity*

This bot has a few main purposes:
1) Be flexible and efficient
2) Handle various features
3) Be server-independent

Currently there is little to no actual error handling except for some `.expect("Glorious error message")` shenanigans.

Here is the current to-do list:
- [x] Load up config files from `./config/`
- [x] Store config files and data into a map
- [x] Connect with Serenity through a bot token
- [x] Backend database for user information
    - [x] XP
    - [x] Level
    - [x] Karma
    - [x] Coins
- [ ] !profile
    - [x] Load up information from the database
    - [x] Show xp, level, karma, and coins
    - [x] Return an embed
    - [ ] Return a custom image with added text and information
- [ ] !about
    - [x] Loads data from config map that hooks to `./config/about.cfg`
    - [x] Return an embed
    - [x] Link to GitHub repository
    - [ ] Link to personal website