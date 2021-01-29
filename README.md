<p align="center">
  <img width="200" src="https://i.postimg.cc/XJZbJQRp/Logo.png" alt="Bubblegum icon">
</p>

Now usable and stable

Todo before 1.0:
- [x] A similar base to PUG 
- [x] Add a preset system
- [x] Custom presets
- [ ] Adding variables
- [ ] Adding mixins
- [ ] Adding loops
- [ ] Adding conditions
- [ ] Adding includes

...And probably a lot of other things

--- 

### Now able to transform this:
```pug
doctype
html
  head
    charset utf-8
    css css/main.css
    title Je suis une super page, enfin je crois
  body
    .container
      .row.justify-contents-center
        .col-6
          h1 Hello world
          div#maSuperImage
            img https://unsplash.com/photos/_Kbydj4K7W8 Cat super image!
        section#main.col-6
          h2 Enjoy the new MEW preprocessor!
          p check here
            a # Mew on Github
            |  if you want
```
To this:

```html
<!DOCTYPE html />
<html>

<head>
    <meta charset="utf-8" />
    <link rel="stylesheet" href="css/main.css" />
    <title>Je suis une super page, enfin je crois</title>
</head>

<body>
<div class="container">
    <div class="row justify-contents-center">
        <div class="col-6">
            <h1>Hello world</h1>
            <div id="maSuperImage"><img src="https://unsplash.com/photos/_Kbydj4K7W8" alt="Cat super image!" /></div>
        </div>
        <section id="main" class="col-6">
            <h2>Enjoy the new MEW preprocessor!</h2>
            <p>check here<a href="#">Mew on Github</a> if you want</p>
        </section>
    </div>
</div>
</body>

</html>
```