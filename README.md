# ◉ Linearite

Tiny Linear CLI

> Create issues without the 13k token MCP tax. Bonus: see who's shipping.

![rust](https://img.shields.io/badge/rust-%23CE422B?style=flat-square&logo=rust)

<br>

### Installation

```bash
curl -fsSL https://kade.work/linearite/install | bash
```

<br>

### Setup

Get your API key: [linear.app/settings/api](https://linear.app/settings/api)

```bash
export LINEAR_API_KEY="lin_api_..."
```

Add to `~/.zshrc` for persistence.

<br>

### Usage

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

```
╔════════════════╗
║ ◉  Linearite   ║
╚════════════════╝
 # ENG-1234
 ¶ Fix API bug
 ⌘ https://linear.app/acme/issue/ENG-1234
 ⎇ eng-1234-fix-api-bug
```

**Velocity Rankings**

```bash
linearite rank-teams
linearite rank-users
```

Ranks by completed issue points. Defaults to last 14 days, top 10.

```bash
linearite rank-teams --since 30d --top 5
linearite rank-users --since 2025-01-01 --top 20
```

Flags: `-s` since (duration like `7d` or date like `2025-01-15`), `-t` top N results

```
╔═════════════════════════════════════════════════════════════╗
║                         ◉ Linearite                         ║
╠══════╦═════════════════════╦═════════════╦══════════════════╣
║ Rank ║      User Name      ║ User Points ║ Issues Completed ║
╠══════╬═════════════════════╬═════════════╬══════════════════╣
║ 1    ║ Tony Stark          ║ 31          ║ 19               ║
╠══════╬═════════════════════╬═════════════╬══════════════════╣
║ 2    ║ Peter Parker        ║ 20          ║ 8                ║
╠══════╬═════════════════════╬═════════════╬══════════════════╣
║ 3    ║ Natasha Romanoff    ║ 19          ║ 7                ║
╚══════╩═════════════════════╩═════════════╩══════════════════╝
```

