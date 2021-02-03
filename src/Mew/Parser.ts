import pretty = require("pretty");
import {Htmlify} from "./Htmlify";

import {IMewConf} from "./Interfaces/IMewConf";
import {BlockElement} from "./Logic/BlockElement";

import {CustomAttributes} from "./Config/CustomAttributes";
import {Attributes} from "./Types/Attributes";
import {Variables} from "./Variables";

export class Parser {
    InputCode: string[];
    Conf: IMewConf;
    Blocks: BlockElement[]

    constructor(InputCode: string, Conf: IMewConf) {
        this.Conf = Conf;
        this.InputCode = this.purgeCode(InputCode.split("\n"))
        Variables.Data = {...Variables.Data, ...Conf.variables}
        this.Blocks = this.defineBlocksOf(this.InputCode)
    }

    /**
     * Return the final code
     * @returns {string}
     */
    public getFinalCode(): string {
        let htmlCode = Htmlify(this.Blocks, 0, this.Conf)
        if (this.Conf.pretty) htmlCode = pretty(htmlCode)
        return htmlCode;
    }

    /**
     * Remove unsable or useless code
     * @param lines {string}
     * @returns {string[]}
     */
    private purgeCode(lines: string[]): string[] {
        let newLines: string[] = [];
        lines.forEach(line => {
            // Remove the line break symbols
            line = line.replace(/(\r\n|\n|\r)/gm, "")

            // Remove empty lines
            if (line.trim().length > 0) newLines.push(line)
        })
        return newLines;
    }

    private defineBlocksOf(InputCode: string[]): BlockElement[] {
        let returnBlocks: BlockElement[] = [],
            currentLine: number = 0,
            ignoredLines: number = 0;

        InputCode.forEach(line => {
            const indent = line.length - line.trimStart().length;
            if (ignoredLines === 0) { // Starting a new block
                ignoredLines++;

                line = Variables.check(line)

                if (line.length > 0) {
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

                    InputCode.forEach(l => {
                        if (checkedLines > currentLine && !blockEnded) {
                            const i = l.length - l.trimStart().length;
                            if (i > indent) {
                                currBlock.push(l);
                                ignoredLines++;
                            } else blockEnded = true;
                        }
                        checkedLines += 1;
                    })

                    const currentBlock = new BlockElement({
                        tag: tag,
                        content: content.join(" "),
                        attributes: attrib,
                        block: this.defineBlocksOf(currBlock), // <- Recursive
                        line: line
                    })

                    returnBlocks.push(currentBlock)
                }
            }

            currentLine += 1;
            if (ignoredLines > 0) ignoredLines--;
        })

        return returnBlocks;
    }

    /**
     * returns the defined html attributes
     * @param words
     * @return {Attributes}
     */
    private getDefinedAttributesFrom = (words: string[]): {} => {
        let line: string = words.join(" ")
        let regex = /(?<attr>[\w]+)="(?<value>[^"\\]*(?:\\[\w\W][^"\\]*)*)"/g;
        let m: RegExpExecArray, results: Attributes = {};
        while ((m = regex.exec(line)) !== null) {
            if (m.index === regex.lastIndex) regex.lastIndex++;
            results[m[1]] = [m[2]];
        }
        return results
    };

    /**
     * Return the length of the Attributes part
     * @param line
     * @returns {number}
     */
    private static getAttrPartLen(line: string) {
        let start = line.indexOf("("),
            end = 0,
            block = -1,
            closed = false;

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
        return end - start - 1;
    }

    /**
     * Add quick attributes of the line
     * @param line
     */
    private getAttributesFrom = (line: string): Attributes => {
        line = line.trim().split(" ")[0]

        let attrsSymboles: string = "";
        CustomAttributes.forEach(attr => attrsSymboles += attr.symbol)

        const regex = new RegExp('([' + attrsSymboles + '][-_/\\w]+)', 'g');
        let m: RegExpExecArray, results: Attributes = {};
        while ((m = regex.exec(line)) !== null) {
            if (m.index === regex.lastIndex) regex.lastIndex++;
            CustomAttributes.forEach(attr => {
                if (attr.name !== "null") results = this.addAttrFrom(results, m[1], attr.symbol, attr.name) as Attributes
                else results = this.addAttrNullFrom(results, m[1], attr.symbol, attr.name) as Attributes
            })
        }
        return results
    };

    /**
     * Remove attributes part
     * @param line
     * @returns {string}
     */
    private clearLineAttr = (line: string): string => {
        line = line.trim()
        let start = line.indexOf("("),
            end = Parser.getAttrPartLen(line) + start,
            pre: string = line.substr(0, start)
        return pre + line.substr(end + 1);
    };

    /**
     * Return attributes
     * @param attrs
     * @param attr
     * @param symbol
     * @param name
     * @returns {Attributes}
     */
    private addAttrFrom = (attrs: Attributes, attr: string, symbol: string, name: string) => {
        if (attr.charAt(0) === symbol) {
            attr = attr.substring(1)
            if (attrs[name] ?? false) attrs[name].push(attr)
            else {
                attrs[name] = [attr]
            }
        }
        return attrs;
    }

    /**
     * Return attributes with no values
     * @param attrs
     * @param attr
     * @param symbol
     * @param name
     * @returns {Attributes}
     */
    private addAttrNullFrom = (attrs: Attributes, attr: string, symbol: string, name: string) => {
        if (attr.charAt(0) === symbol) {
            attr = attr.substring(1)
            if (attrs[name] ?? false) attrs[name].push(attr)
            else {
                attrs[attr] = null
            }
        }
        return attrs;
    }
}