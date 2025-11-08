# linearite
🦀 Tiny Linear CLI designed for AI agents

> Goal: Create issues without the 13k token tax of full MCP integration

## Installation

### Download Pre-built Binary

Download the latest release from [GitHub Releases](https://github.com/kxzk/linearite/releases):

```bash
# Download for your Mac (Apple Silicon or Intel)
# Then make it executable and move to PATH
chmod +x linearite-*
sudo mv linearite-* /usr/local/bin/linearite
```

### From Source

```bash
cargo install --git https://github.com/kxzk/linearite.git
```

## Setup

1. Get your Linear API key from [Linear Settings > API](https://linear.app/settings/api)
2. Set the environment variable:
   ```bash
   export LINEAR_API_KEY="your-api-key-here"
   # Or add to ~/.zshrc
   echo 'export LINEAR_API_KEY="your-api-key-here"' >> ~/.zshrc
   ```

## Usage

### List Teams

```bash
linearite list-teams
```

### List Projects

```bash
linearite list-projects
```

### Create an Issue

```bash
# Basic issue
linearite create "Fix bug in API" --team-id team-abc123

# With description
linearite create "Fix bug in API" \
  --team-id team-abc123 \
  --description "The API is broken"

# With description and project
linearite create "Add feature" \
  --team-id team-abc123 \
  --description "Implement feature X" \
  --project-id proj-xyz789

# Using short flags
linearite create "Test issue" -t team-abc123 -d "Description" -p proj-xyz789
```
