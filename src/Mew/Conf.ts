import {IMewConf} from "./Interfaces/IMewConf";

/**
 * Default settings
 * @type {{encode: string, variables: {}, output_folder: string, presets: Preset[], entry_file: string}}
 */
export const DefaultMewConf: IMewConf = {
    encode: "utf-8",
    entry_file: "./src/index",
    output_folder: "./dist",
    presets: [],
    variables: {}
}