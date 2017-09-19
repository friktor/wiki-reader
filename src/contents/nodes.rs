extern crate serde_json;
extern crate gtk;

use gtk::StyleContextExt;
use gtk::ContainerExt;
use gtk::WidgetExt;
use gtk::BoxExt;

use contents::templates::render_template;
use self::serde_json::Value;

pub fn render_section_node(section: Value) {
  let properties = section["properties"].as_array().unwrap();
  
  println!("SOURCE JSON: \n\n{}\n\n", &section);
  let _w = section["wikicode"].as_str().unwrap();
  let wikicode = String::from(_w);

  let mut templates_ranges: Vec<(usize, usize, &str)> = vec![];
  for property in properties {
    let prop_type = property["type"].as_str().unwrap();
    let prop_code = property["wikicode"].as_str().unwrap();
    let start_slice = wikicode.find(prop_code).unwrap();
    let code_lenght = prop_code.len();

    let result = (start_slice, start_slice+code_lenght, prop_type);
    templates_ranges.push(result);
  }

  templates_ranges.sort_by(|a, b| {
    let a_start = a.0;
    let b_start = b.0;

    a_start.cmp(&b_start)
  });

  let mut ranges: Vec<(usize, usize, &str)> = vec![];
  let mut starter_index: usize = 0;
  for range in &templates_ranges {
    let &(start, end, prop_type) = range;

    let result = (starter_index, start, "text");
    ranges.push(result);
    ranges.push(range.clone());
    starter_index = end+1;
  }

  ranges.retain(|&e| {
    let range_empty = (e.0 == e.1);
    if range_empty { return false; }

    let range_nil = (e.0 == (e.1 - 1));
    if range_nil { return false; }

    true
  });

  // for range in &ranges {
  //   let (start, end, _) = range.clone();

  //   println!("all len: {}, end of chunk: {}", &wikicode.len(), &end);
  //   let text = &wikicode[start..end];

  //   println!("({}, {}, \"{}\")\n{}\n\n", range.0, range.1, range.2, text);
  // }
}