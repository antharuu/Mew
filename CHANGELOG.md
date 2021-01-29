### 0.1.2

- Custom presets
- Preset is now a class
- BlockElements now have the function "attrReplace".
- Attributes are no longer necessarily `Arrays`, but can now be `strings`.

### 0.1.1

- Add presets
- Add custom attributes
- Add more autoclosable tags

##### New Presets

- from `doctype` to  `<!DOCTYPE html>`
- from `charset utf-8` to  `<meta charset="utf-8" />`
- from `css css/main.css` to  `<link rel="stylesheet" href="css/main.css" />`
- from `a http://superLink.com My super link !` to  `<a href="http://superLink.com">My super link !</a>`
- from `img superImage.png My super image !` to  `<img src="superImage.png" alt="My super image !" />`

### 0.1.0

- Complete rewriting in [Typescript](https://www.typescriptlang.org) and multiple files.

### 0.0.2

- Add custom attributes.
- Use [jonschlinkert/pretty](https://github.com/jonschlinkert/pretty) to prettify html output.

### 0.0.1

- Support `mew` files to `html`.
- Basic transformation.
- Add `ID` & `Class` support.