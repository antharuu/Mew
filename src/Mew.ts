import {IMewConf} from "./Mew/Interfaces/IMewConf";
import {Parser} from "./Mew/Parser";
import * as fs from "fs";

import {DefaultMewConf} from "./Mew/Conf";

/**
 * Custom Settings
 * @type {{encode: string, variables: {}, output_folder: string, presets: Preset[], entry_file: string}}
 */
export var Config: IMewConf = {}

/**
 * Compile and return the given code.
 * @param mewCode Code to be compiled.
 * @returns {string} Html content.
 * @constructor
 */
export const Render = (mewCode: string): string => {
    const P = new Parser(mewCode, {...DefaultMewConf, ...Config})
    return P.getFinalCode();
}

/**
 * Allows you to retrieve and compile the contents of a file.
 * @param mewFile mewFile Location and name of the file to be compiled.
 * @returns {string} Html content.
 * @constructor
 */
export const RenderFile = (mewFile: string = Config.entry_file): string => {
    const MewConf: IMewConf = {...DefaultMewConf, ...Config}
    return Render(
        fs.readFileSync(
            formatMewFilename(mewFile),
            MewConf.encode
        )
    );
}

/**
 * Allows you to compile a mew file into an html file.
 * @param mewFile Location and name of the file to be compiled.
 * @param mewOutputFolder Location of the output folder.
 * @constructor
 */
export const Compile = (mewFile: string = Config.entry_file, mewOutputFolder: string = Config.output_folder): void => {
    const MewConf: IMewConf = {...DefaultMewConf, ...Config}

    if (mewFile === undefined) mewFile = MewConf.entry_file;
    if (mewOutputFolder === undefined) mewOutputFolder = MewConf.output_folder;

    let fileParts: string[] = formatMewFilename(mewFile).split(".");
    fileParts = fileParts[fileParts.length - 2].split("/")
    let outputFile = mewOutputFolder + "/" + fileParts[fileParts.length - 1] + ".html";

    fs.mkdir(mewOutputFolder, e => {
        if (!e || (e && e.code === 'EEXIST')) {
            fs.writeFile(outputFile, RenderFile(mewFile), (err) => {
                if (err) return console.log(err);
            });
        }
    });
}

/**
 * Format the filename
 * @param mewFile
 * @returns {string}
 */
const formatMewFilename = (mewFile: string) => (mewFile.substr(-4) !== ".mew") ? mewFile + ".mew" : mewFile;
