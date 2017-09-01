const Wiki = require("wtf_wikipedia")
const atob = require("atob")
const fs = require("fs")

let content = JSON.parse(fs.readFileSync(process.argv.pop(), 'utf-8'));
let page_id = Object.keys(content.query.pages)[0];
let { title, ns, revisions } = content.query.pages[page_id];
let wiki_code = revisions["0"]["*"];

let result = {};

console.log(
  JSON.stringify(
    Wiki.parse(wiki_code),
    null, 2
  )
);