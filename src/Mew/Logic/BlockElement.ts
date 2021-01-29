export class BlockElement {
    tag: string = "div";
    content: string = "";
    attributes: Object
    block: BlockElement[]
    line: string

    constructor(options: Object = {}) {
        let optionsExport = {
            tag: "div",
            content: "",
            attributes: {},
            block: [],
            line: "",
            ...options
        }

        this.tag = optionsExport.tag;
        this.content = optionsExport.content;
        this.attributes = optionsExport.attributes;
        this.block = optionsExport.block;
        this.line = optionsExport.line;
    }
}