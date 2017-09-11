#!/usr/local/bin/python3
# -*- coding: utf-8 -*-

import mwparserfromhell as parser
import string
import random
import copy
import json
import sys
import re

def id_generator(size=6, chars=string.ascii_uppercase + string.digits):
  return ''.join(random.choice(chars) for _ in range(size))

def get_formatted_links(link):
  return {
    "title": str(link.title),
    "wikicode": str(link),
    "url": str(link.url),
    "type": "link",
  }

def get_formatted_wikilinks(link):
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

def get_formatted_heading(heading):
  return {
    "title": str(heading.title),
    "level": heading.level,
    "type": "heading",
  }

def get_formatted_tags(tag):
  result = {
    "attributes": tag.attributes,
    "wikicode": str(tag),
    "type": "tag"
  }

  if not tag.contents:
    result["content"] = str(tag)
  else:
    result["content"] = get_formatted_block(tag.contents)
  
  return result

def get_formatted_template(template):
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
        "properties": get_formatted_block(param.value),
        "wikicode": str(param)
      })
  
  return result

def get_formatted_block(block):
  templates = block.filter_templates(recursive=False)
  links = block.filter_external_links(recursive=False)
  wikilinks = block.filter_wikilinks(recursive=False)
  headings = block.filter_headings(recursive=False)
  tags = block.filter_tags(recursive=False)

  _wikilinks = [get_formatted_wikilinks(link) for link in wikilinks]
  _templates = [get_formatted_template(tmpl) for tmpl in templates]
  _headings = [get_formatted_heading(h) for h in headings]
  _links = [get_formatted_links(link) for link in links]
  _tags = [get_formatted_tags(tag) for tag in tags]

  return _wikilinks + _templates + _headings + _links + _tags

def get_formatted(block):
  return {
    "properties": get_formatted_block(block),
    "wikicode": str(block),
    "type": "section"
  }

def parse(text):
  sections = parser.parse(text).get_sections()
  formatted = [get_formatted(section) for section in sections]
  return formatted

text = ""
for line in sys.stdin:
  text += line

result_blocked = parse(text)
# file = open("result.json", "w", encoding="utf-8")
sys.stdout.write(json.dumps(result_blocked))