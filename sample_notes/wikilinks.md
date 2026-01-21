# Wikilinks

Wikilinks are a simple way to link between notes.

## Syntax

Use double brackets: `[[Note Name]]`

You can also use display text: `[[Note Name|Custom Text]]`

## Examples

- [[Welcome to Tenki]]
- [[Markdown Syntax]]
- [[Keyboard Shortcuts]]

## How It Works

The tenki-core library extracts wikilinks using a regex pattern and builds a directed graph of note relationships.
