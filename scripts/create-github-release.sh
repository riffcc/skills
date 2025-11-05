#!/bin/bash
# Script to create GitHub release for competition-math-researcher v1.1
# This automates the manual release process until 'pal skill release' is implemented

set -e  # Exit on error

SKILL_NAME="competition-math-researcher"
VERSION="1.1"
TAG_NAME="${SKILL_NAME}-v${VERSION}"
RELEASE_TITLE="${SKILL_NAME} v${VERSION}"
ZIP_FILE="releases/${SKILL_NAME}-v${VERSION}.zip"
NOTES_FILE="releases/RELEASE-NOTES-v${VERSION}.md"

echo "=== Creating GitHub Release for ${SKILL_NAME} v${VERSION} ==="
echo

# Step 1: Create git tag
echo "Step 1: Creating git tag '${TAG_NAME}'..."
git tag -a "${TAG_NAME}" -m "${RELEASE_TITLE} - AGI reasoning test on competition mathematics"

# Step 2: Push tag to remote
echo "Step 2: Pushing tag to remote..."
git push origin "${TAG_NAME}"

# Step 3: Create GitHub release (using gh CLI if available, otherwise manual instructions)
if command -v gh &> /dev/null; then
    echo "Step 3: Creating GitHub release via gh CLI..."
    gh release create "${TAG_NAME}" \
        "${ZIP_FILE}" \
        --title "${RELEASE_TITLE}" \
        --notes-file "${NOTES_FILE}" \
        --prerelease

    echo
    echo "✅ Release created successfully!"
    echo "View at: https://github.com/riffcc/palace-skills/releases/tag/${TAG_NAME}"
else
    echo "Step 3: gh CLI not available. Manual release instructions:"
    echo
    echo "1. Go to: https://github.com/riffcc/palace-skills/releases/new"
    echo "2. Choose tag: ${TAG_NAME}"
    echo "3. Title: ${RELEASE_TITLE}"
    echo "4. Description: Copy from ${NOTES_FILE}"
    echo "5. Upload file: ${ZIP_FILE}"
    echo "6. Check 'This is a pre-release'"
    echo "7. Click 'Publish release'"
    echo
    echo "Tag has been created and pushed. Complete the release manually on GitHub."
fi

echo
echo "Done! 🎉"
