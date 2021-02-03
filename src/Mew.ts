import path = require("path");

import {IMewConf} from "./Mew/Interfaces/IMewConf";
import {Parser} from "./Mew/Parser";
import * as fs from "fs";

import {DefaultMewConf} from "./Mew/Config/Conf";

import DeepMerge from "./Mew/Tools/DeepMerge";

/**
 * Custom Settings
 * @type {{encode: string, variables: {}, output_folder: string, presets: Preset[], entry_file: string}}
 */
export var Config: IMewConf = {}
let FinalConfig: IMewConf | null = null,
    JsonConfig: IMewConf | null = null

const configFileName: string = "mew.config.json"

/**
 * Search configuration file at 6 levels
 * @param location
 * @param level
 * @constructor
 */
function SearchConfigFile(location: string, level: number = 0): void {
    if (level >= 6 && JsonConfig === null) JsonConfig = {}
    else {
        const file = path.resolve(location + configFileName);
        if (fs.existsSync(file)) JsonConfig = JSON.parse(fs.readFileSync(file, "utf-8")) as IMewConf
        else SearchConfigFile("../" + location, level + 1)
    }
}

/**
 * Return the configuration of Mew
 * @returns {{encode?: BufferEncoding, pretty?: boolean, variables?: Object, output_folder?: string, presets?: Preset[], entry_file?: string}}
 * @constructor
 */
function GetConfig(): IMewConf {
    if (FinalConfig === null) {

        if (JsonConfig === null) SearchConfigFile("./")

        let Confs = DeepMerge(DefaultMewConf, JsonConfig);
        Confs = DeepMerge(Confs, Config);

        FinalConfig = Confs;
    }
    return FinalConfig;
}

/**
 * Compile and return the given code.
 * @param mewCode Code to be compiled.
 * @returns {string} Html content.
 * @constructor
 */
export const Render = (mewCode: string): string => {
    const P = new Parser(mewCode, GetConfig())
    return P.getFinalCode();
}

/**
 * Allows you to retrieve and compile the contents of a file.
 * @param mewFile mewFile Location and name of the file to be compiled.
 * @returns {string} Html content.
 * @constructor
 */
export const RenderFile = (mewFile: string = GetConfig().entry_file): string => {
    const MewConf: IMewConf = GetConfig()
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
export const Compile = (mewFile: string = GetConfig().entry_file, mewOutputFolder: string = GetConfig().output_folder): void => {
    const MewConf: IMewConf = GetConfig()

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
