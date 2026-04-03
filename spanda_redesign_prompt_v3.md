# SPANDA Website — Visual Redesign Prompt for Gemini 2.5 Pro

---

## CONTEXT — READ FIRST

You already have two codebases in your context:
1. **The full spanda source code** — you know the exact public API: all types, traits, method signatures, builder patterns, and module structure
2. **The existing spanda website source code** — the complete Leptos project with all components

Your job is **visual redesign only**. The existing website code is the source of truth for all content, copy, component structure, file names, and spanda API usage patterns. Keep everything that already exists — only replace the visual layer.

---

## YOUR TASK

Take the existing spanda website codebase and apply a complete visual overhaul:
- **Keep:** every component file, every content string, every feature name, every code example, every link, all existing spanda animation wiring, the Leptos signal patterns, the RAF driver setup
- **Replace:** all Tailwind classes, color variables, font choices, layout structure, spacing, section backgrounds, typography scale, and decorative elements

Do not add new sections. Do not remove existing sections. Do not rewrite component logic. Only change what is visual.

---

## TECH STACK (unchanged from existing project)

- **Leptos 0.7** CSR
- **spanda** — same API calls as in the existing code, no changes
- **Tailwind CSS v4** — replace all existing classes with the new design system below
- **Google Fonts** — replace the existing font imports in `index.html`

---

## NEW DESIGN DIRECTION: "KINETIC EDITORIAL"

**Concept:** High-voltage editorial magazine meets motion-design studio. Think Awwwards SOTD level. Bold, unconventional, unforgettable. Completely opposite of the current dark-orange aesthetic.

### Color Palette — replace all existing color variables in `style/main.css` `@theme {}`

```
--color-bg:           #F5F2EB   /* warm off-white parchment — primary background */
--color-bg-alt:       #EDEAE0   /* slightly darker, alternate sections */
--color-ink:          #0A0A0A   /* near-black — all body text */
--color-ink-muted:    #4A4A4A   /* secondary / muted text */
--color-accent:       #C8FF00   /* electric lime-yellow — THE signature color */
--color-accent-dark:  #8FB800   /* darker lime for hover states */
--color-code-bg:      #0D0D0D   /* jet black for code blocks and dark sections */
--color-code-text:    #C8FF00   /* lime on black for code syntax */
--color-border:       #0A0A0A   /* sharp black borders */
--color-white:        #FFFFFF
```

### Typography — replace existing font imports and font variables

- **Display:** `"Instrument Serif"` — italic, massive sizes, all section headings
- **UI / Labels:** `"DM Mono"` — uppercase, letter-spaced, nav links, section labels, badges, button text
- **Body:** `"Syne"` — geometric, modern, all paragraph and description text
- **Code:** `"JetBrains Mono"` — all code blocks (likely already in use, keep it)

Replace the Google Fonts `<link>` in `index.html` with:
```
https://fonts.googleapis.com/css2?family=Instrument+Serif:ital@0;1&family=DM+Mono:wght@300;400;500&family=Syne:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;500&display=swap
```

### Layout Principles — apply to all existing sections

- **Sections alternate** between warm parchment (`--color-bg`) and jet black (`--color-code-bg`) backgrounds
- **No card grids** — the existing features card grid becomes an editorial two-column numbered list
- **Diagonal section edges** using `clip-path` on section top/bottom edges
- **Thick 4px black borders** on key interactive elements and code blocks
- **Oversized decorative numbers** — stats use Instrument Serif Italic at ~5rem
- **Generous whitespace** — increase padding, let content breathe
- Code blocks: sharp rectangle (no border-radius), left 4px lime accent bar, jet black bg

---

## VISUAL CHANGES PER SECTION

Apply these visual changes to the existing components. Do not change component logic or content.

### Nav (`nav.rs`)
- Font: DM Mono for wordmark and all links
- Background: transparent → `--color-bg` on scroll (keep existing scroll signal logic)
- Bottom border: 1px solid `--color-border`
- No shadows, no blur effects

### Hero (`hero.rs`)
- Heading: Instrument Serif Italic, 11vw desktop / 14vw mobile
  - Line 1 (`"The pulse"`): `--color-ink`
  - Line 2 (`"of motion."`): `--color-accent`
- Subtext: Syne, `--color-ink-muted`
- Primary CTA: black bg, lime text, DM Mono uppercase
- Secondary CTA: transparent, `--color-border`, black text
- Version badge: absolute bottom-left, rotated `-90deg`, DM Mono 4rem, `--color-ink-muted`
- Background: `--color-bg` with a very subtle CSS noise texture overlay

### Stats Strip (`stats_strip.rs`)
- Background: `--color-code-bg` (jet black)
- Numbers: Instrument Serif Italic ~5rem, `--color-accent`
- Labels: DM Mono small uppercase, white
- Vertical dividers between stats: 1px `--color-accent`
- Top edge: `clip-path: polygon(0 8%, 100% 0%, 100% 100%, 0% 100%)`
- Keep existing spanda counter tween logic unchanged

### Features (`features.rs`)
- Remove the existing card grid layout entirely
- Replace with editorial two-column numbered list:
  - Left col (40%): running number `01`, `02`... in Instrument Serif ~6rem, `-webkit-text-stroke: 1px #0A0A0A`, transparent fill (outline text), `--color-ink-muted`
  - Right col (60%): feature name Syne 700 1.5rem + description Syne 400 1rem `--color-ink-muted`
  - Thin 1px `--color-border` horizontal rule between each row
- On hover: lime background flash on the number column — keep the existing spanda Spring hover signal, just change what it animates (background color to `--color-accent`)
- Section label: DM Mono small `// BUILT TO ANIMATE`, `--color-accent`
- Section heading: Instrument Serif 5rem

### Demo (`demo.rs`)
- Background: `--color-code-bg`
- Section label: DM Mono `// INTERACTIVE`, `--color-accent`
- Heading: Instrument Serif white italic 4rem
- Tab bar: inactive = outlined white pill (DM Mono); active = lime bg, black text (DM Mono)
- Canvas / SVG panel: sharp corners, `#111` background, no border-radius
- Keep all existing spanda animation logic untouched

### Code Showcase (`code_showcase.rs`)
- Background: `--color-bg`
- Section label: DM Mono `// SIMPLE API`, `--color-accent`
- Heading: Instrument Serif 4rem
- Code blocks: jet black, NO border-radius, left border 4px `--color-accent`, JetBrains Mono
  - Keywords / types: `--color-accent` (lime)
  - Other tokens: white
- Live preview panels: keep existing spanda-driven SVG animations, only restyle the container

### Quick Start (`quick_start.rs`)
- Background: `--color-code-bg`
- Heading: Instrument Serif Italic 6rem white + lime
- CTA buttons: outlined white border, lime fill on hover, DM Mono uppercase

### Footer (`footer.rs`)
- Background: `--color-bg`
- Wordmark: DM Mono large
- Link columns: Syne body text
- Bottom bar: thin top border, DM Mono small copyright
- Marquee strip: DM Mono small, keep existing spanda `Tween` driving `translateX` — just restyle the text color to `--color-ink-muted`

---

## GLOBAL CSS ADDITIONS (`style/main.css`)

Add these to the existing `main.css` alongside the updated `@theme {}` block:

```css
/* Outline text for decorative numbers */
.text-outline {
  -webkit-text-stroke: 1px var(--color-ink);
  color: transparent;
}

/* Lime gradient text utility */
.gradient-text {
  background: linear-gradient(135deg, #C8FF00, #8FB800);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

/* Custom scrollbar */
::-webkit-scrollbar { width: 4px; }
::-webkit-scrollbar-track { background: #F5F2EB; }
::-webkit-scrollbar-thumb { background: #C8FF00; border-radius: 0; }

/* Custom cursor */
* { cursor: none; }
#cursor {
  position: fixed;
  width: 8px; height: 8px;
  background: #C8FF00;
  border-radius: 50%;
  pointer-events: none;
  z-index: 9999;
  transition: transform 80ms ease;
}
```

Add a `<div id="cursor"></div>` at the top of `<body>` in `index.html` and a small `<script>` that sets its `transform: translate(x, y)` on `mousemove`.

---

## WHAT TO AVOID

- ❌ Do not change any text content, feature names, code examples, or copy
- ❌ Do not add or remove any sections
- ❌ Do not change any spanda API calls or Leptos signal logic
- ❌ Do not change component file names or the file structure
- ❌ No orange (old design color)
- ❌ No dark charcoal/brown as primary background
- ❌ No rounded card grids
- ❌ No Inter, Roboto, or system fonts
- ❌ No purple/blue gradient aesthetics
- ❌ No box shadows everywhere

---

## OUTPUT

Output the complete modified versions of every file that has visual changes. Start with `style/main.css`, then `index.html`, then each component file. For each file output it fully — no truncation, no `// unchanged` shortcuts, no `...` placeholders.
