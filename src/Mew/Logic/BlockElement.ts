import {IElement} from "../Interfaces/IElement";
import {Attributes} from "../Types/Attributes";

export class BlockElement {
    tag: string = "div";
    content: string = "";
    attributes: Attributes
    block: BlockElement[]
    line: string
    infos: IElement

    constructor(elementOptions: IElement = {tag: "div"}) {
        let optionsExport: {
            attributes: {};
            block: BlockElement[];
            content: string;
            line: string;
            tag: string;
        };
        optionsExport = {
            attributes: {},
            block: [],
            content: "",
            line: "",
            tag: "div",
            ...elementOptions
        };

        this.attributes = optionsExport.attributes;
        this.block = optionsExport.block;
        this.content = optionsExport.content;
        this.infos = elementOptions;
        this.line = optionsExport.line;
        this.tag = optionsExport.tag;
    }

    attrReplace(attr: string, searchString: string, replaceString: string) {
        let returned: string[] = []
        this.attributes[attr].forEach(c => {
            c = c.replace(searchString, replaceString)
            returned.push(c)
        });
        this.attributes[attr] = returned;
    }
}