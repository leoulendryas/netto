# netto

> Find out how much of your codebase you actually wrote.

`netto` is a fast Rust CLI that counts your lines of code — and separates what you built from what your framework scaffolded for you. It reads your git history, detects your stack, skips generated files, and gives you an honest score.

```
NETTO
~/projects/myapp  ·  scanned in 0.42s

[ React 18 ]  [ Node.js ]  [ TypeScript ]

─────────────────────────────────────────────────────────
CORE COUNTS

    12,847        15,203             14
  source lines   all lines   files skipped

─────────────────────────────────────────────────────────
LANGUAGE BREAKDOWN

  TypeScript    ████████████████████░░░░░░░░  62.3%
  CSS           ███████░░░░░░░░░░░░░░░░░░░░░  18.1%
  JSON / YAML   ████░░░░░░░░░░░░░░░░░░░░░░░░  11.4%
  Markdown      ██░░░░░░░░░░░░░░░░░░░░░░░░░░   5.4%
  Shell         █░░░░░░░░░░░░░░░░░░░░░░░░░░░   2.8%

─────────────────────────────────────────────────────────
I WROTE THIS

  34.2%
  original authorship

  your lines              4,393
  framework baseline      8,454
  verdict                 humbling — lots of boilerplate out there

─────────────────────────────────────────────────────────
GIT INSIGHTS

      17 days        284
  commit streak   total commits

  you added       +6,821 lines across 284 commits
  authorship      alex <alex@company.com>

  PROBLEM CHILDREN

  src/components/Dashboard.tsx     ████████████  47×
  src/api/auth.ts                  ████████░░░░  34×
  src/store/index.ts               █████░░░░░░░  21×

─────────────────────────────────────────────────────────
  ◆ 10K — true craftsman territory

  netto --help for options  ·  netto diff for weekly delta
```

---

## Install

**Via cargo:**
```bash
cargo install netto
```

**Via Homebrew (macOS):**
```bash
brew tap leoulendryas/netto
brew install netto
```

**Download a binary** from the [releases page](https://github.com/leoulendryas/netto/releases).

---

## Usage

```bash
# Scan the current directory
netto

# Scan a specific project
netto ~/projects/myapp

# Subtract a framework baseline (only count what you added)
netto --baseline ~/fresh-react-app

# Filter git stats to a specific author
netto --author "Your Name"

# Skip git analysis
netto --no-git

# Output as JSON
netto --json
```

---

## How it works

**Line counting** — netto reads every file in your project, skips blank lines and comments, and separates code from config and markup. Auto-generated files (detected by header comments like `// auto-generated`) are excluded entirely.

**Framework baseline** — point `--baseline` at a fresh scaffold of your framework and netto subtracts those lines from your count. What's left is the code you actually wrote.

**"I wrote this" score** — your git-attributed additions divided by total source lines. A humble number is normal. Most projects are 20–40% original.

**Commit streak** — consecutive days you've made at least one commit. Counts backward from today.

**Problem children** — files changed most frequently in git history. Usually where the complexity lives.

---

## Supported languages

| Language | Extensions |
|---|---|
| Rust | `.rs` |
| TypeScript | `.ts`, `.tsx` |
| JavaScript | `.js`, `.jsx`, `.mjs` |
| Python | `.py`, `.pyw` |
| Go | `.go` |
| Java | `.java` |
| CSS / SCSS | `.css`, `.scss`, `.sass` |
| HTML | `.html`, `.htm` |
| Shell | `.sh`, `.bash`, `.zsh` |
| JSON / YAML | `.json`, `.yml`, `.yaml` |
| Markdown | `.md`, `.mdx` |

---

## Options

| Flag | Description |
|---|---|
| `--baseline <path>` | Path to a fresh scaffold to subtract |
| `--author <name>` | Filter git stats to this author |
| `--no-git` | Skip git analysis entirely |
| `--json` | Output results as JSON |
| `--version` | Print version |
| `--help` | Print help |

---

## Contributing

PRs are welcome. If you find a language missing, a file type being miscounted, or a framework not being detected — open an issue or send a fix.

```bash
git clone https://github.com/leoulendryas/netto
cd netto
cargo build
cargo test
```

---

## License

MIT — see [LICENSE](LICENSE)
