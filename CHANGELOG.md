### 0.2.0

#### New features & changes

- Complete rewrite _(again)_, Clean this time.
- The use has been further simplified.
- Renaming `entry` to `entry_file` in the configuration.
- Renaming `output` to `output_folder` in the configuration.
- Now uses the prism CDN for testing *(instead of js and css files)*.
- We can now have a configuration file named `mew.config.json` at the root of the project or 6 levels deeper.
- The different configurations are superimposed and no longer replace each other.

#### Refactor

- Now all `@ts-ignore` has been removed.
- The code is now well typed and organized.
- Use of Interfaces, to make the code more readable and maintainable.

#### Bugfixes

- Variables can be correctly modified and no longer return to their original values.

### 0.1.6

- Cleaning and improvement of the `package.json`.
- Presets are now easy to use.
- Adding a preset demo in the Readme.

### 0.1.5

- The use can now be done from a simple javascript script.
- The typescript is now compiled.

### 0.1.4

- Variables can now be declared and modified directly in the Mew file.
- The Variables code is now in the Variables file.

### 0.1.3

- Adding variables.
- Added "noImplicitAny" rule for script types.

### 0.1.2

- Custom presets.
- Preset is now a class.
- BlockElements now have the function "attrReplace".
- Attributes are no longer necessarily `Arrays`, but can now be `strings`.

### 0.1.1

- Add presets.
- Add custom attributes.
- Add more autoclosable tags.

##### New Presets

- from `doctype` to  `<!DOCTYPE html>`.
- from `charset utf-8` to  `<meta charset="utf-8" />`.
- from `css css/main.css` to  `<link rel="stylesheet" href="css/main.css" />`.
- from `a http://superLink.com My super link !` to  `<a href="http://superLink.com">My super link !</a>`.
- from `img superImage.png My super image !` to  `<img src="superImage.png" alt="My super image !" />`.

### 0.1.0

- Complete rewriting in [Typescript](https://www.typescriptlang.org) and multiple files.

### 0.0.2

- Add custom attributes.
- Use [jonschlinkert/pretty](https://github.com/jonschlinkert/pretty) to prettify html output.

### 0.0.1

- Support `mew` files to `html`.
- Basic transformation.
- Add `ID` & `Class` support.