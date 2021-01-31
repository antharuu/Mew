"use strict";
var __spreadArrays = (this && this.__spreadArrays) || function () {
    for (var s = 0, i = 0, il = arguments.length; i < il; i++) s += arguments[i].length;
    for (var r = Array(s), k = 0, i = 0; i < il; i++)
        for (var a = arguments[i], j = 0, jl = a.length; j < jl; j++, k++)
            r[k] = a[j];
    return r;
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.Htmlify = void 0;
var BlockElement_1 = require("./Logic/BlockElement");
var Presets_1 = require("./Presets");
// noinspection JSUnusedLocalSymbols
var cl = console["log"]; // Personal shortcut TODO: remove later
var autoClosableTags = [
    "!DOCTYPE", "br", "hr", "meta", "area",
    "base", "col", "embed", "img", "input", "link",
    "param", "source", "track", "wbr", "command",
    "keygen", "menuitem"
];
function checkPresets(block, userPresets) {
    var presets = __spreadArrays(Presets_1.Presets, userPresets);
    var rBlock = block;
    presets.forEach(function (preset) {
        var _a;
        if (block.tag === preset.tag) {
            var out = preset.output;
            rBlock = new BlockElement_1.BlockElement(out);
            if ((_a = preset.callback) !== null && _a !== void 0 ? _a : false)
                rBlock = preset.callback(rBlock, block);
        }
    });
    return rBlock;
}
var Htmlify = function (blocks, i, options) {
    if (i === void 0) { i = 0; }
    var finalCode = "";
    blocks.forEach(function (block) {
        if (block.tag !== "|") {
            // @ts-ignore
            block = checkPresets(block, options.presets);
            finalCode += "<" + block.tag;
            var _loop_1 = function (attribute, value) {
                if (value !== null) {
                    finalCode += " " + attribute + "=\"";
                    var v_1 = 0;
                    if (typeof value === "string")
                        value = [value];
                    value.forEach(function (val) {
                        if (v_1 !== 0)
                            finalCode += " ";
                        finalCode += val;
                        v_1++;
                    });
                    finalCode += "\"";
                }
                else {
                    finalCode += " " + attribute;
                }
            };
            for (var _i = 0, _a = Object.entries(block.attributes); _i < _a.length; _i++) {
                var _b = _a[_i], attribute = _b[0], value = _b[1];
                _loop_1(attribute, value);
            }
            finalCode += ">";
            finalCode += block.content.trim();
            finalCode += exports.Htmlify(block.block, i + 1, options).trim(); // <- Recursive
            if (!autoClosableTags.includes(block.tag))
                finalCode += "</" + block.tag + ">";
        }
        else {
            finalCode += block.content;
        }
    });
    return finalCode;
};
exports.Htmlify = Htmlify;
//# sourceMappingURL=Htmlify.js.map