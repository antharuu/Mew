const pretty = require("pretty")
import {Htmlify} from "./Htmlify"
import {BlockElement} from "./Logic/BlockElement";

const customAttributes = [
    {name: "class", symbol: "."},
    {name: "id", symbol: "#"},
    {name: "href", symbol: "@"}
]

export class Parser {
    private lines: string[]; // Lines to work on
    blocks: BlockElement[]; // Blocks of logic

    finalCode: string; // Code to export

    constructor(inputCode: string, options: Object) {
        this.lines = inputCode.split("\n")
        this.purgeLines();
        this.blocks = this.defineBlockOf(this.lines)
        this.finalCode = pretty(Htmlify(this.blocks, 0, options))
    }

    /**
     * Base line cleanups
     */
    private purgeLines(): void {
        let newLines: Array<string> = [];
        this.lines.forEach(line => {

            // Remove the line break symbol
            line = line.replace(/(\r\n|\n|\r)/gm, "")

            // Remove empty lines
            if (line.trim().length > 0) newLines.push(line)
        })
        this.lines = newLines;
    }

    /**
     * Set the logic from lines
     * @param lines
     * @private
     */
    private defineBlockOf(lines: Array<string>) {
        let blocks: BlockElement[] = [],
            currentLine: number = 0,
            ignoredLines: number = 0;
        lines.forEach(line => {
            const indent = line.length - line.trimStart().length;

            if (ignoredLines === 0) { // Starting a new block
                ignoredLines++;
                let words = line.trim().split(" ");

                let attrib = {};
                if (words[0].includes("(")) {
                    attrib = this.getDefinedAttributesFrom(words)
                    line = this.clearLineAttr(line);
                }
                attrib = {...attrib, ...this.getAttributesFrom(line)}

                let tag = line.trim().split(/(^[-_@|\w]+)/g)[1] ?? "div"

                let content: string[] = line.trim().split(" ");
                content.shift()

                let checkedLines = 0;
                let blockEnded = false;
                const currBlock: string[] = [];

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

                const currentBlock = new BlockElement()
                currentBlock.tag = tag
                currentBlock.content = content.join(" ")
                currentBlock.attributes = attrib
                currentBlock.block = this.defineBlockOf(currBlock) // <- Recursive
                currentBlock.line = line // <- Recursive

                blocks.push(currentBlock)
            }

            currentLine += 1;
            if (ignoredLines > 0) ignoredLines--;
        })

        return blocks;
    }

    clearLineAttr = (line: string): string => {
        line = line.trim()
        let start = line.indexOf("("),
            end = 0,
            block = -1,
            closed: boolean = false,
            pre: string = line.substr(0, start)
        for (let i = 0; i < line.length; i++) if (!closed) {
            const l = line.charAt(i);
            if (l === "(") block++;
            if (l === ")") {
                if (block === 0) {
                    end = i
                    closed = true;
                }
                block--;
            }
        }
        return pre + line.substr(end + 1);
    };

    /**
     * returns the defined html attributes
     * @param words
     */
    getDefinedAttributesFrom = (words: string[]): {} => {
        let line: string = words.join(" ")
        const regex = /(?<attr>[\w]+)="(?<value>[^"\\]*(?:\\[\w\W][^"\\]*)*)"/g;
        let m: RegExpExecArray, results: Object = {};
        while ((m = regex.exec(line)) !== null) {
            if (m.index === regex.lastIndex) regex.lastIndex++;
            // @ts-ignore
            results[m[1]] = [m[2]];
        }
        return results
    };

    /**
     * Add quick attributes of the line
     * @param line
     */
    getAttributesFrom = (line: string): Object => {
        line = line.trim().split(" ")[0]

        let attrsSymboles: string = "";
        customAttributes.forEach(attr => {
            attrsSymboles += attr.symbol
        })

        const regex = new RegExp('([' + attrsSymboles + '][-_/\\w]+)', 'g');
        let m: RegExpExecArray, results = {};
        while ((m = regex.exec(line)) !== null) {
            if (m.index === regex.lastIndex) regex.lastIndex++;
            customAttributes.forEach(attr => {
                results = this.addAttrFrom(results, m[1], attr.symbol, attr.name)
            })
        }
        return results
    };

    addAttrFrom = (attrs: Object, attr: string, symbol: string, name: string) => {
        if (attr.charAt(0) === symbol) {
            attr = attr.substring(1)
            // @ts-ignore
            if (attrs[name] ?? false) attrs[name].push(attr)
            else { // @ts-ignore
                attrs[name] = [attr]
            }
        }
        return attrs;
    }

    getFinalCode = () => this.finalCode ?? "";
}