# TBE Companion

A small Rust Discord app built with Serenity.

## Commands

- `/ping` replies with `Pong!`
- `/about` replies with a short app status message

## Setup

1. Create a Discord application and bot in the Discord Developer Portal.
2. Enable the bot token and invite the bot to a server with the `applications.commands` and `bot` scopes.
3. Set environment variables:

```sh
export DISCORD_TOKEN="your-bot-token"
export DISCORD_GUILD_ID="your-test-server-id"
```

`DISCORD_GUILD_ID` is optional. Keeping it set during development makes slash command updates appear immediately in that guild. If it is omitted, the app registers global commands.

## Run

```sh
cargo run
```

## Test

```sh
cargo test
```
