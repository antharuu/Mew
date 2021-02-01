import {Preset} from "../../../src_old/Mew/Logic/Preset"; // Todo: refactor this

export interface IMewConf {
    entry_file?: string;
    output_folder?: string;
    encode?: BufferEncoding;
    presets?: Preset[];
    variables?: Object
}