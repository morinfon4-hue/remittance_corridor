# Maintenance Notes

This document explains how to keep remittance_corridor clean after updates.

## Before Commit

<pre>
git status
git diff --cached --name-only
git ls-files
</pre>

## Generated Files To Avoid

<pre>
target/
frontend/node_modules/
frontend/dist/
contracts/**/test_snapshots/
*.tsbuildinfo
.env files
</pre>

## Before Push

<pre>
git remote -v
git config user.name
git config user.email
git shortlog -sne --all
git status
</pre>
