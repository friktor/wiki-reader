from operator import itemgetter

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

def split_by_paragraph(section_nodes):
  parapraphs = [ [] ]
  position = 0

  for node in section_nodes:
    node_type = node["type"]

    if node_type == "text":
      content = node["text"]
      if content != "":
        if content == "\n\n":
          position += 1
          parapraphs.append([])
        else:
          parapraphs[position].append(node)    
    else:
      parapraphs[position].append(node)
  
  return parapraphs

def extract_content_node(tuple_range):
  return tuple_range[2]

def inline_section(section):
  properties = section["properties"]
  wikicode = section["wikicode"]

  if len(properties) is 0:
    node = (0, 0, { "type": "text", "text": wikicode })
    return extract_content_node(node)

  p_ranges = props_ranges(properties, wikicode)
  t_ranges = plain_ranges(p_ranges, wikicode)
  ranges = p_ranges + t_ranges

  ranged_section = sorted(ranges, key=itemgetter(0))
  nodes = [extract_content_node(n) for n in ranged_section]
  splitted = split_by_paragraph(nodes)

  return splitted

def parse(ast):
  sections = [inline_section(section) for section in ast]
  return sections