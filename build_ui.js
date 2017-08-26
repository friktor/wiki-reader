const pug = require("pug")
const fs = require("fs")

let templates = ["app.pug", "pages.pug", "headerbar.pug", "home.pug"]
let options = {}

templates.forEach((template) => {
  let fn = pug.compileFile(`./resources/ui/${template}`, {
    basedir: __dirname+'/resources/ui',
    pretty: true
  })

  let outputName = template.replace('.pug', '.xml')

  fs.writeFileSync(
    `./resources/c_ui/${outputName}`,
    fn(options),
    { encoding: "utf-8" }
  )
})