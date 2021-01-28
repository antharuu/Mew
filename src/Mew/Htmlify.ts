import {BlockElement} from "./Logic/BlockElement";

// noinspection JSUnusedLocalSymbols
const {['log']: cl} = console; // Personal shortcut TODO: remove later

const autoClosableTags = [
    "a", "doctype", "br", "hr", "meta"
]

export const Htmlify = (blocks: Array<BlockElement>, i: number = 0) => {

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
        finalCode += Htmlify(block.block, i + 1).trim() // <- Recursive
        if (!autoClosableTags.includes(block.tag)) {
            finalCode += "</" + block.tag + ">";
        }
        finalCode += "\n";
    })

    return finalCode;
};

