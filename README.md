# ◉ Linearite

![rust](https://img.shields.io/badge/rust-%23CE422B?style=flat-square&logo=rust)

> A _tiny_ Linear CLI

<br>

* Create issues without the 13k token MCP Tax
* Bonus: see who's shipping

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

_+ Discovery_

```bash
linearite list-teams
linearite list-projects
```

_+ Create Issues_

```bash
# Flags
# -t --team-id (team identifier)
# -d --description (description of the issue)
# -p --project (project identifier)
linearite create "Fix API bug" --team-id team-abc123

linearite create "Add feature X" \
  --team-id team-abc123 \
  --description "Detailed context" \
  --project-id proj-xyz789
```

```
╔════════════════╗
║ ◉  Linearite   ║
╚════════════════╝
 # ENG-1234
 ¶ Fix API bug
 ⌘ https://linear.app/acme/issue/ENG-1234
 ⎇ eng-1234-fix-api-bug
```

_+ Velocity Rankings_

```bash
# Ranks by completed issue points. Defaults to last 14 days, top 10.
linearite rank-teams
linearite rank-users
```

```bash
# Flags
# -s --since (duration like `7d` or date like `2025-01-15`)
# -t --top (top N results)
linearite rank-teams --since 30d --top 5
linearite rank-users --since 2025-01-01 --top 20
```

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

