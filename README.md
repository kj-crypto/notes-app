# 📝 Notes App

A modern, responsive notes application built with [Svelte](https://svelte.dev/) and [Vite](https://vitejs.dev/). App integrated with [Tauri](https://tauri.app/) for desktop.
Easily create, edit, and organize your notes with a clean and fast interface.

## 🚀 Features

- Create, edit, and delete notes
- Tagging and filtering
- Light/dark theme toggle
- Responsive design
- Fast, instant updates

## 📦 Project Structure

```
notes-app/
├── src-tauri/
├── svelte/
│ ├── src/
│ │ └── components/
│ ├── static/
│ │ ├── app.css
│ │ └── modal.css
├── vite.config.ts
├── svelte.config.js
├── README.md
└── ...
```

## 🛠️ Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v24 or higher)
- [Docker](https://www.docker.com/)

### Installation

1. Clone the repository
2. Install dependencies

```bash
npm install
```

3. Build the Docker image

```bash
bash tauri-setup.sh build
```

It will build docker image for tauri app compilation process. More info after `bash tauri-setup.sh --help`

### Development

1. Run `npm run dev` to start the svelte development server
2. Run `bash tauri-setup.sh run .` in terminal. It will start docker container and attach to it in interactive mode
3. In container navigate to `src-tauri`
4. Setup the `DATA_JSON_FILENAME` environment variable to point to the data file
5. Run `cargo build` to build debug version of the tauri app. e.g. `export DATA_JSON_FILENAME=data.json && cargo build`
6. At host machine navigate to `src-tauri/target/debug` and run `./app`

## Compilation

1. Run `npm run build` to build production version of the svelte app
2. Run `bash tauri-setup.sh run .` in terminal. It will start docker container and attach to it in interactive mode
3. In container navigate to `src-tauri`
4. Setup the `DATA_JSON_FILENAME` environment variable to point to the data file
5. Run `cargo tauri build --bundles appimage` to build production version of the tauri app. e.g. `export DATA_JSON_FILENAME=data.json && cargo tauri build --bundles appimage`
6. At host machine navigate to `src-tauri/target/release/bundle` and you will find the tauri app like AppImage.

### Notes

Change static files in the `static` folder does not trigger hot reload of the development server. You need to restart the development server manually for the changes to take effect.
