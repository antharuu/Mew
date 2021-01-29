import {BlockElement} from "./BlockElement";

export class Preset {
    tag: string
    output: BlockElement
    callback: CallableFunction

    constructor(
        tag: string,
        output: BlockElement = new BlockElement(),
        callback: CallableFunction = (r, old) => r
    ) {
        this.tag = tag
        this.output = output
        this.callback = callback
    }
}