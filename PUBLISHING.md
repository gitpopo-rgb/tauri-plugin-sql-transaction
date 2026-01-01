# Publishing Guide

This guide explains how to publish `tauri-plugin-sql-transaction` to crates.io (Rust) and npm (TypeScript).

## Prerequisites

- Rust and Cargo installed
- Node.js and pnpm (or npm) installed
- [crates.io](https://crates.io/) account with API token
- [npm](https://www.npmjs.com/) account with publishing rights

## Version Management

Update version numbers in both manifests:

1. **Cargo.toml**: Update `version = "x.y.z"`
2. **package.json**: Update `"version": "x.y.z"`

Keep versions synchronized between Rust and TypeScript packages.

## Pre-publish Checklist

- [ ] All tests pass: `cargo test --lib`
- [ ] TypeScript builds successfully: `pnpm build`
- [ ] README is up to date with latest API changes
- [ ] CHANGELOG is updated with version changes
- [ ] Version numbers match in Cargo.toml and package.json
- [ ] No uncommitted changes in git

## Publishing to crates.io (Rust)

### 1. Login to crates.io

```bash
cargo login <YOUR_API_TOKEN>
```

Get your API token from [crates.io/settings/tokens](https://crates.io/me/settings).

### 2. Dry Run

Test the publishing process without actually publishing:

```bash
cargo publish --dry-run
```

This will:
- Build the crate
- Verify all dependencies
- Package the crate
- Check for errors

### 3. Publish

If the dry run succeeds:

```bash
cargo publish
```

### 4. Verify

Check that your package appears on crates.io:
- https://crates.io/crates/tauri-plugin-sql-transaction

## Publishing to npm (TypeScript/JavaScript)

### 1. Build the TypeScript Package

```bash
pnpm install
pnpm build
```

This creates the distribution files in `dist-js/`.

### 2. Update package.json

Ensure these fields are set correctly:

```json
{
  "name": "tauri-plugin-sql-transaction-api",
  "version": "1.0.0",
  "description": "SQL transaction plugin for Tauri supporting SQLite, MySQL, and PostgreSQL",
  "author": "Your Name",
  "license": "MIT OR Apache-2.0",
  "repository": {
    "type": "git",
    "url": "https://github.com/yourusername/tauri-plugin-sql-transaction"
  },
  "keywords": ["tauri", "plugin", "sql", "transaction", "sqlite", "mysql", "postgresql"]
}
```

### 3. Login to npm

```bash
npm login
```

### 4. Dry Run (Optional)

Check what will be published:

```bash
npm publish --dry-run
```

### 5. Publish

```bash
npm publish
```

Or with pnpm:

```bash
pnpm publish
```

### 6. Verify

Check that your package appears on npm:
- https://www.npmjs.com/package/tauri-plugin-sql-transaction-api

## Post-publish Steps

1. **Tag the release in git:**
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. **Create GitHub Release:**
   - Go to your repository on GitHub
   - Click "Releases" → "Create a new release"
   - Select the tag you just created
   - Add release notes describing changes

3. **Update documentation:**
   - Ensure README on GitHub is up to date
   - Update any external documentation or tutorials

## Automation (Optional)

Consider setting up GitHub Actions to automate publishing:

### .github/workflows/publish.yml

```yaml
name: Publish

on:
  push:
    tags:
      - 'v*'

jobs:
  publish-rust:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo publish --token ${{ secrets.CARGO_TOKEN }}

  publish-npm:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: pnpm/action-setup@v2
        with:
          version: 8
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
          registry-url: 'https://registry.npmjs.org'
      - run: pnpm install
      - run: pnpm build
      - run: pnpm publish --no-git-checks
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
```

### Required Secrets

Add these secrets to your GitHub repository:
- `CARGO_TOKEN`: Your crates.io API token
- `NPM_TOKEN`: Your npm access token

## Troubleshooting

### Cargo publish fails with "already published"

You cannot republish the same version. Increment the version number and try again.

### npm publish fails with authentication error

Run `npm login` again to refresh your authentication.

### Package is too large

Add unwanted files to `.npmignore` or use the `files` field in package.json to specify what to include.

### Tests fail before publish

Ensure all tests pass locally:
```bash
cargo test --lib
cargo check
pnpm build
```

## Version Numbering

Follow [Semantic Versioning (SemVer)](https://semver.org/):

- **MAJOR** (1.x.x): Breaking changes
- **MINOR** (x.1.x): New features, backward compatible
- **PATCH** (x.x.1): Bug fixes, backward compatible

## Support

For questions or issues with publishing, please open an issue on GitHub.
