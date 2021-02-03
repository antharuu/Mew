import {BlockElement} from "./Logic/BlockElement";
import {Preset} from "./Logic/Preset";
import {IMewConf} from "./Interfaces/IMewConf";

import {SelfClosableTags} from "./Config/SelfClosableTags";
import {getPresetsFrom, Presets} from "./Config/Presets";

function checkPresets(block: BlockElement, userPresets: Preset[]) {
    let presets = [...Presets, ...getPresetsFrom(userPresets)]
    let rBlock: BlockElement = block;
    presets.forEach(preset => {
        if (block.tag === preset.tag) {
            const out = preset.output;
            rBlock = new BlockElement(out.infos)
            if (preset.callback ?? false) rBlock = preset.callback(rBlock, block)
        }
    })
    return rBlock;
}

export const Htmlify = (blocks: Array<BlockElement>, i: number = 0, options: IMewConf) => {
    let finalCode = "";
    blocks.forEach(block => {

        if (block.tag !== "|") {
            block = checkPresets(block, options.presets)
            finalCode += "<" + block.tag;
            for (let [attribute, value] of Object.entries(block.attributes)) {
                if (value !== null) {
                    finalCode += " " + attribute + "=\""
                    let v = 0;
                    if (typeof value === "string") value = [value]
                    value.forEach((val: string) => {
                        if (v !== 0) finalCode += " ";
                        finalCode += val;
                        v++;
                    })
                    finalCode += "\""
                } else {
                    finalCode += " " + attribute
                }
            }
            finalCode += ">";
            finalCode += block.content.trim()
            finalCode += Htmlify(block.block, i + 1, options).trim() // <- Recursive
            if (!SelfClosableTags.includes(block.tag)) finalCode += "</" + block.tag + ">";
        } else {
            finalCode += block.content
        }
    })
    return finalCode;
};