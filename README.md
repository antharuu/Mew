<p align="center">
  <img width="200" src="https://i.postimg.cc/XJZbJQRp/Logo.png" alt="Bubblegum icon">
</p>

Now usable and stable

Todo before 1.0:

- <span style="color:green">✓</span> A similar base to PUG
- <span style="color:green">✓</span> Add a preset system
- <span style="color:green">✓</span> Custom presets
- <span style="color:green">✓</span> Adding variables
- Adding mixins
- Adding loops
- Adding conditions
- Adding includes

...And probably a lot of other things

--- 

### Instalation

```
npm i mewjs
```

### Usage:

```js
const Mew = require("./Mew")

Mew.Transform({
    entry: "./tests", // Default: "./src"
    output: "./public", // Default: "./dist"
    variables: { // You can pass some variables here
        hello: "Hello world"
    }
})
```

### Exemple:

Now able to transform this

```pug
$myCss = "css/main.css"

doctype
html
  head
    viewport
    charset utf-8
    css {{myCss}}
    title {{bonjour}}
  body
    $bonjour = "Hello world"
    .container
      .row.justify-contents-center
        .col-6
          h1 {{bonjour}}
          div#maSuperImage
            img#catImage https://unsplash.com/photos/_Kbydj4K7W8 Cat super image!
        section#main.col-6
          h2 Enjoy the new MEW preprocessor!
          button:disabled My super button
          p check here
            a {{github}} Mew on Github
            |  if you want
```

To this:

```html
<!DOCTYPE html>
<html>

<head>
    <meta name="viewport"
          content="width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0">
    <meta charset="utf-8">
    <link rel="stylesheet" href="css/main.css">
    <link rel="stylesheet" href="css/secondary.css">
    <title>Bonjour le monde!</title>
</head>

<body>
<div class="container">
    <div class="row justify-contents-center">
        <div class="col-6">
            <h1>Hello world</h1>
            <div id="maSuperImage"><img id="catImage" src="https://unsplash.com/photos/_Kbydj4K7W8"
                                        alt="Cat super image!"></div>
        </div>
        <section id="main" class="col-6">
            <h2>Enjoy the new MEW preprocessor!</h2>
            <button disabled>My super button</button>
            <p>check here<a href="https://github.com/antharuu/Mew">Mew on Github</a> if you want</p>
        </section>
    </div>
</div>
</body>

</html>
```

> Beware the syntax may still have to change partially on some things.