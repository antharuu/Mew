const {['log']: cl} = console,
    ds = "/",
    fs = require("fs");

class Parser {
    lines;
    returnLines = [];
    indent;
    returnChar = "\n";

    constructor(inputCode) {
        this.lines = inputCode.split("\n")
        this.purgeLines();
        this.indent = this.getIndentation();
    }

    getFinalCode() {
        return this.returnLines.join(this.returnChar);
    }

    getIndentation() {
        let prevIndent = 0,
            returnIndent = 1;
        this.lines.forEach((line) => {
            const indent = line.length - line.trimStart().length;
            if (indent > returnIndent) returnIndent = indent
            prevIndent = indent
        })
        return returnIndent;
    }

    purgeLines() {
        let newLines = [];
        this.lines.forEach(line => {
            line = line.replace(/(\r\n|\n|\r)/gm, "")
            if (line.trim().length > 0) {
                newLines.push(line)
            }
        })
        cl(newLines)
    }
}

class MewParser {
    constructor(
        file,
        output,
        encode = "utf-8"
    ) {
        const M = new Parser(fs.readFileSync(file, encode));

        let fn = file.split(".");
        fn = fn[fn.length - 2].split("/")
        fn = fn[fn.length - 1]

        let outputFile = output + ds + fn + ".html";

        const finalCode = M.getFinalCode();

        fs.mkdir(output, function (e) {
            if (!e || (e && e.code === 'EEXIST')) {
                fs.writeFile(outputFile, finalCode, function (err) {
                    if (err) return console.log(err);
                });
            }
        });
    }
}

const Mew = (options) => {
    const params = {
        entry: "./src",
        output: "./dist",
        encode: "utf-8",
        ...options
    }

    params.files = fs.readdirSync(params.entry);

    params.files.forEach((file) => new MewParser(
        params.entry + ds + file,
        params.output,
        params.encode
    ))
}

Mew({
    entry: "./tests"
})