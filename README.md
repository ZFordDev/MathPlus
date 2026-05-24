[![Website](https://img.shields.io/badge/Website-zford.dev-000000?style=flat-square)](https://zford.dev)
[![Ko‑Fi](https://img.shields.io/badge/Support-KoFi-FF5E5B?style=flat-square)](https://ko-fi.com/zforddev)
[![itch.io](https://img.shields.io/badge/itch.io-MathPlus-FA5C5C?style=flat-square)](https://zforddev.itch.io/mathplus)

---

# **MathPlus**

[![Download](https://img.shields.io/badge/Download-Releases-blue)](../../releases)
![Platform](https://img.shields.io/badge/Platforms-Windows%20%7C%20Linux%20(Coming%20Soon)-blue)
![Status](https://img.shields.io/badge/Status-Community%20Active-4CAF50)
![License](https://img.shields.io/badge/License-MIT-green)
![Rust](https://img.shields.io/badge/Built%20with-Rust-orange)

MathPlus is a small Windows calculator built in Rust using `egui`.  
It’s intentionally simple: quick math, a clean interface, and nothing running in the background.  
No ads, no telemetry, no “smart” features — just a calculator that opens instantly and does its job.

It’s not trying to replace anything fancy. It’s just a lightweight tool that feels nice to use.

---

## A simple Rust example you can learn from
MathPlus is also a great reference project if you're learning Rust or trying out `egui` for the first time.  
The codebase is small, well‑structured, and easy to follow — ideal for experimenting with GUI apps, state handling, and basic expression evaluation.

Feel free to explore, modify, or use it as a base for your own ideas.

---

## **What it does**
- Opens fast and stays out of the way  
- Handles standard operations (add, subtract, multiply, divide)  
- Clean, minimal UI  
- Keyboard‑friendly  
- No network calls, no background services, no hidden processes  

That’s it. Nothing complicated.

---

## **Using MathPlus**
Just launch it and start typing.

Keyboard support includes:
- `0`–`9`, `.`, `+`, `-`, `*`, `/`
- `Enter` or `=` evaluates the expression
- `Backspace` deletes the last character
- `Esc` clears the input
- `Ctrl + C` copies the current result

It’s a simple GUI app — no setup, no configuration, no learning curve.

---

## **Windows Installer**
You can grab the installer from the **Releases** section.

The installer includes:
- Start Menu shortcut  
- Optional desktop shortcut  
- Clean uninstall  
- Installs to:  
  ```
  C:\Program Files\ZFordDev\MathPlus
  ```

A portable `.exe` is also available if you prefer no install.

---

## **Contributors**
MathPlus continues to grow thanks to the community.

### ❤️ Special Thanks

**@slash-aech** — for the fantastic UI improvements in v1.2.1  
- refined button styling and layout  
- improved color palette and rounding  
- fixed layout clipping  
- added a proper copy‑to‑clipboard button  
- implemented `Ctrl + C` support  

Your work genuinely improved the feel of the app — thank you!

**@MD-Mushfiqur123** — for laying the groundwork for future expansion  
- early foundations for the updater system  
- initial structure for the scientific calculator add‑on  
- forward‑thinking contributions that help shape the next phase of MathPlus  

Your contributions set the stage for what comes next.

---

## **Project Status**
MathPlus is **Community Active**.

The core calculator is stable, but the project is evolving through community contributions and upcoming ecosystem integrations.

### Potential future additions:
- Calculation history  
- Scientific mode  
- Themes  
- Built‑in updater  
- “MathPlus++” advanced version  

---

## **License**
Released under the MIT License.  
See `LICENSE` for details.
