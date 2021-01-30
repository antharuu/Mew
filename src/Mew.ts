import * as fs from "fs";
import {ParseFiles} from "./Mew/ParseFiles"
import {BlockElement} from "./Mew/Logic/BlockElement";
import {Preset} from "./Mew/Logic/Preset";

const ds = "/"; // Directory separator

/**
 * @param options entry, output, encode
 * @constructor
 */
const Mew = (options: Object) => {
    let params: {
        files: string[];
        output: string;
        encode: BufferEncoding;
        entry: string;
        presets: Preset[];
        variables: Object
    }
    params = {
        variables: {},
        files: [],          // List of files
        entry: "./src",     // Entry folder
        output: "./dist",   // Output folder
        encode: "utf-8",    // File encode
        presets: [],        // Custom presets
        ...options          // User options
    };

    // Getting the list of files to be parsed. 
    params.files = fs.readdirSync(params.entry);

    // Parse each file.
    params.files.forEach((file: string) => new ParseFiles(
        params.entry + ds + file,
        params.output,
        params.encode,
        {
            presets: params.presets,
            variables: params.variables
        }
    ))
}

/**
 * Test implementation
 * TODO: remove this at the end
 **/
Mew({
    entry: "./tests",
    presets: [],
    variables: {
        bonjour: "Bonjour le monde!",
        github: "https://github.com/antharuu/Mew"
    }
})