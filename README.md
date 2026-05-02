# 🚀 Netto

**Find out how much of your codebase you actually wrote.**

Netto is a high-performance, parallelized codebase analytics tool built in Rust. It doesn't just count lines; it tells the story of your project—stripping away the boilerplate, acknowledging your frameworks, and celebrating your coding streaks.

---

## ✨ Features

- **Parallel Processing:** Uses `rayon` to scan thousands of files across all CPU cores in seconds.
- **"I Wrote This" Score:** Intelligent authorship detection that compares your hand-written code against framework baselines and generated files.
- **Git Integration:** Deep mining of git history to track streaks, total commits, and your "Problem Children" (most-changed files).
- **Beautiful CLI:** Clean, modern terminal output with ASCII charts, color-coded bars, and milestone badges.
- **Framework Aware:** Automatically detects and categorizes projects (Rust, Node.js, Python, Go, etc.).
- **Smart Filtering:** respects `.gitignore`, skips binary files, and auto-detects generated headers like `// auto-generated`.

---

## 🌍 Supported Languages

Netto supports over **50+ languages** and configuration formats out of the box:

- **Web:** TypeScript, JavaScript, HTML, CSS, SCSS, Svelte, Vue, Astro, PHP, Ruby.
- **Systems & Backend:** Rust, Go, C, C++, C#, Java, Kotlin, Zig, Swift, Objective-C.
- **Functional:** Elixir, Haskell, Erlang, Clojure, F#, OCaml.
- **Data & Scripting:** Python, R, Julia, Jupyter Notebooks, Shell, PowerShell, Perl.
- **Infrastructure:** SQL, Dockerfile, GraphQL, Terraform (HCL).
- **Config & Docs:** JSON, YAML, TOML, XML, Markdown, Rst, Latex.

---

## 📦 Installation

### From Crates.io (Recommended)
```bash
cargo install netto
```

### From Source
```bash
git clone https://github.com/YOUR_GITHUB_USERNAME/netto.git
cd netto
cargo install --path .
```

---

## 🛠️ Usage

### Basic Scan
Analyze the current directory:
```bash
netto
```

### Baseline Subtraction
See how much *original* logic you added to a scaffold (like `create-react-app`):
```bash
netto --baseline path/to/fresh/scaffold
```

### Git Analytics
Filter stats by a specific author:
```bash
netto --author "Your Name"
```

### Options
```text
Usage: netto [OPTIONS] [PATH]

Arguments:
  [PATH]  Path to the project root [default: .]

Options:
      --baseline <BASELINE>  Point to a fresh scaffold to subtract it from the count
  -w, --web                  Open the full report in your browser after scanning
  -j, --json                 Output raw JSON
      --no-git               Skip git history analysis
      --author <AUTHOR>      Filter git stats by this author name
  -h, --help                 Print help
  -V, --version              Print version
```

---

## 🏆 Milestone Badges

Netto rewards your progress with ASCII trophies as you cross code thresholds:
- **🥉 1K Lines:** The journey begins.
- **🥈 5K Lines:** You're building something real.
- **🥇 10K Lines:** A true craftsman.
- **🏆 Legendary:** Your codebase is a monument.

---

## 🛡️ License

Distributed under the MIT License. See `LICENSE` for more information.
