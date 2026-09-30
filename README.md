# vibe-action demo

Demo project for [Vibe Action](https://vibe-action.keygenqt.com) — a command router
for shell and LLM tasks via YAML pipelines.

A minimal Rust app whose pull requests are checked by three `ci` actions from the
[vibe-action-groups](https://github.com/keygenqt/vibe-action-groups) repository:

| Action        | What it does                                           | Result goes to |
| ------------- | ------------------------------------------------------ | -------------- |
| `ci spdx`     | Checks SPDX license headers in files changed in the PR | PR comment     |
| `ci validate` | Pre-review check: TODOs, debug code, obvious bugs      | PR comment     |
| `ci summary`  | Generates a PR description from the diff               | PR comment     |

All results are posted as one PR comment, updated on every push. A failing check
fails the workflow.

## Steps

### 1. Create the repository and the app

Create an empty repository (no README, no gitignore, no license), then:

```bash
git clone git@github.com:<you>/vibe-action-demo-ci.git
cd vibe-action-demo-ci
cargo init --name vibe-action-demo-ci
```

Replace `src/main.rs`:

```rust
// SPDX-FileCopyrightText: Copyright 2025 <you> <email@example.com>
// SPDX-License-Identifier: MIT

mod utils;

fn main() {
    println!("Hello, vibe-action demo!");
}
```

```bash
cargo run
git add .
git commit -m "Init demo app"
git branch -M main
git push -u origin main
```

### 2. Add the API key

Settings → Secrets and variables → Actions → New repository secret:
name `DEEPSEEK_API_KEY`, value — the key. Or:

```bash
gh secret set DEEPSEEK_API_KEY
```

The key is used only by the workflow at run time; it never appears in the repo.

### 3. Add the workflow

Create `.github/workflows/ci.yml` (see [ci.yml](.github/workflows/ci.yml)),
then:

```bash
git add .
git commit -m "Add ci workflow"
git push
```

### 4. Open the demo PR

```bash
git switch -c feature/utils
```

Create `src/utils.rs`:

```rust
// SPDX-FileCopyrightText: Copyright 2025 <you> <email@example.com>
// SPDX-License-Identifier: MIT

/// Add two integers.
pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(2, 3), 5);
    }
}
```

```bash
git add src/utils.rs
git commit -m "Add utils"
git push -u origin feature/utils
gh pr create --base main --title "Add utils" --body "Demo PR for vibe-action ci"
```

The workflow builds vibe-action on the first run (cached afterwards)
and posts one PR comment with the three results.

## Run locally

```bash
git clone https://github.com/keygenqt/vibe-action-demo-ci.git
cd vibe-action-demo-ci
cargo install vibe-action
vibe-action ci spdx .
```

Local runs need `~/.vibe-action/config.yaml` with the `ci` group and a model
cluster — see the [Vibe Action docs](https://vibe-action.keygenqt.com).
