import {Preset} from "../Logic/Preset";

export interface IMewConf {
    encode?: BufferEncoding;
    entry_file?: string;
    output_folder?: string;
    presets?: Preset[];
    pretty?: boolean
    variables?: Object,
}