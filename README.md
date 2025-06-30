# gistory

`gistory` is an experimental command line and TUI application for exploring a Git repository's history. It parses natural language date ranges and renders matching commits in an interactive table.

## Building

```bash
cargo build
```

## Running

Run from within a Git repository or pass a path via `--repo`:

```bash
cargo run -- "this month"
# or
cargo run -- "jan 1 to feb 15" --repo /path/to/repo
```

Use the arrow keys to move through the list. Press `q` to quit.

## Features

- Natural language date parsing using **chrono-english**
- Loads commits between the parsed `since` and `until` range
- Interactive TUI table with commit message preview
- Automatically detects a GitHub remote and builds commit links

## Roadmap

Future work will add filtering by column, sorting, and CSV export as outlined in `BRIEF.md`.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
