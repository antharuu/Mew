import * as fs from "fs";
import {ParseFiles} from "./Mew/ParseFiles"

const ds = "/"; // Directory separator

/**
 * @param options entry, output, encode
 * @constructor
 */
const Mew = (options) => {
    const params = {
        entry: "./src",     // Entry folder
        output: "./dist",   // output folder
        encode: "utf-8",    // File encode
        ...options
    }

    // Getting the list of files to be parsed.
    params.files = fs.readdirSync(params.entry);

    // Parse each file.
    params.files.forEach((file) => new ParseFiles(
        params.entry + ds + file,
        params.output,
        params.encode
    ))
}

/**
 * Test implementation
 * TODO: remove this at the end
 **/
Mew({
    entry: "./tests"
})