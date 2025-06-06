![SwooshUI](https://github.com/user-attachments/assets/82e2ceae-5aec-4db3-94bd-eacf41c8a591)

# SwooshUI

**SwooshUI** is a modern, declarative UI language for the browser, inspired by the elegant simplicity of macOS's file bar. It brings the crisp, intuitive design language of desktop interfaces to the web, allowing developers to rapidly create clean, native-feeling UIs without relying on bloated libraries or verbose frameworks.

## ✨ Features

- 🖱️ **MacOS-style UI syntax** — Inspired by the top file bar found in macOS.
- 💡 **Declarative language** — Define your UI in simple, readable syntax.
- 🌐 **Browser-first** — Built specifically for web environments, no build tools required.
- ⚡ **Lightweight & Fast** — No dependencies, just drop and go.
- 🎨 **Customizable** — Theme and extend components with ease.

## 🚀 Getting Started

### 1. Include SwooshUI in your HTML

```html
<script src="https://cdn.jsdelivr.net/npm/swooshui@latest/dist/swooshui.min.js"></script>
````

### 2. Define a SwooshUI component

```html
<swoosh-ui>
  menu "File" {
    item "New Window" action="newWindow()"
    item "Open..." action="openFileDialog()"
    separator
    item "Exit" action="closeApp()"
  }

  menu "Edit" {
    item "Undo" shortcut="Cmd+Z"
    item "Redo" shortcut="Shift+Cmd+Z"
  }
</swoosh-ui>
```

### 3. Add your JavaScript logic

```js
function newWindow() {
  console.log("New Window triggered");
}

function openFileDialog() {
  console.log("Open dialog launched");
}

function closeApp() {
  console.log("App closed");
}
```

## 📚 Syntax Overview

SwooshUI uses a domain-specific language within custom HTML tags:

```html
<swoosh-ui>
  menu "Menu Name" {
    item "Label" action="functionName()" [shortcut="Cmd+S"]
    separator
  }
</swoosh-ui>
```

* `menu` defines a new dropdown.
* `item` creates a clickable entry.
* `action` binds to a JavaScript function.
* `shortcut` displays keyboard shortcut hints.
* `separator` adds a dividing line.

## 🛠️ Customization

You can style SwooshUI using CSS variables:

```css
:root {
  --swoosh-bg: #f8f8f8;
  --swoosh-hover: #e0e0e0;
  --swoosh-font: "Helvetica Neue", sans-serif;
}
```

## 📦 Installation (Optional via NPM)

```bash
npm install swooshui
```

Then import it in your project:

```js
import 'swooshui';
```

## 📄 License

MIT License © 2025 \[Your Name or Organization]

## 🙌 Contributing

Want to help shape SwooshUI? Contributions, ideas, and feedback are welcome! Please see the [CONTRIBUTING.md](CONTRIBUTING.md) file for guidelines.

---

Designed to bring the native desktop feel to the web, **SwooshUI** makes your browser-based apps feel right at home.
