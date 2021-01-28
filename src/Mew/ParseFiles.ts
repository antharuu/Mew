import * as fs from "fs";
import {Parser} from "./Parser";

const {['log']: cl} = console; // Personal shortcut TODO: remove later
const ds = "/"; // Directory separator

/**
 * Parse a file and save into the output file
 */
export class ParseFiles {
    constructor(
        file: string,                       // Entry folder
        output: string,                     // output folder
        encode: BufferEncoding = "utf-8"    // File encode
    ) {
        // Parsing and saving the code
        const M = new Parser(fs.readFileSync(file, encode));
        const finalCode = M.getFinalCode();

        let fn = file.split(".");
        fn = fn[fn.length - 2].split("/")
        let outputFile = output + ds + fn[fn.length - 1] + ".html";
        fs.mkdir(output, function (e) {
            if (!e || (e && e.code === 'EEXIST')) {
                fs.writeFile(outputFile, finalCode, function (err) {
                    if (err) return console.log(err);
                });
            }
        });
    }
}