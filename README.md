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

## Setup

1. Default branch: `main`.
2. Add the repository secret `DEEPSEEK_API_KEY`
   (Settings → Secrets and variables → Actions).
3. Open a pull request — the `ci` workflow runs and posts results.

## Try it

The demo PR adds `src/utils.rs`. Expected result: green checks and one PR comment
with the `ci spdx`, `ci validate`, and `ci summary` results.

## Run locally

```bash
git clone https://github.com/keygenqt/vibe-action-demo-ci.git
cd vibe-action-demo-ci
cargo install vibe-action
vibe-action ci spdx .
```
