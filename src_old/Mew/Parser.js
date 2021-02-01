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
exports.Parser = void 0;
var pretty = require("pretty");
var Htmlify_1 = require("./Htmlify");
var BlockElement_1 = require("./Logic/BlockElement");
var Variables_1 = require("./Variables");
var customAttributes = [
    { name: "class", symbol: "." },
    { name: "id", symbol: "#" },
    { name: "href", symbol: "@" },
    { name: "null", symbol: ":" }
];
var Parser = /** @class */ (function () {
    function Parser(inputCode, options) {
        var _this = this;
        this.clearLineAttr = function (line) {
            line = line.trim();
            var start = line.indexOf("("), end = _this.getAttrPartLen(line) + start, block = -1, closed = false, pre = line.substr(0, start);
            return pre + line.substr(end + 1);
        };
        /**
         * returns the defined html attributes
         * @param words
         */
        this.getDefinedAttributesFrom = function (words) {
            var line = words.join(" ");
            var regex = /(?<attr>[\w]+)="(?<value>[^"\\]*(?:\\[\w\W][^"\\]*)*)"/g;
            var m, results = {};
            while ((m = regex.exec(line)) !== null) {
                if (m.index === regex.lastIndex)
                    regex.lastIndex++;
                // @ts-ignore
                results[m[1]] = [m[2]];
            }
            return results;
        };
        /**
         * Add quick attributes of the line
         * @param line
         */
        this.getAttributesFrom = function (line) {
            line = line.trim().split(" ")[0];
            var attrsSymboles = "";
            customAttributes.forEach(function (attr) {
                attrsSymboles += attr.symbol;
            });
            var regex = new RegExp('([' + attrsSymboles + '][-_/\\w]+)', 'g');
            var m, results = {};
            while ((m = regex.exec(line)) !== null) {
                if (m.index === regex.lastIndex)
                    regex.lastIndex++;
                customAttributes.forEach(function (attr) {
                    if (attr.name !== "null")
                        results = _this.addAttrFrom(results, m[1], attr.symbol, attr.name);
                    else
                        results = _this.addAttrNullFrom(results, m[1], attr.symbol, attr.name);
                });
            }
            return results;
        };
        this.addAttrFrom = function (attrs, attr, symbol, name) {
            var _a;
            if (attr.charAt(0) === symbol) {
                attr = attr.substring(1);
                // @ts-ignore
                if ((_a = attrs[name]) !== null && _a !== void 0 ? _a : false)
                    attrs[name].push(attr);
                else { // @ts-ignore
                    attrs[name] = [attr];
                }
            }
            return attrs;
        };
        this.addAttrNullFrom = function (attrs, attr, symbol, name) {
            var _a;
            if (attr.charAt(0) === symbol) {
                attr = attr.substring(1);
                // @ts-ignore
                if ((_a = attrs[name]) !== null && _a !== void 0 ? _a : false)
                    attrs[name].push(attr);
                else { // @ts-ignore
                    attrs[attr] = null;
                }
            }
            return attrs;
        };
        this.getFinalCode = function () { var _a; return (_a = _this.finalCode) !== null && _a !== void 0 ? _a : ""; };
        this.lines = inputCode.split("\n");
        this.purgeLines();
        this.blocks = this.defineBlockOf(this.lines);
        this.finalCode = pretty(Htmlify_1.Htmlify(this.blocks, 0, options));
    }
    /**
     * Base line cleanups
     */
    Parser.prototype.purgeLines = function () {
        var newLines = [];
        this.lines.forEach(function (line) {
            // Remove the line break symbol
            line = line.replace(/(\r\n|\n|\r)/gm, "");
            // Remove empty lines
            if (line.trim().length > 0)
                newLines.push(line);
        });
        this.lines = newLines;
    };
    /**
     * Set the logic from lines
     * @param lines
     * @private
     */
    Parser.prototype.defineBlockOf = function (lines) {
        var _this = this;
        var blocks = [], currentLine = 0, ignoredLines = 0;
        lines.forEach(function (line) {
            var _a;
            var indent = line.length - line.trimStart().length;
            if (ignoredLines === 0) { // Starting a new block
                ignoredLines++;
                line = Variables_1.Variables.check(line);
                if (line.length > 0) {
                    var words = line.trim().split(" ");
                    var attrib = {};
                    if (words[0].includes("(")) {
                        attrib = _this.getDefinedAttributesFrom(words);
                        line = _this.clearLineAttr(line);
                    }
                    attrib = __assign(__assign({}, attrib), _this.getAttributesFrom(line));
                    var tag = (_a = line.trim().split(/(^[-_@|\w]+)/g)[1]) !== null && _a !== void 0 ? _a : "div";
                    var content = line.trim().split(" ");
                    content.shift();
                    var checkedLines_1 = 0;
                    var blockEnded_1 = false;
                    var currBlock_1 = [];
                    lines.forEach(function (l) {
                        if (checkedLines_1 > currentLine && !blockEnded_1) {
                            var i = l.length - l.trimStart().length;
                            if (i > indent) {
                                currBlock_1.push(l);
                                ignoredLines++;
                            }
                            else
                                blockEnded_1 = true;
                        }
                        checkedLines_1 += 1;
                    });
                    var currentBlock = new BlockElement_1.BlockElement();
                    currentBlock.tag = tag;
                    currentBlock.content = content.join(" ");
                    currentBlock.attributes = attrib;
                    currentBlock.block = _this.defineBlockOf(currBlock_1); // <- Recursive
                    currentBlock.line = line; // <- Recursive
                    blocks.push(currentBlock);
                }
            }
            currentLine += 1;
            if (ignoredLines > 0)
                ignoredLines--;
        });
        return blocks;
    };
    Parser.prototype.getAttrPartLen = function (line) {
        var start = line.indexOf("("), end = 0, block = -1, closed = false;
        for (var i = 0; i < line.length; i++)
            if (!closed) {
                var l = line.charAt(i);
                if (l === "(")
                    block++;
                if (l === ")") {
                    if (block === 0) {
                        end = i;
                        closed = true;
                    }
                    block--;
                }
            }
        return end - start - 1;
    };
    return Parser;
}());
exports.Parser = Parser;
