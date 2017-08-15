const Wiki = require("wtf_wikipedia")
const fs = require("fs")

let wikicode = fs.readFileSync('./test.wikicode', { encoding: "utf-8" })
let parsed = Wiki.parse(wikicode)

fs.writeFileSync('test.json', JSON.stringify(parsed, null, 2), { encoding: "utf-8" })