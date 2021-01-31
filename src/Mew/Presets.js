"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Presets = exports.getPresetsFrom = void 0;
var BlockElement_1 = require("./Logic/BlockElement");
var Preset_1 = require("./Logic/Preset");
var getAttributeBlockContent = function (attrName, rBlock, oldBlock) {
    // @ts-ignore
    rBlock.attributes[attrName] = [oldBlock.content];
    return rBlock;
};
var getPresetsFrom = function (userPresets) {
    var presets = [];
    for (var _i = 0, userPresets_1 = userPresets; _i < userPresets_1.length; _i++) {
        var preset = userPresets_1[_i];
        // @ts-ignore
        presets.push(new Preset_1.Preset(preset.tag, new BlockElement_1.BlockElement(preset.element), preset.callback));
    }
    return presets;
};
exports.getPresetsFrom = getPresetsFrom;
exports.Presets = [
    new Preset_1.Preset("doctype", new BlockElement_1.BlockElement({ tag: "!DOCTYPE", attributes: { html: null } })),
    new Preset_1.Preset("charset", new BlockElement_1.BlockElement({ tag: "meta" }), function (rBlock, oldBlock) { return getAttributeBlockContent("charset", rBlock, oldBlock); }),
    new Preset_1.Preset("css", new BlockElement_1.BlockElement({ tag: "link", attributes: { rel: ["stylesheet"] } }), function (rBlock, oldBlock) { return getAttributeBlockContent("href", rBlock, oldBlock); }),
    new Preset_1.Preset("a", new BlockElement_1.BlockElement({
        tag: "a",
    }), function (rBlock, oldBlock) {
        var c = oldBlock.content.split(" ");
        if (c.length < 2)
            throw "A link needs at least 2 arguments";
        rBlock.attributes = oldBlock.attributes;
        // @ts-ignore
        rBlock.attributes["href"] = [c[0]];
        c.shift();
        rBlock.content = c.join(" ");
        return rBlock;
    }),
    new Preset_1.Preset("img", new BlockElement_1.BlockElement({
        tag: "img",
    }), function (rBlock, oldBlock) {
        var c = oldBlock.content.split(" ");
        rBlock.attributes = oldBlock.attributes;
        // @ts-ignore
        rBlock.attributes["src"] = [c[0]];
        if (c.length >= 2) {
            c.shift();
            // @ts-ignore
            rBlock.attributes["alt"] = [c.join(" ")];
        }
        return rBlock;
    }),
    new Preset_1.Preset("viewport", new BlockElement_1.BlockElement({
        tag: "meta",
        attributes: {
            name: "viewport",
            content: [
                "width=device-width,",
                "user-scalable=no,",
                "initial-scale=$size$,",
                "maximum-scale=$size$,",
                "minimum-scale=$size$",
            ]
        },
    }), function (rBlock, oldBlock) {
        if (oldBlock.content === "")
            oldBlock.content = "1.0";
        rBlock.attrReplace("content", "$size$", oldBlock.content);
        return rBlock;
    })
];
//# sourceMappingURL=Presets.js.map