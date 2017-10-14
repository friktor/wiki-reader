import mwparserfromhell as parser
import re

ExternalLink = parser.nodes.external_link.ExternalLink
Argument = parser.nodes.argument.Argument
Template = parser.nodes.template.Template
Wikilink = parser.nodes.wikilink.Wikilink
Comment = parser.nodes.comment.Comment
Heading = parser.nodes.heading.Heading
Text = parser.nodes.text.Text
Tag = parser.nodes.tag.Tag
Node = parser.nodes.Node

def get_heading(heading):
  return {
    "title": str(heading.title),
    "level": heading.level,
    "type": "heading",
  }

def get_template(template):
  params = template.params
  name = str(template.name)
  
  result = {
    "type": "template",
    "content": [],
    "params": {},
    "name": name,
  }

  for param in params:
    if param.showkey:
      result["params"][str(param.name)] = str(param.value)
    else:
      result["content"].append(normalize_section(param.value.nodes))
  
  return result

def get_tag(tag):
  result = {
    "closing_tag": str(tag.closing_tag),
    "self_closing": tag.self_closing,
    "attributes": tag.attributes,
    "implicit": tag.implicit,
    "type": "tag"
  }

  if tag.closing_wiki_markup:
    result["closing_wiki_markup"] = str(tag.closing_wiki_markup),

  if tag.contents:
    result["properties"] = normalize_section(tag.contents.nodes)
  else:
    result["properties"] = []
  return result

def get_wikilink(link):
  result = {
    "title": str(link.title)
  }

  matching_file = re.compile("^(File|Файл):.*$")
  if matching_file.fullmatch(result["title"]):
    result["type"] = "filelink"
  else:
    result["type"] = "wikilink"
    if link.text:
      result["text"] = str(link.text)

  if (link.text):
    result["text"] = str(link.text)
  return result

def get_external_link(link):
  return {
    "title": str(link.title),
    "url": str(link.url),
    "type": "link",
  }


def normalize_section(nodes):
  result = []

  for index, node in enumerate(nodes):
    node_type = type(node)

    if node_type == Text:
      result.append({
        "text": node.value,
        "type": "text"
      })

    elif node_type == Heading:
      heading = get_heading(node)
      result.append(heading)
    
    elif node_type == Template:
      template = get_template(node)
      result.append(template)
    
    elif node_type == Tag:
      tag = get_tag(node)
      result.append(tag)
    
    elif node_type == Wikilink:
      wikilink = get_wikilink(node)
      result.append(wikilink)
    
    elif node_type == ExternalLink:
      external_link = get_external_link(node)
      result.append(external_link)

  return result

def parse(wikicode):
  sections = parser.parse(wikicode).get_sections()
  result_nodes = [ normalize_section(section.nodes) for section in sections ]
  return result_nodes