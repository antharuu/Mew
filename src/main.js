const Mew = require("./Mew")

Mew.Transform({
    entry: "./tests", // Default: "./src"
    output: "./dist", // Default: "./dist"
    variables: {
        hello: "Hello world"
    }
})