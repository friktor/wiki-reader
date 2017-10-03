from operator import itemgetter
import re

# Generate inline ast rows by ranges and parapraphs
# Level-1: Array<(Section)>
# Level-2:   Array<(Paragraph)>
# Level-3:     Array<(Node)>
# Level-n:       Node<(keys: {
#                  type: text|template|link|wikilink|heading
#                })>
# ___________________________________________________________

def props_ranges(properties, source):
  ranges = []

  for property in properties:
    prop_code = property["wikicode"]
    prop_type = property["type"]
    
    s = source.find(prop_code)
    e = s + len(prop_code)

    if prop_type == "template":
      ranged_content = []
      
      for node in property["content"]:
        node_section = inline_section(node)
        ranged_content.append(node_section)
      
      property["content"] = ranged_content
    
    if prop_type == "tag":
      node_section = inline_section(property)
      property["content"] = node_section

    ranges.append((s, e, property))
  
  return sorted(ranges, key=itemgetter(0))

def plain_ranges(template_ranges, source):
  position = 0
  ranges = []

  for index, range in enumerate(template_ranges):
    last = index is (len(template_ranges) - 1)
    template_start = range[0]
    template_end = range[1]

    text = source[position:template_start]
    ranges.append((position, template_start, {
      "type": "text",
      "text": text
    }))
    
    last_end = len(source) - 1
    last_start = template_end
    
    if last and last_end is not last_end:
      last_text = source[last_start: last_end]
      ranges.append(last_start, last_end, last_text)
    
    position = template_end
  
  return ranges

def extract_content_node(tuple_range):
  return tuple_range[2]

def inline_section(section):
  properties = section["properties"]
  wikicode = section["wikicode"]

  if len(properties) is 0:
    node = (0, 0, [{ "type": "text", "text": wikicode }])
    return extract_content_node(node)

  p_ranges = props_ranges(properties, wikicode)
  t_ranges = plain_ranges(p_ranges, wikicode)
  ranges = p_ranges + t_ranges

  ranged_section = sorted(ranges, key=itemgetter(0))
  nodes = [extract_content_node(n) for n in ranged_section]
  return nodes

def clean_section(section):
  def clean_heading(heading_text):
    for node in section:
      if node["type"] == "text":
        start = node["text"].find(heading_text)
        if start != -1:
          node["text"] = node["text"].replace(heading_text, '')
          break

  def filter_node(node):
    if node["type"] == "tag" and node["closing_tag"] == "li":
      return False

    if node["type"] == "text":
      text = node["text"]
      return len(text) > 0 and text != "\n"

    if node["type"] == "heading":
      clean_heading(node["wikicode"])
      node["title"] = node["title"].strip()

    if node["type"] == "tag":
      node["content"] = clean_section(node["content"])

    if node["type"] == "template":
      for index, t_section in enumerate(node["content"]):
        node["content"][index] = clean_section(t_section)

    if node["type"] in ["template", "heading"]:
      del node["wikicode"]

    return True

  def striphtml(data):
    p = re.compile(r'<.*?>')
    return p.sub('', data)

  result = [node for node in section if filter_node(node)]

  current_clean_index = 0
  for node in result:
    if node["type"] == "text":
      node["text"] = striphtml(node["text"])

    if node["type"] == "tag" and node["closing_tag"] in ['b', 'i']:
      closing_markup = node["closing_wiki_markup"][0]

      for ni in [current_clean_index-1, current_clean_index+1]:
        if len(result) != ni:
          if result[ni]["type"] == "text" and result[ni]["text"].find(closing_markup) != -1:
            result[ni]["text"] = result[ni]["text"].replace(closing_markup, '')
    
    current_clean_index += 1

  return result

def parse(ast):
  sections = [inline_section(section) for section in ast]
  cleaned = [clean_section(section) for section in sections]
  return cleaned