const Mew = require("../src/Mew")

Mew.Config = {
    entry_file: "./tests/index",
    variables: {
        bonjour: "Hello world from Mew! ♥"
    }
}

let m = Mew.Compile()

console.log(m)