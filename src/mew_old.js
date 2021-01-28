const {['log']: cl} = console,
    ds = "/",
    fs = require("fs"),
    pretty = require('pretty');


class Parser {
    lines;
    finalCode;
    indent;
    returnChar = "\n";
    blocks;

    constructor(inputCode) {
        this.lines = inputCode.split("\n")
        this.purgeLines();
        this.getIndentation();
        this.blocks = this.defineBlockOf(this.lines)
        this.finalCode = pretty(this.generateCodeOf(this.blocks))
    }

    getFinalCode = () => this.finalCode;

    getIndentation = () => {
        let prevIndent = 0,
            returnIndent = 1;
        this.lines.forEach((line) => {
            const indent = line.length - line.trimStart().length;
            if (indent > returnIndent) returnIndent = indent
            prevIndent = indent
        })
        this.indent = returnIndent;
    };

    purgeLines = () => {
        let newLines = [];
        this.lines.forEach(line => {
            line = line.replace(/(\r\n|\n|\r)/gm, "")
            if (line.trim().length > 0) {
                newLines.push(line)
            }
        })
        this.lines = newLines;
    };

    defineBlockOf = (lines) => {
        let blocks = [];
        let block;
        let currentLine = 0;
        let ignoredLines = 0;
        lines.forEach(line => {
            const indent = line.length - line.trimStart().length;
            if (ignoredLines === 0) {
                ignoredLines++;
                let words = line.trim().split(" ");
                let attrib = {};
                if (words[0].includes("(")) {
                    attrib = this.getDefinedAttributesFrom(words)
                    line = this.clearLineAttr(line)
                    words = line.trim().split(" ");
                }
                let selector = this.splitPartsSelector(words);
                let tag = selector[0]
                if (tag.length === 0) tag = "div"
                let content = line.trim().split(" ");
                content.shift()
                let checkedLines = 0;
                let blockEnded = false;
                let currBlock = [];
                selector.shift()
                lines.forEach(l => {
                    if (checkedLines > currentLine && !blockEnded) {
                        const i = l.length - l.trimStart().length;
                        if (i > indent) {
                            currBlock.push(l);
                            ignoredLines++;
                        } else blockEnded = true;
                    }
                    checkedLines += 1;
                })

                block = {
                    tag: tag,
                    content: content.join(" "),
                    attributes: {...attrib, ...this.getAttributesFrom(selector)},
                    block: this.defineBlockOf(currBlock) // <- Recursive
                }
                blocks.push(block)
            }
            currentLine += 1;
            if (ignoredLines > 0) ignoredLines--;
        })
        return blocks;
    }

    splitPartsSelector(words) {
        let w = words[0];
        w = this.splitAttrFrom(w, ".")
        w = this.splitAttrFrom(w, "#")
        return w.split(" ")
    }

    splitAttrFrom(w, attr) {
        const s = "@@@@@"
        while (w.includes(attr)) w = w.replace(attr, s)
        while (w.includes(s)) w = w.replace(s, " " + attr)
        return w;
    }

    getAttributesFrom(selector) {
        let attrs = {}
        selector.forEach(attr => {
            attrs = this.getAttrFrom(attrs, attr, ".", "class")
            attrs = this.getAttrFrom(attrs, attr, "#", "id")
        })
        return attrs;
    }

    getAttrFrom = (attrs, attr, symbol, name) => {
        if (attr.charAt(0) === symbol) {
            attr = attr.substring(1)
            if (attrs[name] ?? false) attrs[name].push(attr)
            else attrs[name] = [attr]
        }
        return attrs;
    }

    autoClosableTags = [
        "a", "doctype", "br", "hr", "meta"
    ]

    generateCodeOf = (blocks, i = 0) => {
        let finalCode = "";
        const indent = "    ".repeat(i);
        blocks.forEach(block => {
            finalCode += indent + "<" + block.tag;
            for (const [attribute, value] of Object.entries(block.attributes)) {
                finalCode += " " + attribute + "=\""
                let v = 0;
                value.forEach(val => {
                    if (v !== 0) finalCode += " ";
                    finalCode += val;
                    v++;
                })
                finalCode += "\""
            }
            finalCode += ">";
            finalCode += block.content.trim()
            finalCode += this.generateCodeOf(block.block, i + 1).trim()
            if (!this.autoClosableTags.includes(block.tag)) {
                finalCode += "</" + block.tag + ">";
            }
            finalCode += "\n";
        })

        return finalCode;
    }

    getDefinedAttributesFrom(words) {
        words = words.join(" ")
        let attrs = {}
        let start = words.indexOf("(") + 1
        words = words.substr(start);
        let close = 0;
        let block = 0;
        let closed = false;
        for (let i = 0; i < words.length; i++) {
            if (!closed) {
                const l = words.charAt(i);
                if (l === "(") block++;
                if (l === ")") {
                    if (block === 0) {
                        close = i
                        closed = true;
                    }
                    block--;
                }
            }
        }
        let attributes = words.substr(0, close);
        const regex = /([^\s=]+)="([^"\\]*(?:\\[\w\W][^"\\]*)*)"/g;
        let m, results = {};
        while ((m = regex.exec(attributes))) {
            results[m[1]] = [m[2]];
        }
        return results;
    }

    clearLineAttr(line) {
        let start = line.indexOf("(")
        let end = line.lastIndexOf(")")
        let pre = line.substr(0, start)
        let fix = line.substr(end + 1)
        line = pre + fix;
        return line;
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

