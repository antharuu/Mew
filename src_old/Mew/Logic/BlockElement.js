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
exports.BlockElement = void 0;
var BlockElement = /** @class */ (function () {
    function BlockElement(options) {
        if (options === void 0) { options = {}; }
        this.tag = "div";
        this.content = "";
        var optionsExport;
        optionsExport = __assign({ tag: "div", content: "", attributes: {}, block: [], line: "" }, options);
        this.tag = optionsExport.tag;
        this.content = optionsExport.content;
        this.attributes = optionsExport.attributes;
        this.block = optionsExport.block;
        this.line = optionsExport.line;
    }
    BlockElement.prototype.attrReplace = function (attr, searchString, replaceString) {
        var returned = [];
        // @ts-ignore
        this.attributes[attr].forEach(function (c) {
            c = c.replace(searchString, replaceString);
            returned.push(c);
        });
        // @ts-ignore
        this.attributes[attr] = returned;
    };
    return BlockElement;
}());
exports.BlockElement = BlockElement;
