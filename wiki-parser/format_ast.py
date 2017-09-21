import mwparserfromhell as parser
import re

def w_link(link):
  return {
    "title": str(link.title),
    "wikicode": str(link),
    "url": str(link.url),
    "type": "link",
  }

def w_wikilink(link):
  result = {
    "title": str(link.title),
    "wikicode": str(link)
  }

  matching_file = re.compile("$(File|Файл):", re.IGNORECASE)
  if re.fullmatch(matching_file, result["title"]):
    result["type"] = "filelink"
  else:
    result["type"] = "wikilink"

  if (link.text):
    result["text"] = str(link.text)
  return result

def w_heading(heading):
  return {
    "title": str(heading.title),
    "wikicode": str(heading),
    "level": heading.level,
    "type": "heading",
  }

def w_tag(tag):
  result = {
    "attributes": tag.attributes,
    "wikicode": str(tag),
    "type": "tag"
  }

  if not tag.contents:
    result["content"] = str(tag)
  else:
    result["content"] = block(tag.contents)
  
  return result

def w_template(template):
  params = template.params
  name = str(template.name)
  
  result = {
    "wikicode": str(template),
    "type": "template",
    "content": [],
    "params": {},
    "name": name,
  }

  for param in params:
    if param.showkey:
      result["params"][str(param.name)] = str(param.value)
    else:
      result["content"].append({
        "properties": block(param.value),
        "wikicode": str(param)
      })
  
  return result

def block(section):
  links = section.filter_external_links(recursive=False)
  templates = section.filter_templates(recursive=False)
  wikilinks = section.filter_wikilinks(recursive=False)
  headings = section.filter_headings(recursive=False)
  tags = section.filter_tags(recursive=False)

  _wikilinks = [w_wikilink(link) for link in wikilinks]
  _templates = [w_template(tmpl) for tmpl in templates]
  _headings = [w_heading(h) for h in headings]
  _links = [w_link(link) for link in links]
  _tags = [w_tag(tag) for tag in tags]

  return _wikilinks + _templates + _headings + _links + _tags


def formating(section):
  return {
    "properties": block(section),
    "wikicode": str(section),
    "type": "section"
  }

def parse(wikicode):
  sections = parser.parse(wikicode).get_sections()
  formatted = [formating(section) for section in sections]
  return formatted