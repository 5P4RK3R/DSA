#!/bin/bash

# Exit on errors
set -e

# Build the book
mdbook build

# Navigate to the book directory
cd book

# Initialize a new git repository
git init
git add -A

# Commit the changes
git commit -m "Deploy book to GitHub Pages"

# Push to the gh-pages branch
git push -f https://github.com/5P4RK3R/Library.git rust:gh-pages

# Return to the previous directory
cd ..
