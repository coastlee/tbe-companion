# TBE Companion

A small Rust Discord app built with Serenity.

## Commands

- `/ping` replies with `Pong!`
- `/about` replies with a short app status message

## Setup

1. Create a Discord application and bot in the Discord Developer Portal.
2. Enable the bot token and invite the bot to a server with the `applications.commands` and `bot` scopes.
3. Copy `.env.example` if you want a local reference file:

```sh
cp .env.example .env
```

4. Export the required environment variables before running the bot:

```sh
export DISCORD_TOKEN="your-bot-token"
export DISCORD_GUILD_ID="your-test-server-id"
```

`DISCORD_TOKEN` is required. `DISCORD_GUILD_ID` is optional. Keeping it set
during development makes slash command updates appear immediately in that guild.
If it is omitted or left blank, the app registers global commands.

## Run

```sh
cargo run
```

## Test

```sh
cargo test
```

## Local Checks

Run the same Rust checks used by CI before opening a pull request:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```
