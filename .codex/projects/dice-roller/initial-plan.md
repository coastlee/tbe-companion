# Dice Roller - Initial Plan

These are my initial thoughts about what I want to accomplish for the dice-rolling capability of this new discord bot, broken up by milestones.

## Milestone 1 - New Bot is Invitable and Visible on the Discord Server

Simple. Bot is created and can be invited to my discord server.

## Milestone 2 - Bot is online

- [ ] Explore the various ways to host a discord bot
- [ ] Identify free hosting solutions, explore the pros / cons / limitations of such
- [ ] Consider self-hosting, explore the pros / cons / limitations of such
- [ ] Decide how to host the bot
- [ ] Host the bot
- [ ] Bot shows as online

## Milestone 3 - Basic Dice Rolling Capabilities

- [ ] Bot can respond `/roll` slash commands
- [ ] Build a series of unit tests to validate a variety of different dice rolling commands such as:
    - 2d6 rolls two six-sided die
    - adding mathematical operators like +4, -2, *4, /3 modifies the result of the roll accordingly
    - kh3 would "keep the highest 3 die results" for something like 4d6
    - supports all dice types: 1d2, 1d4, 1d,6, 1d8, 1d10, 1d12, 1d20, 1d100

## Milestone 4 - The Broken Empires RPG Dice Engine Supported

- [ ] Supports the concept of Success Levels
  - This means that if I roll a **25** on a skill level of **45**, I take the *tens number* as the **Success Level**. 25 = 2 success levels.
  - [ ] Supports **Expertise** if provided; this means the character has a higher minimum **Success Level**; if they have an expertise of 4 and roll a 25 against a skill of 45, they gain **4 Success Levels** instead of the normal *two*.
- [ ] Supports the concept of Degrees of Success
- [ ] Supports blackjack styles of success
  - This means that if I roll a **Persuade** skill check and have a skill of **45**, I am only successful if I roll a **45 or less**
- [ ] Supports critical successes
- [ ] Supports cirtical failures

## Milestone 5 - Character Sheet Integration

- [ ] Can connect to a character sheet and execute dice rolls from the character sheet based on the data captured by the sheet
  - [ ] /import <character url>
    - [ ] /import supports a google-sheet character sheet
  - [ ] /skill <skill-name> invokes the active character's skill check, tracking success, success levels, critical failure/success
