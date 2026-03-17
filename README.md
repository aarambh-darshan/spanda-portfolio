<div align="center">
  <br />
  <img src="public/favicon.svg" alt="Spanda Logo" width="100"/>
  <h1>Spanda Portfolio Site</h1>
  <p>
    <strong>A high-performance, Awwwards-style portfolio showcasing the <a href="https://github.com/aarambh-darshan/spanda">Spanda</a> animation library.</strong>
  </p>
  <p>
    <a href="https://crates.io/crates/spanda"><img src="https://img.shields.io/crates/v/spanda.svg" alt="Crates.io" /></a>
    <a href="https://docs.rs/spanda"><img src="https://docs.rs/spanda/badge.svg" alt="Documentation" /></a>
    <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-purple.svg" alt="License: MIT" /></a>
  </p>
  <br />
</div>

## 🌌 Overview

This repository contains the source code for the fully interactive, physics-driven portfolio website built exclusively to demonstrate the capabilities of **Spanda** — a deterministic, high-performance animation engine written in Rust.

Built with **Rust**, **Leptos**, and compiled to **WebAssembly (WASM)**, the site delivers a buttery-smooth 60fps+ experience alongside modern, award-winning web design aesthetics (glassmorphism, kinetic typography, and fluid spring physics).

## ✨ Features Demonstrated

- **High-Performance Tweening**: Multi-stop timelines driving text-reveal and split-character animations flawlessly.
- **Physics-Based Springs**: Interactive spring configurations (`SpringConfig::wobbly`, `gentle`, `stiff`) maintaining perfect velocity continuity across state changes.
- **Complex Easing**: Real-time rendering of all Spanda easing curves (Sine, Quad, Expo, Elastic, Bounce) mapping raw linear progression to eased values.
- **Shape Morphing**: Interpolating multi-dimensional arrays (`[f32; N]`) to seamlessly morph SVG paths between completely different geometries.
- **SVG Line Drawing**: Utilizing Spanda tweens to drive stroke-dashoffset animations for elegant calligraphy reveals.

## 🛠️ Built With

- **[Spanda](https://github.com/aarambh-darshan/spanda)** - Core Animation & Physics Engine
- **[Leptos](https://leptos.dev/)** - Fullstack Rust Web Framework
- **Vanilla CSS** - Bespoke styling without heavy CSS frameworks, maximizing rendering performance
- **WebAssembly** - Running native Rust performance in the browser

## 🚀 Getting Started

### Prerequisites

You'll need the Rust toolchain and the `wasm32-unknown-unknown` target installed:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### Running Locally

To start the development server and automatically rebuild on changes:

```bash
git clone https://github.com/aarambh-darshan/spanda-portfolio.git
cd spanda-portfolio
trunk serve --open
```

Head over to `http://127.0.0.1:8080` to experience the animations in action.

## 📁 Repository Structure

```text
spanda-portfolio/
├── Cargo.toml          # Rust dependencies (Spanda, Leptos, wasm-bindgen)
├── Trunk.toml          # Trunk WASM bundler configuration
├── index.html          # HTML entry point & SEO meta tags
├── robots.txt          # SEO crawlers instructions
├── sitemap.xml         # SEO site map
├── style.css           # Core styling, glassmorphism, layouts
└── src/
    ├── main.rs         # Application entry
    ├── animation.rs    # Bindings bridging Spanda loops to requestAnimationFrame
    └── sections/       # Leptos UI Components
        ├── hero.rs         # Kinetic typography & split text hero
        ├── features.rs     # Feature grid
        ├── demos.rs        # Interactive Playground (Tween, Springs, Morph)
        ├── code.rs         # Syntax-highlighted code showcase
        └── get_started.rs  # Installation instructions & footer
```

## 📜 License

This project is licensed under the [MIT License](LICENSE). 
You are free to use, modify, and distribute this software as you see fit.

## ⭐ Support

If you like the Spanda animation engine or this portfolio, consider starring the [main Spanda repository](https://github.com/aarambh-darshan/spanda) and contributing!
