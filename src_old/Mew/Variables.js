"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Variables = void 0;
var Variables;
(function (Variables) {
    Variables.Data = {};
    var Checkers = [
        {
            callback: function (str) {
                for (var _i = 0, _a = Object.entries(Variables.Data); _i < _a.length; _i++) {
                    var _b = _a[_i], key = _b[0], value = _b[1];
                    var regex = new RegExp("{{([ ]+)?([" + key + "]+)([ ]+)?}}", "g");
                    str = str.replace(regex, value);
                }
                return str;
            }
        },
        {
            callback: function (str) {
                var regex = /\$([\w]+)[ ]*?([=])(.*)/;
                var m;
                while ((m = regex.exec(str)) !== null) {
                    if (m.index === regex.lastIndex)
                        regex.lastIndex++;
                    var vName = m[1];
                    var vValue = m[3].trim();
                    if (vValue.charAt(0) === '"')
                        vValue = vValue.substr(1, vValue.length - 2);
                    // @ts-ignore
                    Variables.Data[vName] = vValue;
                    str = "";
                }
                return str;
            }
        }
    ];
    Variables.check = function (str) {
        Checkers.forEach(function (checker) {
            str = checker.callback(str);
        });
        return str;
    };
})(Variables = exports.Variables || (exports.Variables = {}));
