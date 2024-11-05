<p align="center">
  <img width="200" src="https://i.postimg.cc/XJZbJQRp/Logo.png" alt="Mew logo">
</p>

<h1 align="center">Mew</h1>

<p align="center">
🎨 Mew - A lightweight CSS preprocessor with elegant BEM support
</p>

<p align="center">
A modern CSS preprocessor crafted in Rust, featuring intuitive nesting, variables, and seamless BEM integration. Write cleaner, more maintainable CSS with a minimalist yet powerful syntax.
</p>

<p align="center">
  <a href="https://github.com/antharuu/Mew/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/antharuu/Mew" alt="License MIT">
  </a>
  <a href="https://github.com/antharuu/Mew/issues">
    <img src="https://img.shields.io/github/issues/antharuu/Mew" alt="Open Issues">
  </a>
  <a href="https://github.com/antharuu/Mew/network/members">
    <img src="https://img.shields.io/github/forks/antharuu/Mew" alt="Forks">
  </a>
  <a href="https://github.com/antharuu/Mew/stargazers">
    <img src="https://img.shields.io/github/stars/antharuu/Mew" alt="Stars">
  </a>
</p>

## Features

- 🎯 **BEM Support**: Write cleaner CSS with automatic BEM class generation
- 🔄 **Nested Rules**: Simplified syntax for nested selectors
- 📦 **Variables**: Basic variable support for reusable values
- 📁 **Directory Processing**: Process single files or entire directories
- 🎨 **Clean Output**: Generate well-formatted CSS output

## Installation

Currently, Mew needs to be built from source:

```bash
# Clone the repository
git clone https://github.com/antharuu/Mew.git
cd Mew

# Build the project
cargo build --release

# The binary will be available in target/release/mew
```

Optional: Add to your PATH for system-wide access:

```bash
# Linux/macOS
echo 'export PATH="$PATH:/path/to/mew/target/release"' >> ~/.bashrc
source ~/.bashrc

# Windows - Add the release directory to your system's PATH environment variable
```

## Usage

Process a single file:

```bash
./target/release/mew input.mew
```

Process a directory:

```bash
./target/release/mew ./styles
```

## Syntax

### Variables

```scss
$variable-name: value;

button {
  property: $variable-name;
}
```

### Nesting with BEM

```scss
// You can omit the dot (recommended)
card {
  /* Becomes .card */

  &header {
    /* Becomes .card__header */
  }

  @primary {
    /* Becomes .card--primary */
  }
}

// Or use it explicitly if needed
.block {
  /* Also valid, becomes .block */
}
```

### Pseudo-selectors

```scss
button {
  &:hover {
    /* Becomes .button:hover */
  }
}
```

## Example

### Input (.mew file)

```scss
$nav-bg: #ffffff;
$nav-spacing: 16px;

// No dot needed for the main block
nav {
  background: $nav-bg;
  padding: $nav-spacing;

  &list {
    display: flex;
    margin: 0;
    padding: 0;
  }

  &item {
    list-style: none;
    padding: $nav-spacing;

    &:hover {
      background: #f5f5f5;
    }
  }

  @mobile {
    padding: $nav-spacing;
  }
}
```

### Output (.css file)

```css
.nav {
    background: #ffffff;
    padding: 16px;
}

.nav__list {
    display: flex;
    margin: 0;
    padding: 0;
}

.nav__item {
    list-style: none;
    padding: 16px;
}

.nav__item:hover {
    background: #f5f5f5;
}

.nav--mobile {
    padding: 16px;
}
```

## Current Limitations

- No advanced functions (like darken, lighten, mix, etc.)
- No mathematical operations beyond basic calculations
- No import/include functionality
- No mixins or extends
- No color manipulation

## Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a new branch (`git checkout -b feature/improvement`)
3. Make your changes
4. Run the tests (`cargo test`)
5. Commit your changes (`git commit -am 'Add new feature'`)
6. Push to the branch (`git push origin feature/improvement`)
7. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<p align="center">
  Made with ❤️ by <a href="https://github.com/antharuu">Antharuu</a>
</p>