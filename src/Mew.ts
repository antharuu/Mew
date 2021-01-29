import * as fs from "fs";
import {ParseFiles} from "./Mew/ParseFiles"
import {BlockElement} from "./Mew/Logic/BlockElement";
import {Preset} from "./Mew/Logic/Preset";

const ds = "/"; // Directory separator

/**
 * @param options entry, output, encode
 * @constructor
 */
const Mew = (options) => {
    const params = {
        entry: "./src",     // Entry folder
        output: "./dist",   // Output folder
        encode: "utf-8",    // File encode
        presets: [],        // Custom presets
        ...options
    }

    // Getting the list of files to be parsed.
    params.files = fs.readdirSync(params.entry);

    // Parse each file.
    params.files.forEach((file) => new ParseFiles(
        params.entry + ds + file,
        params.output,
        params.encode,
        {
            presets: params.presets
        }
    ))
}

/**
 * Test implementation
 * TODO: remove this at the end
 **/
Mew({
    entry: "./tests",
    presets: []
})