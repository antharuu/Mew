export class Variable {
    name: string
    value: any
    type: string | undefined = undefined

    constructor(name: string, value: any, type: string | undefined = undefined) {
        this.name = name
        this.value = value
        this.type = type
    }
}