# Press.rs Website 🦀🕸️

This is the web interface for the [Press.rs](../) archiver, implemented in Rust using WebAssembly. It allows you to compress and decompress files directly in the browser without uploading them to a server.

## 🛠️ Tech Stack

- **[Yew](https://yew.rs/)** — A component-based framework for creating web apps in Rust (similar to React).
- **[Trunk](https://trunkrs.dev/)** — Build and bundling tool for WASM applications.
- **[Tailwind CSS](https://tailwindcss.com/)** — A utility-first CSS framework for styling.
- **[WebAssembly](https://webassembly.org/)** — Running Rust code in the browser with high performance.

## 📋 Prerequisites

Before starting, ensure you have the following installed:

1. **Rust & Cargo**: [Installation guide](https://www.rust-lang.org/tools/install).
2. **WASM Target**: Add WebAssembly support to your Rust compiler:
   ```sh
   rustup target add wasm32-unknown-unknown
   ```
3. **Trunk**: The tool for building and serving the app:
   ```sh
   cargo install trunk --locked
   ```
## 🚀 Running Locally

1. Navigate to the website directory:
   ```sh
   cd website
   ```

2. Start the development server:    
   ```sh
   trunk serve --open
   ```

3. Open your browser and navigate to `http://localhost:8080`.

## 🎨 Styling

Styles are defined in `styles.css` and use Tailwind classes. The Tailwind configuration is located in `tailwind.config.js`.

## 🌍 Deployment

The project is configured for automatic deployment to GitHub Pages using GitHub Actions. The configuration is located in `.github/workflows/deploy_website.yml`. Deployment happens automatically when pushing to the main branch.
