import {BlockElement} from "./Logic/BlockElement";
import {Presets} from "./Presets";
import {Preset} from "./Logic/Preset";

// noinspection JSUnusedLocalSymbols
const {['log']: cl} = console; // Personal shortcut TODO: remove later

let variables: Object = {};

const autoClosableTags = [
    "!DOCTYPE", "br", "hr", "meta", "area",
    "base", "col", "embed", "img", "input", "link",
    "param", "source", "track", "wbr", "command",
    "keygen", "menuitem"
]

function checkPresets(block: BlockElement, userPresets: Preset[]) {
    let presets = [...Presets, ...userPresets]
    let rBlock: BlockElement = block;
    presets.forEach(preset => {
        if (block.tag === preset.tag) {
            const out = preset.output;
            rBlock = new BlockElement(out)
            if (preset.callback ?? false) rBlock = preset.callback(rBlock, block)
        }
    })
    return rBlock;
}

function checkVariables(str: string) {
    for (const [key, value] of Object.entries(variables)) {
        const regex = new RegExp("{{([ ]+)?([" + key + "]+)([ ]+)?}}", "g")
        // @ts-ignore
        const value = variables[key] ?? undefined;
        str = str.replace(regex, value);
    }
    return str;
}

export const Htmlify = (blocks: Array<BlockElement>, i: number = 0, options: Object) => {
    // @ts-ignore
    variables = {...options.variables}

    let finalCode = "";
    blocks.forEach(block => {

        if (block.tag !== "|") {
            // @ts-ignore
            block = checkPresets(block, options.presets)
            finalCode += "<" + block.tag;
            for (let [attribute, value] of Object.entries(block.attributes)) {
                if (value !== null) {
                    finalCode += " " + attribute + "=\""
                    let v = 0;
                    if (typeof value === "string") value = [value]
                    value.forEach((val: string[]) => {
                        if (v !== 0) finalCode += " ";
                        finalCode += val;
                        v++;
                    })
                    finalCode += "\""
                } else {
                    finalCode += " " + attribute
                }
            }
            if (!autoClosableTags.includes(block.tag)) finalCode += ">";
            else finalCode += " />";
            finalCode += block.content.trim()
            finalCode += Htmlify(block.block, i + 1, options).trim() // <- Recursive
            if (!autoClosableTags.includes(block.tag)) finalCode += "</" + block.tag + ">";
        } else {
            finalCode += block.content
        }
    })

    finalCode = checkVariables(finalCode)
    return finalCode;
};

