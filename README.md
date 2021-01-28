# MEW

Currently unusable !! I work on it.

---

### Now able to transform this:
```mew
doctype
html
  head
    meta(charset="utf-8")
    link(rel="stylesheet" href="css/main.css")
    title I'm a super page!
  body
    .container
      .row.justify-contents-center
        .col-6
          h1 Hello world
        section#main.col-6
          h2 Enjoy the new MEW preprocessor!
          p check here
            a(href="#") Mew_old on Github
```
To this:

```html
<doctype>
  <html>

    <head>
      <meta charset="utf-8">
      <link rel="stylesheet" href="css/main.css">
      </link>
      <title>I'm a super page!</title>
    </head>

    <body>
      <div class="container">
        <div class="row justify-contents-center">
          <div class="col-6">
            <h1>Hello world</h1>
          </div>
          <section id="main" class="col-6">
            <h2>Enjoy the new MEW preprocessor!</h2>
            <p>check here<a href="#">Mew_old on Github</p>
          </section>
        </div>
      </div>
    </body>

  </html>
```