# Dice Roller - Implementation Phases

Last updated: 2026-09-16

This plan turns the initial milestones and answered project questions into an
implementation sequence. The preferred working style is test-driven: write or
update failing tests first, verify the failure, then implement the smallest code
needed to pass.

## Implementation Plan

Build the bot in thin, reviewable PRs. Start with a Discord bot skeleton, then
prove Discord command registration with a small hello-world command, then build
the dice parser as a standalone tested library before connecting it to `/roll`.

The MVP should focus on a reliable Broken Empires-flavored Discord app that can
roll dice publicly by default, produce compact text output, and support a useful
`/roll` command. Character sheets, Postgres persistence, and the Broken Empires
rules engine should layer on after the generic dice engine is stable.

Persistent storage should use Postgres wherever the bot is hosted. Free hosting
is strongly preferred, but a small paid option is acceptable if needed. The bot
should run 24/7, without requiring near-perfect uptime.

## Phase 0 - Project Baseline

Goal: Establish a clean baseline for development and review.

- [ ] Confirm the repository has an initial `main` commit so feature branches can
  branch from `main`.
- [ ] Create a feature branch for the bot skeleton work.
- [ ] Keep `.env.example`, setup docs, and local run instructions current.
- [ ] Add or verify basic CI for `cargo fmt`, `cargo clippy`, and `cargo test`.
- [ ] Document required Discord environment variables.

Notes:

- The current source already contains a Serenity-based scaffold with `/ping` and
  `/about`, so this phase may mostly be an audit/cleanup pass before formal PRs.

## Phase 1 - Bot Skeleton PR

Goal: Create the smallest Discord bot application that can start cleanly.

- [ ] Load `DISCORD_TOKEN` from environment.
- [ ] Optionally load `DISCORD_GUILD_ID` for fast guild-scoped command
  registration during development.
- [ ] Start a Serenity client with the minimum required gateway intents.
- [ ] Add logging/tracing for bot startup and Discord connection lifecycle.
- [ ] Add tests for configuration loading and validation.
- [ ] Update README setup and run instructions.

Acceptance criteria:

- [ ] `cargo test` passes.
- [ ] The bot process starts with valid environment variables.
- [ ] Missing or invalid configuration produces clear errors.

## Phase 2 - Hello World Slash Command PR

Goal: Prove slash command registration and interaction responses.

- [ ] Register a simple guild-scoped slash command during development.
- [ ] Add a command such as `/hello`, `/ping`, or `/about`.
- [ ] Respond with compact plain text.
- [ ] Add unit tests for command response routing.
- [ ] Verify the bot is invitable, visible on the Discord server, and online.

Acceptance criteria:

- [ ] The bot can be invited to the Discord server.
- [ ] The bot appears online.
- [ ] A slash command responds successfully in Discord.

## Phase 3 - Standalone Generic Dice Library PR

Goal: Build the dice engine as a tested Rust module/library before wiring it to
Discord.

- [ ] Define a dice expression parser for forms like `2d6+4`, `2d6 + 4`, and
  slash-command-friendly split inputs like `dice:2d6 mod:4`.
- [ ] Support compact keep-highest syntax such as `4d6kh3`.
- [ ] Support standard dice types including `d2`, `d4`, `d6`, `d8`, `d10`,
  `d12`, `d20`, and `d100`.
- [ ] Apply normal operator precedence.
- [ ] Round division results to nearest.
- [ ] Always retain individual die results in the evaluation result.
- [ ] Support deterministic RNG injection for automated tests.
- [ ] Use normal random rolling for gameplay.
- [ ] Add tests for parsing, evaluation, operator precedence, rounding, and kept
  or dropped dice.

Common dice features to add here or in follow-up PRs:

- [ ] Drop lowest / drop highest.
- [ ] Exploding dice.
- [ ] Rerolls.
- [ ] Advantage / disadvantage aliases.
- [ ] Percentile aliases.
- [ ] Count-successes style results.

Acceptance criteria:

- [ ] Dice parser tests fail before implementation and pass afterward.
- [ ] The engine returns enough structured detail to print individual dice,
  kept/dropped dice, modifiers, and totals.
- [ ] The engine can be used without Discord.

## Phase 4 - `/roll` Discord Command PR

Goal: Make `/roll` the first playable vertical slice.

- [ ] Add a `/roll` slash command.
- [ ] Accept dice expressions in all supported syntax forms.
- [ ] Support optional roll labels/reasons.
- [ ] Return compact text output.
- [ ] Show individual die results by default.
- [ ] Include totals when requested.
- [ ] Make rolls public by default.
- [ ] Support private or GM-only roll visibility.
- [ ] Support blackjack-style pass/fail against a target number when provided.
- [ ] Add command-level tests around formatting and request handling.

Acceptance criteria:

- [ ] A user can roll common expressions like `2d6`, `2d6+4`, and `4d6kh3`.
- [ ] Output is compact and readable in Discord.
- [ ] Public, private, and GM-only visibility behavior is defined and tested.

## Phase 5 - Broken Empires Rules Module

Goal: Layer Broken Empires skill-check logic on top of the generic dice engine.

- [ ] Create a separate Broken Empires rules module rather than mixing rules into
  the generic dice parser.
- [ ] Support skill checks where rolling the modified skill value or under
  succeeds.
- [ ] Compute Success Levels from the tens die.
- [ ] Treat a `0` tens die as 1 SL.
- [ ] Apply Expertise as a minimum Success Level only on successful rolls.
- [ ] Apply difficulty modifiers to the target number.
- [ ] Support critical success on doubles under the skill value or an exact skill
  value roll.
- [ ] Support critical failure on doubles over the skill value.
- [ ] Treat `01` through `05` as automatic success.
- [ ] Treat `99` through `100` as automatic failure.
- [ ] Output roll, target, success/failure, SLs, Expertise, critical state, and
  optional narrative labels.
- [ ] Add thorough tests for edge cases, especially `01`, `05`, `99`, `100`,
  doubles, exact target rolls, Expertise, and difficulty.

Acceptance criteria:

- [ ] Broken Empires skill checks are deterministic and heavily covered by tests.
- [ ] The rules module can be tested without Discord.
- [ ] `/roll` or a later command can call the rules module without duplicating
  rules logic.

## Phase 6 - Persistence And Server Settings

Goal: Add durable storage and server-level configuration.

- [ ] Choose the hosting target and provision Postgres there.
- [ ] Add database connection configuration.
- [ ] Add migrations.
- [ ] Store server settings.
- [ ] Store GM role or GM visibility settings.
- [ ] Store characters and imported sheet URLs.
- [ ] Avoid long-term roll history for MVP unless a later requirement changes
  that decision.
- [ ] Add tests around repositories/storage adapters where practical.

Acceptance criteria:

- [ ] The bot can read and write required MVP data in Postgres.
- [ ] Server settings survive bot restarts.
- [ ] Imported sheet URLs can be retrieved later.

## Phase 7 - Character Sheet Import

Goal: Import Broken Empires characters from one official public Google Sheet
template.

- [ ] Define the official Google Sheet template contract.
- [ ] Add `/import <character url>` for public Google Sheet links.
- [ ] Copy sheet data into the bot as an imported snapshot.
- [ ] Add `/refresh` to update the stored snapshot from the sheet.
- [ ] Store the original sheet URL.
- [ ] Add a command or alias similar to Avrae's `!vsheet` that prints the
  character sheet link.
- [ ] Support one active character per user for MVP.
- [ ] Allow a user to have multiple characters after MVP.
- [ ] Let GMs view character sheet links for manual review.
- [ ] Keep GM approval through `tbe-companion` outside the MVP.

Acceptance criteria:

- [ ] A public official-template Google Sheet can be imported.
- [ ] Rolls use the stored snapshot, not live sheet reads.
- [ ] `/refresh` updates the stored snapshot.
- [ ] A GM can find the imported sheet link for manual review.

## Phase 8 - `/skill` Character Rolls

Goal: Let players roll skills from imported character snapshots.

- [ ] Add `/skill <skill-name>`.
- [ ] Resolve skills from the active character.
- [ ] Support skill aliases or fuzzy matching.
- [ ] Use the Broken Empires rules module for success/failure, SLs, Expertise,
  and critical state.
- [ ] Include optional narrative labels.
- [ ] Keep Discord autocomplete as nice-to-have, not required for MVP.
- [ ] Add tests for skill lookup, aliases/fuzzy matching, formatting, and rules
  integration.

Acceptance criteria:

- [ ] A player can import a character and roll a skill by name.
- [ ] Output includes roll, Success Levels, Expertise, critical state,
  success/failure, and optional labels.
- [ ] Character sheet integration waits until dice and rules modules are stable.

## Phase 9 - Hosting And Operations

Goal: Deploy the bot in a low-cost, maintainable way.

- [ ] Compare free hosting, low-cost paid hosting, and self-hosting.
- [ ] Prefer free hosting where practical.
- [ ] Consider self-hosting because the operator already self-hosts many apps.
- [ ] Deploy the bot and Postgres to the selected environment.
- [ ] Configure environment variables and secrets.
- [ ] Add basic logs and restart behavior.
- [ ] Document deployment and rollback steps.
- [ ] Confirm the bot can run 24/7 with acceptable uptime.

Acceptance criteria:

- [ ] Bot is online from the chosen hosting environment.
- [ ] Postgres is available from that environment.
- [ ] Restarting the bot does not lose persistent settings or imported
  characters.

## Post-MVP Backlog

These are intentionally outside the MVP unless reprioritized.

- [ ] Decide whether one character can be active in multiple servers or campaigns.
- [ ] Support multiple characters per Discord user.
- [ ] Add opposed rolls for the Broken Empires system.
- [ ] Add damage, initiative, armor, hit location, stress, wounds, downtime rolls,
  and other Broken Empires mechanics after their rules are reviewed.
- [ ] Add buttons, modals, context menus, and autocomplete.
- [ ] Add `/again` or `/reroll` to repeat the current user's last roll with a
  fresh result.
- [ ] Add broader multi-server accessibility.
- [ ] Add custom character sheet support or integrate with another sheet system.
- [ ] Add GM approval workflows in `tbe-companion`.
- [ ] Add backup, export, and import support for bot data.
- [ ] Consider private Google Sheets with OAuth.
