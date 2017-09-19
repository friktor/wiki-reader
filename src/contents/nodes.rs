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
  
  let _w = section["wikicode"].as_str().unwrap();
  let wikicode = String::from(_w);

  let mut templates_ranges: Vec<(usize, usize, &str, &str)> = vec![];
  for property in properties {
    let prop_type = property["type"].as_str().unwrap();
    let prop_code = property["wikicode"].as_str().unwrap();
    
    let start = wikicode.find(prop_code).unwrap();
    let end = prop_code.len() + start;

    let result = (start, end, prop_type, prop_code);
    templates_ranges.push(result);
  }

  templates_ranges.sort_by(|a, b| {
    let a_start = a.0;
    let b_start = b.0;

    a_start.cmp(&b_start)
  });

  // for range in &templates_ranges {
  //   println!("({}, {}, \"{}\")\n", range.0, range.1, range.2);
  // }

  // let mut text_ranges: Vec<(usize, usize, &str, &str)> = vec![];
  // let mut skiped: usize = 0;

  // for range in &templates_ranges {
  //   let text: String = wikicode.chars().skip(skiped).take((range.0 - 1)).collect();
  //   println!("(skiped: {}, taked: {})\n{}\n\n", skiped, range.0, text);
  //   skiped = (range.0 - 1);
  // }
}