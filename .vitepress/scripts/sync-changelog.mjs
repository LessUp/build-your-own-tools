#!/usr/bin/env node
/**
 * Sync CHANGELOG.md to docs/zh/changelog.md and docs/en/changelog.md
 * Inspired by kimi-cli's sync-changelog.mjs
 */
import { readFileSync, writeFileSync, existsSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const docsDir = join(__dirname, "..");
const rootDir = join(docsDir, "..");

const sourcePath = join(rootDir, "CHANGELOG.md");

// Chinese version
const zhTargetPath = join(docsDir, "zh", "changelog.md");
const zhHeader = `# 变更日志

本文档记录了 Build Your Own Tools 项目的每次发布变更。

`;

// English version
const enTargetPath = join(docsDir, "en", "changelog.md");
const enHeader = `# Changelog

This page documents the changes in each Build Your Own Tools release.

`;

function processChangelog(header) {
  if (!existsSync(sourcePath)) {
    console.log("CHANGELOG.md not found, skipping sync");
    return null;
  }

  let content = readFileSync(sourcePath, "utf-8");

  // Remove the HTML comment block at the top
  content = content.replace(/<!--[\s\S]*?-->\n*/g, "");

  // Remove the "# Changelog" title
  content = content.replace(/^# Changelog\n+/i, "");

  // Convert title format: ## [0.3.0] - 2025-05-14 -> ## 0.3.0 (2025-05-14)
  content = content.replace(
    /^## \[([^\]]+)\] - (\d{4}-\d{1,2}-\d{1,2})/gm,
    "## $1 ($2)"
  );

  // Remove subsection headers like ### Added, ### Changed, ### Fixed
  content = content.replace(/^### (Added|Changed|Fixed|Improved|Features|Breaking)\n+/gm, "");

  return header + content.trim() + "\n";
}

try {
  const zhContent = processChangelog(zhHeader);
  const enContent = processChangelog(enHeader);

  if (zhContent) {
    writeFileSync(zhTargetPath, zhContent);
    console.log(`Synced changelog to ${zhTargetPath}`);
  }

  if (enContent) {
    writeFileSync(enTargetPath, enContent);
    console.log(`Synced changelog to ${enTargetPath}`);
  }
} catch (error) {
  console.error("Error syncing changelog:", error.message);
  process.exit(1);
}
