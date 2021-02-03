import {BlockElement} from "../Logic/BlockElement";

export interface IElement {
    attributes?: {},
    attrReplace?: { attr: string, searchString: string, replaceString: string }
    block?: BlockElement[],
    content?: string,
    line?: string
    tag: string,
}