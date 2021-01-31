const Mew = require("./Mew")

Mew.Transform({
    entry: "./tests", // Default: "./src"
    output: "./dist", // Default: "./dist"
    variables: {
        hello: "Hello world"
    },
    presets: [
        {
            /**
             * Exemple custom preset with FontAwesome
             * It will transform "fa b github"
             * to "<i class='fab fa-github'></i>"
             */

            tag: "fa",
            element: {
                tag: "i",
                attributes: {
                    class: "fa-icons"
                }
            },
            callback(newElement, oldElement) {
                const oldContent = oldElement.content.split(" ");
                let type = "s"
                if (oldContent.length >= 2) {
                    type = oldContent[0]
                    oldContent.shift()
                }
                oldContent.join(" ")
                newElement.attributes = {
                    ...newElement.attributes, class: [
                        `fa${type}`,
                        `fa-${oldContent}`
                    ]
                }
                return newElement; // Dont forget to return the new Element.
            }
        }
    ]
})