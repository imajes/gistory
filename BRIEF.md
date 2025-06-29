# Project Name: gistory

# Language: Rust

# Goal: Create a TUI tool for exploring git commit history using natural language date filters

I want to build a command-line TUI application in Rust named `gistory`.

The purpose of the tool is to help developers explore the commit history of a Git repository, using **natural language time expressions** like:

- "this year"
- "jan 1 to mar 1"
- "a month to two weeks ago"
- "since last Friday"

---

## Features

1. 🧠 **Natural language date parsing**

   - Use the `event-parser` crate to convert time ranges like `"this year"` into concrete `NaiveDate` values for `since` and `until`.

2. 🧰 **Git integration**

   - Use `git2` to read commits between those dates.
   - For each commit, extract:
     - Short SHA (first 7 characters)
     - Author
     - Commit message
     - Date (MM/DD/YY)
     - Month/Year (e.g., `June 2025`)
     - A link to the commit on GitHub, if `origin` remote exists

3. 🖥️ **TUI interface**

   - Use `ratatui` (the actively maintained fork of tui-rs) to:
     - Render a table with the following columns:
       - [Link] SHA
       - Month + Year
       - Full date (MM/DD/YY)
       - Author
       - Commit message
     - Let the user navigate rows
     - On row select: show side panel or split view with:
       - Full commit message
       - Diff (git diff from that commit)

4. 🔍 **Filtering and sorting**

   - Allow filtering by:
     - Month (e.g., only January commits)
     - Author (e.g., commits by “Lynn Zhou”)
     - Message content
   - Allow sorting by any column (timestamp, author, etc.)

5. 📤 **CSV export**

   - Export current table (filtered/sorted) or full data to CSV
   - Use the `csv` crate

6. 💾 **Cache**
   - Cache parsed commit data in memory (not persisted yet)
   - All filtering/sorting/searching happens client-side after the initial scan

---

## Suggested Modules

- `main.rs`: startup and CLI wiring
- `cli.rs`: handles argument parsing using `clap`
- `gitlog.rs`: handles Git repository discovery and commit extraction
- `data.rs`: defines `CommitRow` struct and helpers
- `app.rs`: TUI application state and filters
- `tui.rs`: render logic using `ratatui::widgets::Table`

---

## Example CLI usage

```
gistory “this year”
gistory “jan 1 to feb 15”
```

---

## Example table output (in TUI)

| SHA     | Month     | Date     | Author        | Message                         |
| ------- | --------- | -------- | ------------- | ------------------------------- |
| a1b2c3d | June 2025 | 06/29/25 | James Cox     | Refactor control center alerts  |
| 9f8e7d6 | June 2025 | 06/25/25 | Lynn Zhou     | Add Turbo frame to alerts panel |
| c0ffee1 | May 2025  | 05/18/25 | Owen McKenzie | Fix HAML component rendering    |

---

## Crates to use

- `clap` for CLI args
- `event-parser` for date range parsing
- `git2` for commit reading
- `ratatui` for TUI rendering
- `csv` for export
- `chrono` for date handling

---

## Constraints

- Must run locally inside any existing Git repo (or accept path)
- Should gracefully handle cases where no GitHub remote is found
- Should work well in 80x24 terminal, but scale up if larger

---

## 🧭 Development Phases

### 🔹 Phase 1: MVP CLI

- ✅ Parse natural language date range
- ✅ Load matching commits from the local git repo
- ✅ Display to stdout as a nicely aligned table (no TUI yet)
- ✅ Infer GitHub URL from remote.origin.url if possible
- 💾 Cache results in-memory for later use
- 🔁 **Git commit** after this phase with message: `feat(cli): load and print filtered git log as table`

### 🔹 Phase 2: TUI foundation

- 🖥 Initialize `ratatui` app framework
- 🧱 Render commits as a scrollable table with selectable row
- 🪟 Add right-hand panel to preview full commit message + diff
- 🔁 **Git commit** after this phase with message: `feat(tui): add interactive table and commit preview panel`

### 🔹 Phase 3: Filters & Sorting

- 🔍 Add input handling to filter by author, date month, or substring
- ↕️ Enable sort by any column (toggle with header key or shortcuts)
- 🔁 **Git commit** after this phase with message: `feat(filters): enable filtering and sorting of table data`

### 🔹 Phase 4: CSV Export

- 📤 Add export option (hotkey or CLI arg) to save either:
  - full commit table
  - currently filtered/sorted view
- 🔁 **Git commit** after this phase with message: `feat(export): allow saving table view to CSV`

### 🔹 Phase 5: Polish

- 🎨 Use color styles and layout padding for clarity
- 🔄 Support live reload (`r` key) to rescan git repo
- 🧪 Add tests for commit parsing + range filtering
- 🔁 **Git commit** after this phase with message: `chore: polish UI and add tests`

---

## ✅ Version Control

Please commit after every major step or file milestone using conventional commits:

- `feat(...)`: for new features
- `fix(...)`: for bug fixes
- `chore(...)`: for structure, formatting, etc.
- `refactor(...)`: for internal reorganizations

---

Start by generating the Cargo project, parsing CLI input, and building the commit loader for Phase 1.

## Original Specification

It should render as a table, and it should allow filtering and sorting.

It should be able to do the following:

- take a natural language string to set the time boundary for the commits. (It should be able to parse anything from "this calendar year" to "a month to two weeks ago" etc.) - This crate looks promising: <https://github.com/isaacrlee/event-parser> ;

- gather all of the commits+data for that period, and cache those results in a local object;

- render a table showing the following fields to begin with:

  - url linked commit sha (use logic to grab the remote and build the url)
  - the month + year the commit happened
  - the MM/DD/YY timestamp
  - the author
  - the commit message

- it should be possible to browse these items: selecting each of them should show a right-split or overlay that displays the full git message and then the diff for that commit only.

- filters: it should be possible to filter for just commits in January, or by an author - essentially just restricting the table to whatever filter term is required.

- sort: it should be possible to sort on any of the columns.

- export - I'd like to be able to push either "all data" or "current view" to csv.
