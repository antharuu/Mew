"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Preset = void 0;
var BlockElement_1 = require("./BlockElement");
var Preset = /** @class */ (function () {
    function Preset(tag, output, callback) {
        if (output === void 0) { output = new BlockElement_1.BlockElement(); }
        if (callback === void 0) { callback = function (r, old) { return r; }; }
        this.tag = tag;
        this.output = output;
        this.callback = callback;
    }
    return Preset;
}());
exports.Preset = Preset;
//# sourceMappingURL=Preset.js.map