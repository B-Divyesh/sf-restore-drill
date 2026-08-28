# Restore Drill visual thesis

## Direction: recovery proof as a halftone field report

Restore Drill looks like a printed verification sheet from a machine room: warm
uncoated paper, near-black green ink, one vermilion inspection stamp, coarse
halftone shadows, registration marks, and terse monospaced evidence. The image
of an archive passing through an isolated test chamber explains the product's
job instead of decorating it. It deliberately avoids polished cloud-dashboard
gradients: this product is about physical, inspectable evidence.

This is an explicitly single-mode, light treatment. The paper color is always
painted so browser or OS theme defaults cannot alter the report metaphor.

## Tokens

| Role | Token | Value | Use |
| --- | --- | --- | --- |
| Paper | `--paper` | `#F3EBD7` | page background |
| Paper deep | `--paper-deep` | `#E5D8BA` | grouped evidence and code |
| Ink | `--ink` | `#13231D` | primary text and outlines |
| Muted ink | `--ink-muted` | `#526158` | secondary text |
| Vermilion | `--signal` | `#B7371E` | primary action and inspection stamp |
| White ink | `--on-signal` | `#FFF9EA` | text on vermilion |
| Verified | `--success` | `#17613C` | success, always paired with a label |
| Caution | `--warning` | `#805100` | warnings, always paired with an icon/label |
| Failure | `--danger` | `#9A271B` | errors, always paired with a label |
| Focus | `--focus` | `#1D5F86` | 3 px keyboard focus ring |

All normal-size text combinations meet 4.5:1. Vermilion is used as a solid
control with white-ink text and never as low-contrast body copy.

## Type and spacing

- Display/editorial: Georgia, `Times New Roman`, serif. Its engraved letterforms
  make the title feel like a durable field manual.
- Evidence/UI: `ui-monospace`, SFMono-Regular, Consolas, monospace. It makes
  hashes, durations, commands, and states scan as operational evidence.
- No web-font request is made; the system stacks are local, fast, and private.
- Scale: 14 / 16 / 20 / 28 / clamp(44, 7vw, 88) px. Body is never below 16 px.
- Spacing follows an 8 px base with 4 px optical adjustments. Reading measure
  is capped at 70 characters; major bands use 64–112 px of vertical space.

## Composition and interaction grammar

The desktop hero is an asymmetric two-column broadsheet: decisive copy at left,
the test-chamber illustration at right. Evidence is presented as ruled log rows,
not a grid of interchangeable cards. Labels are uppercase and tracked like
inspection marks; buttons are rectangular, slightly offset on press, and at
least 44 px tall. On a 390 px phone, navigation collapses, the hero stacks, the
decorative crop is reduced, and terminal lines wrap without horizontal
scrolling.

The web walkthrough uses a dark evidence terminal and a paper report slip. Its
playback can be paused or restarted, its output region is keyboard focusable,
and new lines are announced through a polite live region.

## Motion policy

Only state changes move. The hero registers into place once (500 ms, transform
and opacity), buttons depress by 2 px, and recorded lines appear in sequence.
Nothing loops. Under
`prefers-reduced-motion: reduce`, transforms and sequencing are removed and all
states switch instantly using opacity or direct replacement.

## Original asset plan and provenance

- `site/public/restore-chamber.webp`: generated specifically for Restore Drill
  using the factory image generator (`/opt/fleet/lib/gen-image.sh`, deployment
  recorded in its adjacent generation metadata), then locally converted to
  WebP. Prompt: “Wide editorial cutaway of a sealed backup archive descending
  through an isolated cylindrical test chamber and emerging as a healthy
  database stack, 1950s technical manual screen print, coarse two-color
  halftone dots, warm paper, near-black green and vermilion, strong silhouette,
  right-weighted composition with calm negative space, no words, no letters,
  no logos, no watermark.” Generated artwork is project-original and used as
  the hero explanation; no stock assets or third-party icon sets are used.
- Registration crosses, dotted textures, probe paths, and status marks are
  hand-made CSS/SVG primitives in the application. They carry no third-party
  provenance.
- `site/public/restore-drill-og.jpg` and `site/public/apple-touch-icon.png` are
  local crops of the project-original `restore-chamber.webp` artwork. They add
  social and device previews without introducing a third-party asset.
