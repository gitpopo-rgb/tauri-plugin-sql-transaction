#!/bin/bash

# Release script for tauri-plugin-sql-transaction
# Usage: ./scripts/release.sh <version>
# Example: ./scripts/release.sh 1.0.1

set -e

VERSION=$1

if [ -z "$VERSION" ]; then
    echo "Usage: ./scripts/release.sh <version>"
    echo "Example: ./scripts/release.sh 1.0.1"
    exit 1
fi

echo "🚀 Starting release process for version $VERSION..."

# Check for uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
    echo "❌ Error: You have uncommitted changes. Please commit or stash them first."
    exit 1
fi

# Update version in Cargo.toml
echo "📝 Updating Cargo.toml version..."
sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
rm Cargo.toml.bak

# Update version in package.json
echo "📝 Updating package.json version..."
sed -i.bak "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" package.json
rm package.json.bak

# Run tests
echo "🧪 Running tests..."
cargo test --lib

# Build TypeScript
echo "📦 Building TypeScript package..."
pnpm install
pnpm build

# Check Rust build
echo "🔨 Checking Rust build..."
cargo check

# Commit version changes
echo "💾 Committing version changes..."
git add Cargo.toml package.json
git commit -m "chore: bump version to $VERSION"

# Create git tag
echo "🏷️  Creating git tag v$VERSION..."
git tag "v$VERSION"

# Show next steps
echo ""
echo "✅ Version $VERSION prepared successfully!"
echo ""
echo "Next steps:"
echo "1. Review the changes: git show HEAD"
echo "2. Push to remote: git push origin main --tags"
echo "3. Publish to crates.io: cargo publish"
echo "4. Publish to npm: pnpm publish"
echo ""
echo "Or to undo these changes:"
echo "  git reset HEAD~1"
echo "  git tag -d v$VERSION"
