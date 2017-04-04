const caseconv = require('./index.js');

process.argv.forEach((val, idx) => {
    console.log(val + " " + caseconv.unjumble(val, "kebab"));
});
