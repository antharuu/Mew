import {IMewConf} from "./Interfaces/IMewConf";

export class Parser {
    InputCode: string;
    Conf: IMewConf;

    constructor(InputCode: string, Conf: IMewConf) {
        this.InputCode = InputCode;
        this.Conf = Conf;
    }

    /**
     * Return the finalCode
     * @returns {string}
     */
    getFinalCode(): string {
        return "Pouet";
    }
}