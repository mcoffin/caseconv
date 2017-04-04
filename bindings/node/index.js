const nativeModule = require('./build/Release/node-caseconv');

function caseForName(caseName) {
    if ((/^camel$/i).test(caseName)) {
        return 0;
    }
    if ((/^snake$/i).test(caseName)) {
        return 1;
    }
    if ((/^kebab$/i).test(caseName)) {
        return 2;
    }
    throw new Error("invalid case name: " + caseName);
}

exports.unjumble = function unjumble(src, caseName) {
    const c = caseForName(caseName);
    return nativeModule.unjumble(src, c);
};
