---
name: release
description: >
  Release workflow for the modelcards project. Bumps version in Cargo.toml,
  creates/updates CHANGELOG.md, sanity-checks README.md, commits, tags, pushes,
  and monitors the GitHub Actions release workflow until binary assets appear.
  Use when the user says "release", "cut a release", "ship a new version",
  "bump version", "publish", or anything about preparing and pushing a new release.
  Also trigger when the user runs /release.
---

# Release Skill

Automates the full release cycle for the modelcards Rust CLI project.
The workflow is interactive — the user confirms the version before anything
is committed or pushed.

## Prerequisites

- All changes must be committed before starting (the skill verifies this)
- The GitHub CLI (`gh`) must be authenticated
- Remote `origin` points to `github.com/hexerei/modelcards`

## Workflow

Run these steps in order. Stop and report to the user if any step fails.

### Step 1: Verify clean working tree

Run `git status --porcelain`. If there is any output, stop and tell the user
what's uncommitted. Do not proceed until the tree is clean.

### Step 2: Determine new version

Read the current version from `Cargo.toml` (the `version = "x.y.z"` line).
Parse it as semver (major.minor.patch).

Default bump is **minor** (0.1.3 -> 0.2.0). Present the user with options:

```
Current version: 0.1.3
  [1] patch  -> 0.1.4
  [2] minor  -> 0.2.0  (default)
  [3] major  -> 1.0.0
  [4] custom
```

Wait for the user to confirm or choose before proceeding.

### Step 3: Update Cargo.toml

Replace the version string in Cargo.toml with the new version.
Run `cargo check` to make sure the version change doesn't break anything
and to update Cargo.lock.

### Step 4: Create or update CHANGELOG.md

Use the [Keep a Changelog](https://keepachangelog.com/) format.

**If CHANGELOG.md does not exist**, create it with:
- Header and intro text per Keep a Changelog spec
- A section for every existing git tag, with commit messages grouped by type
  (Added, Changed, Fixed, Removed). Use `git log --oneline <prev-tag>..<tag>`
  for each tag range. For the first tag, use all commits up to that tag.
- A final section for the new release with commits since the last tag.

**If CHANGELOG.md already exists**, prepend a new release section with commits
since the last tag. Do not modify existing sections.

To categorize commits:
- Starts with "add" / "feat" / "implement" / "introduce" -> **Added**
- Starts with "fix" / "resolve" / "correct" -> **Fixed**
- Starts with "remove" / "delete" / "drop" -> **Removed**
- Everything else -> **Changed**

Use the tag date (or today's date for the new release) in the heading:
`## [0.2.0] - 2026-03-26`

### Step 5: Quick README sanity check

Read README.md and do a fast check:
- Does the described CLI usage still match `cargo run -- --help` output?
- Are there any obviously stale sections (e.g., referencing removed commands)?

If something looks off, tell the user what you found and suggest a fix.
Apply fixes only with user approval. If README looks fine, say so and move on.

### Step 6: Commit and tag

Stage `Cargo.toml`, `Cargo.lock`, and `CHANGELOG.md` (and `README.md` if changed).
Commit with message: `release: v<version>`

Create an annotated tag: `git tag -a v<version> -m "v<version>"`

### Step 7: Push and monitor release

Push the commit and tag:
```
git push origin main
git push origin v<version>
```

Then monitor the GitHub Actions "Release on Tag" workflow:
1. Wait a few seconds for the run to appear
2. Poll with `gh run list --workflow=release.yml --limit=1` until status is "completed"
3. Once done, check the results:
   - **upload-assets**: list the release assets with `gh release view v<version>`
     and report binary names and sizes to the user.
   - **publish-crate**: verify the crate was published by checking the workflow
     job status. If it succeeded, report the crates.io URL:
     `https://crates.io/crates/modelcards/<version>`
   - If any job **failed**: show the user the workflow URL so they can investigate:
     `gh run view <run-id> --web`

Report the final outcome clearly: version released, assets available,
crate published, or what went wrong.

### Note: crates.io token

The release workflow requires a `CARGO_REGISTRY_TOKEN` secret in the GitHub
repository settings. If the publish-crate job fails with an auth error, the
user needs to add their crates.io API token as a repository secret.
