# MathPlus
### The Simple Calculator

I wanted to learn egui, so naturally I built another calculator.

<p align="center">
  <img src="assets/screenshots/mathplus-light.png" width="45%" />
  <img src="assets/screenshots/mathplus-dark.png" width="45%" />
</p>

## What can it do?

Well, it's a calculator! But it has a few tricks:

- **Everyday maths**: Add, subtract, multiply, divide. You know the drill.
- **History**: Forgot what you calculated? Press `H`.
- **Themes**: Light, dark, or let your system decide.
- **Keyboard friendly**: Because clicking buttons gets old.
- **Copy results**: One shortcut and your answer is on the clipboard.
- **Two calculation modes**: Chained (left to right) or Standard (operator precedence).

## Get MathPlus

Running Windows, Linux, or macOS?

Grab the latest version from the **[Releases Page](https://github.com/ZFordDev/MathPlus/releases/latest)**.

## Keyboard shortcuts

For those who prefer keeping their hands on the keyboard:

| Key | What it does |
|---|---|
| `0-9` `.` `+` `-` `*` `/` | Type your maths |
| `Enter` or `=` | Calculate |
| `Backspace` | Oops, delete that |
| `Esc` or `C` | Start over |
| `Ctrl+C` | Copy the result |
| `H` | Open history |

**Made a typo?** Click anywhere in the expression field and fix it. No need to start over.

### A note about calculation modes

MathPlus has two ways of doing things:

**Chained:** Calculates from left to right.

**Standard:** Follows normal mathematical operator precedence.

You can switch between them using the **Mode** selector. Scientific mode always uses standard precedence.

## Want to build it yourself?

You'll need ***Rust 1.85+***.

```bash
git clone https://github.com/ZFordDev/MathPlus.git
cd MathPlus
cargo build --release
```

Your freshly built calculator will be waiting in `target/release/`.

## Found a bug? Have an idea?

MathPlus started as a learning project, and there's always room to improve.

Bug reports, suggestions, and pull requests are welcome!

Check out [CONTRIBUTING.md](CONTRIBUTING.md) if you'd like to help.

For security issues, please use [GitHub Security Advisories](https://github.com/ZFordDev/MathPlus/security/advisories/new) rather than opening a public issue. More details in [SECURITY.md](SECURITY.md).

## License

As usual its **`MIT`** see [LICENCE](LICENCE) for details.
