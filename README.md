# linearite

Minimal Linear CLI for AI agents.

Create issues without the 13k token overhead of full MCP integration.

---

## Install

**Binary**

```bash
curl -L https://github.com/kxzk/linearite/releases/latest/download/linearite-darwin -o linearite
chmod +x linearite
mv linearite /usr/local/bin/
```

**Source**

```bash
cargo install --git https://github.com/kxzk/linearite.git
```

---

## Setup

Get your API key: [linear.app/settings/api](https://linear.app/settings/api)

```bash
export LINEAR_API_KEY="lin_api_..."
```

Add to `~/.zshrc` for persistence.

---

## Usage

**Discovery**

```bash
linearite list-teams
linearite list-projects
```

**Create Issues**

```bash
linearite create "Fix API bug" --team-id team-abc123

linearite create "Add feature X" \
  --team-id team-abc123 \
  --description "Detailed context" \
  --project-id proj-xyz789
```

Flags: `-t` team, `-d` description, `-p` project

---

## License

MIT
