# Dice Roller - Open Project Questions

Last updated: 2026-09-16

Use this as a living checklist for shaping the Discord app. Check off questions as
they are answered, and keep the answer directly beneath the question.

## Project Direction

- [x] 1. Is this bot meant only for your private Discord server, or should it eventually be usable by other groups/servers?

  Answer: It should be usable by other groups within the server. Broader use across
  many Discord servers is not a priority, but the bot should be robust enough to
  support it later if needed.

  Future feature: Make the bot more broadly accessible across multiple Discord
  servers.

- [x] 2. Is the primary audience players, GMs, or both?

  Answer: Both players and GMs.

- [x] 3. Should the bot feel like a plain utility, or should it have some Broken Empires flavor in its messages?

  Answer: The bot should be Broken Empires specific.

- [x] 4. What would make Milestone 1-3 good enough for actual table use?

  Answer: The bot needs to be online, accessible by the server, and able to
  respond to dice roll commands. Success means dice roll emulation works. Ideally,
  it supports special commands that change the final computed number, such as
  `4d6kh3` dropping the lowest die. Ideally, it also supports blackjack-style
  pass/fail results given a target number.

- [x] 5. Are there existing dice bots you like or dislike that should influence the UX?

  Answer: Dice Maiden and Avrae are the most familiar references. It is fine to
  draw from either. Dice Maiden has the blackjack-style modifier for tracking
  successes that this bot will need to follow.

## Discord UX

- [x] 6. Should all interactions be slash commands, or do you also want buttons, modals, context menus, or autocomplete?

  Answer: Scope interactions to slash commands for now.

  Future feature: Buttons, modals, context menus, and autocomplete can come at a
  later date.

- [x] 7. Should rolls be public by default?

  Answer: Rolls should be public by default.

- [x] 8. Do you need private/GM-only rolls?

  Answer: Yes.

- [x] 9. Should roll output be compact text, rich Discord embeds, or configurable?

  Answer: Compact text.

- [x] 10. Should the bot support labels like `/roll 2d6+3 reason:"climbing the wall"`?

  Answer: Yes.

- [x] 11. Should users be able to repeat their last roll?

  Answer: Not for the initial version.

  Future feature: Support a command like `/again` or `/reroll` to repeat the
  current user's last roll while generating a fresh result.

- [x] 12. Should the bot maintain per-server settings, such as default system, default visibility, or GM role?

  Answer: Yes.

## Generic Dice Engine

- [x] 13. What exact syntax do you want to support: `2d6+4`, `2d6 + 4`, `/roll dice:2d6 mod:4`, or all of those?

  Answer: Support all of those forms.

- [x] 14. Should `kh3` syntax be written like `4d6kh3`, `4d6 kh3`, or both?

  Answer: Use compact syntax like `4d6kh3`.

- [x] 15. Do you want common dice features beyond the current plan: drop lowest, exploding dice, rerolls, advantage/disadvantage, percentile aliases, count successes?

  Answer: Yes. Add common dice features.

- [x] 16. For math operators, should normal precedence apply, or should dice expressions be evaluated left-to-right?

  Answer: Use normal operator precedence.

- [x] 17. How should division round: floor, ceiling, nearest, or show decimals?

  Answer: Round to nearest.

- [x] 18. Should the bot always show individual die results, or only totals unless requested?

  Answer: Always show individual die results. Show totals if requested.

- [x] 19. Should dice results be auditable/reproducible in any way, or is normal random rolling enough?

  Answer: Normal random rolling is enough for gameplay, but the dice engine
  should be deterministic in automated tests.
- [x] 20. The plan lists `1d,6`; is that a typo for `1d6`?

  Answer: Yes. This is a typo for `1d6`.

## Broken Empires Rules

- [x] 21. What is the exact relationship between Success Levels and Degrees of Success? Are they different concepts or names for related outcomes?

  Answer: Success Levels apply when a roll succeeds by rolling the skill value or
  under. The number showing on the tens die is the number of Success Levels
  (SLs), representing the quality of the success. A `0` on the tens die counts as
  1 SL.

  Degree of Success applies during opposed rolls, when the table needs to know
  not only which side won, but by how much. The winner's SLs are reduced by the
  loser's SLs, if any. The result is the winner's final SLs, or Degree of
  Success.

- [x] 22. Does Expertise set a minimum Success Level only when the roll succeeds, or even on failure?

  Answer: Expertise sets a minimum Success Level only when the roll succeeds.

- [x] 23. What are the exact rules for critical success?

  Answer: Critical success is an extraordinary skill roll result. It is achieved
  when the roll is doubles under the skill value, or when the roll equals the
  skill value exactly.

- [x] 24. What are the exact rules for critical failure?

  Answer: Critical failure carries extra consequences. Doubles over the skill
  value is a critical failure. In combat, a critically failed defense gives a
  bonus SL to the opponent. Outside of combat, a critical failure may have a
  different meaning, or none at all.

- [x] 25. How does the system treat rolls like `00`, `100`, or percentile edge cases?

  Answer: A roll of `01` through `05` is always a success regardless of skill
  value. A roll of `99` through `100` is always a failure. If `99` or `100` is
  over the skill value, it is a critical failure.

- [x] 26. Are opposed rolls part of the core dice engine?

  Answer: Opposed rolls are part of the Broken Empires system, but they do not
  need to be part of the MVP.

  Opposed rolls happen when there is a resisting opposite force, such as a
  hobgoblin swinging a sword at you, a guard trying to spot you as you sneak
  through the darkness, a trap springing beneath you, poison attacking your body,
  or a Spellweaver casting a spell at you.

  Both sides roll their skills and whoever has the most SLs is the winner.
  Resolving an opposed roll follows these rules:

  - If only one side succeeds, the succeeding side wins.
  - If both sides succeed, the side with the most SLs wins.
  - If both sides succeed and SLs are tied, the higher die roll wins with 0 SLs.
  - If both sides succeed, SLs are tied, and die rolls are tied, the higher
    modified skill value wins.
  - If both sides succeed, SLs are tied, and one roll is a critical success, the
    critical success wins with 0 SLs.
  - If both rolls fail, neither side wins.
  - If both rolls fail and a winner is required, such as failed Stealth opposed
    by failed Perception, the higher modified skill value wins.
  - If both rolls fail and one is a critical failure, neither side wins.
  - If both rolls fail, one is a critical failure, and a winner is required, the
    normal failure beats the critical failure.

- [x] 27. Do difficulty modifiers affect the target number, the Success Levels, or something else?

  Answer: Difficulty modifies the target number.

- [x] 28. Should `/skill Persuade` output only success/failure, or also roll, target, Success Levels, Expertise, crit state, and narrative labels?

  Answer: Output the roll, Success Levels, Expertise, critical state,
  success/failure, and optional narrative labels.

- [x] 29. Are there other Broken Empires rolls that should be planned now, like damage, initiative, armor, hit location, stress, wounds, or downtime rolls?

  Answer: Yes, but not as part of the MVP. These mechanics need more review
  after reading their rules sections.

  🟡 Future feature: Return to damage, initiative, armor, hit location, stress,
  wounds, downtime rolls, and other Broken Empires mechanics after the MVP.

## Character Sheets

- [x] 30. Will Google Sheets be the only character sheet source at first?

  Answer: Yes, at first. Future work may include creating a custom character
  sheet or integrating with another character sheet system.

- [x] 31. Are sheets expected to follow one fixed template, or should the bot support configurable mappings?

  Answer: Character sheets are expected to follow one official Google Sheet
  template.

- [x] 32. Should `/import <character url>` copy sheet data into the bot, or read live from the sheet every time?

  Answer: Use an imported snapshot for rolls. Add `/refresh` to pull updated
  sheet data when needed.

- [x] 33. Do character sheets need to be public links, or should the bot support private Google Sheets with OAuth later?

  Answer: Character sheets should be public links.

- [x] 34. Can a Discord user have multiple characters?

  Answer: Yes, but this does not need to ship with the MVP.

- [ ] 35. Can one character be active in multiple servers or campaigns?

  Answer: Indifferent for now.

  🟡 Needs deeper discussion before a final decision.

- [x] 36. Should the GM be able to view, assign, approve, or manage imported characters?

  Answer: Yes. At minimum, the GM should be able to see the link to the
  character sheet for manual review.

  🟡 Future feature: Approval through `tbe-companion` is outside the MVP.

- [x] 37. Should skill names support aliases or fuzzy matching, like `persuade`, `Persuasion`, `talk`, etc.?

  Answer: Yes.

- [x] 38. Do you want Discord autocomplete for `/skill <skill-name>`?

  Answer: Nice to have.

## Hosting And Operations

- [x] 39. Is free hosting a hard requirement, or just preferred?

  Answer: Free hosting is heavily preferred, but not an absolute requirement.
  Roughly 95% preference for free hosting, with 5% willingness to pay if needed.

- [x] 40. Is occasional cold-start latency acceptable?

  Answer: Yes.

- [x] 41. Does the bot need to run 24/7, or only during sessions?

  Answer: The bot should run 24/7, though it does not need anywhere close to
  100% uptime.

- [x] 42. Are you comfortable self-hosting on a home machine/server?

  Answer: Yes. Self-hosting is worth considering because many applications are
  already self-hosted.

- [x] 43. Do you expect persistent storage, and if so, where should it live?

  Answer: Yes. Set up Postgres wherever the bot is hosted.

- [x] 44. What data should the bot store long-term: server settings, characters, roll history, imported sheet URLs?

  Answer: Store server settings, characters, and imported sheet URLs. Imported
  sheet URLs should be available in a way similar to Avrae's `!vsheet` alias,
  which prints out the character sheet link.

- [x] 45. Do you need backups or export/import for bot data?

  Answer: Export/import is outside the MVP, but desirable.

  Future feature: Add backup, export, and import support for bot data.

## Milestone Shape

- [x] 46. Should Milestone 3 build the dice parser as a standalone tested library before wiring it to Discord?

  Answer: Yes.

- [x] 47. Should Milestone 4 be implemented as a separate Broken Empires rules module layered on top of generic dice?

  Answer: Yes.

- [x] 48. What is the smallest playable vertical slice: `/roll`, `/check target`, or `/skill` from a sheet?

  Answer: `/roll`.

- [x] 49. Should character-sheet integration wait until the dice/rules engine is fully stable?

  Answer: Yes.

- [x] 50. What should the first pull request contain: bot skeleton, dice parser, Discord slash command, or hosting proof-of-life?

  Answer: The first PR should contain the bot skeleton. The second PR should add
  a Discord slash command that prints hello world. The third PR should add the
  dice parser.
