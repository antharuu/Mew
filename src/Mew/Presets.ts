import {BlockElement} from "./Logic/BlockElement";

// noinspection JSUnusedLocalSymbols
const {['log']: cl} = console; // Personal shortcut TODO: remove later

const getAttributeBlockContent = (attrName: string, rBlock: BlockElement, oldBlock: BlockElement) => {
    rBlock.attributes[attrName] = [oldBlock.content]
    return rBlock
}

export const Presets = [
    {
        tag: "doctype",
        output: new BlockElement({tag: "!DOCTYPE", attributes: {html: null}})
    },
    {
        tag: "charset",
        output: new BlockElement({tag: "meta"}),
        callback: (rBlock: BlockElement, oldBlock: BlockElement) => getAttributeBlockContent("charset", rBlock, oldBlock)
    },
    {
        tag: "css",
        output: new BlockElement({
            tag: "link",
            attributes: {rel: ["stylesheet"]}
        }),
        callback: (rBlock: BlockElement, oldBlock: BlockElement) => getAttributeBlockContent("href", rBlock, oldBlock)
    },
    {
        tag: "a",
        output: new BlockElement({
            tag: "a",
        }),
        callback: (rBlock: BlockElement, oldBlock: BlockElement) => {
            const c = oldBlock.content.split(" ");
            if (c.length < 2) throw "A link needs at least 2 arguments"
            rBlock.attributes["href"] = [c[0]]
            c.shift()
            rBlock.content = c.join(" ")
            return rBlock
        }
    },
    {
        tag: "img",
        output: new BlockElement({
            tag: "img",
        }),
        callback: (rBlock: BlockElement, oldBlock: BlockElement) => {
            const c = oldBlock.content.split(" ");
            rBlock.attributes["src"] = [c[0]]
            if (c.length >= 2) {
                c.shift()
                rBlock.attributes["alt"] = [c.join(" ")]
            }
            return rBlock
        }
    }
]