const slm = require("slm").__express
const fs = require("fs")

let templates = ["app.slm", "pages.slm"]
let options = {}

templates.forEach((template) => {
  slm(`./resources/ui/${template}`, options, (error, rendered) => {
    if (error) {
      console.error(error)
    }

    fs.writeFile(`./resources/c_ui/${template.replace('.slm', '.xml')}`, rendered, {
      encoding: 'utf-8'
    }, (error, status) => {
      if (error) {
        console.error(error)
      }
    })
  })
})