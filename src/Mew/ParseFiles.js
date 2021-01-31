"use strict";
var __assign = (this && this.__assign) || function () {
    __assign = Object.assign || function(t) {
        for (var s, i = 1, n = arguments.length; i < n; i++) {
            s = arguments[i];
            for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p))
                t[p] = s[p];
        }
        return t;
    };
    return __assign.apply(this, arguments);
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.ParseFiles = void 0;
var fs = require("fs");
var Parser_1 = require("./Parser");
var Variables_1 = require("./Variables");
var cl = console["log"]; // Personal shortcut TODO: remove later
var ds = "/"; // Directory separator
/**
 * Parse a file and save into the output file
 */
var ParseFiles = /** @class */ (function () {
    function ParseFiles(file, // Entry folder
    output, // output folder
    encode, // File encode
    options) {
        if (encode === void 0) { encode = "utf-8"; }
        // @ts-ignore
        Variables_1.Variables.Data = __assign({}, options.variables);
        // Parsing and saving the code
        var M = new Parser_1.Parser(fs.readFileSync(file, encode), options);
        var finalCode = M.getFinalCode();
        var fn = file.split(".");
        fn = fn[fn.length - 2].split("/");
        var outputFile = output + ds + fn[fn.length - 1] + ".html";
        fs.mkdir(output, function (e) {
            if (!e || (e && e.code === 'EEXIST')) {
                fs.writeFile(outputFile, finalCode, function (err) {
                    if (err)
                        return console.log(err);
                });
            }
        });
    }
    return ParseFiles;
}());
exports.ParseFiles = ParseFiles;
//# sourceMappingURL=ParseFiles.js.map