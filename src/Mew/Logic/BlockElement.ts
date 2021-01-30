export class BlockElement {
    tag: string = "div";
    content: string = "";
    attributes: Object
    block: BlockElement[]
    line: string

    constructor(options: Object = {}) {
        let optionsExport: {
            line: string;
            attributes: {};
            block: BlockElement[];
            tag: string;
            content: string;
        };
        optionsExport = {
            tag: "div",
            content: "",
            attributes: {},
            block: [],
            line: "",
            ...options
        };

        this.tag = optionsExport.tag;
        this.content = optionsExport.content;
        this.attributes = optionsExport.attributes;
        this.block = optionsExport.block;
        this.line = optionsExport.line;
    }

    attrReplace(attr: string, searchString: string, replaceString: string) {
        let returned: string[] = []
        // @ts-ignore
        this.attributes[attr].forEach(c => {
            c = c.replace(searchString, replaceString)
            returned.push(c)
        })
        // @ts-ignore
        this.attributes[attr] = returned;
    }
}