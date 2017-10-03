use utils::add_class_to_widget;
use layout::tags::apply_tags;
use serde_json::Value;
use gtk;

use gtk::{ TextView, Box };
use gtk::TextBufferExt;
use gtk::TextViewExt;
use gtk::BoxExt;

fn create_textview() -> gtk::TextView {
  let textview = gtk::TextView::new();
  add_class_to_widget(&textview, "article");

  textview.set_wrap_mode(gtk::WrapMode::Word);
  textview.set_editable(false);

  textview
}

fn get_styled_textview(textview: TextView, ranges: Vec<(String, String)>) -> TextView {
  let buffer = textview.get_buffer().unwrap();
  let start_iter = buffer.get_start_iter();
  
  // adding styling tags to textview
  apply_tags(&buffer);
  
  // apply styles by tag to textview blocks by range 
  for range in &ranges {
    let text = &range.0;
    let tag = &range.1;

    if let Some(ranges) = start_iter.forward_search(&*text, gtk::TEXT_SEARCH_TEXT_ONLY, None) {
      let start_range = ranges.0;
      let end_range = ranges.1;

      buffer.apply_tag_by_name(&*tag, &start_range, &end_range);
    }
  }

  textview
}

pub struct Tree {
  ranges: Vec<(String, String)>,
  pub layout: gtk::Box,
  tree: Value
}

impl Tree {
  pub fn new(tree: Value) -> Tree {
    Tree {
      layout: gtk::Box::new(gtk::Orientation::Vertical, 0),
      ranges: vec![],
      tree
    }
  }

  pub fn setup(&mut self, single_section: bool) {
    if single_section {
      if let Some(section) = self.tree.clone().as_array() {
        let section_content = self.render_section(section.clone());
        self.layout.pack_start(&section_content, false, true, 0);
      }
    } else {
      if let Some(sections) = self.tree.clone().as_array() {
        for section in sections {
          if let Some(section) = section.clone().as_array() {
            let section_content = self.render_section(section.clone());
            self.layout.pack_start(&section_content, false, true, 0);
          }
        }
      }
    }
  }

  // @desc: after insertion text nodes of tag block - return next textview contexted block
  fn insert_tag_node(&mut self, tag: &Value, textview: TextView) -> (String, TextView) {
    let content_nodes = tag["content"].as_array();
    let mut textview = textview.clone();
    let mut tag_text = String::new();
      
    if let Some(nodes) = content_nodes {
      for node in nodes {
        let buffer = textview.get_buffer().unwrap();
        let end_iter = buffer.get_end_iter();

        if let Some(node_type) = node["type"].as_str() {
          if node_type != "template" {
            let (text, next_textview) = self.insert_text_node(node, textview.clone());
            tag_text.push_str(&*text);
            textview = next_textview;
          } else {
            // TODO: handle inline templates in tag
          }
        }
      }
    }

    if let Some(closing_tag) = tag["closing_tag"].as_str() {
      self.ranges.push((tag_text.clone(), String::from(closing_tag)));
    }
    
    (tag_text, textview)
  }

  // @desc: insertion text node, and after return next textview contexted block
  fn insert_text_node(&mut self, node: &Value, textview: TextView) -> (String, TextView) {
    let mut text_of_node = String::new();
    let mut textview = textview.clone();
    
    fn get_tag_name(node_type: &str) -> String {
      let mut name = String::from(node_type.clone());
      if node_type == "wikilink" || node_type == "link" {
        name = String::from("link");
      }

      name
    }

    if let Some(node_type) = node["type"].as_str() {
      let taggable_text = match node_type {
        "link" => node["title"].as_str(),
        "text" => node["text"].as_str(),
        "wikilink" => {
          let have_text = !node["text"].is_null();
          
          if have_text {
            node["text"].as_str()
          } else {
            node["title"].as_str()
          }
        },
        _ => None
      };

      let tag_name = get_tag_name(node_type);
      
      if let Some(taggable_text) = taggable_text {
        let buffer = textview.get_buffer().unwrap();
        let mut end_iter = buffer.get_end_iter();

        let text = String::from(taggable_text);
        buffer.insert(&mut end_iter, &*text);
        
        self.ranges.push((text.clone(), tag_name));
        text_of_node = text;
      } else {
        let (text, next_textview) = self.insert_tag_node(node, textview);
        textview = next_textview;
        text_of_node = text;
      }
    }

    (text_of_node, textview)
  }

  fn get_template_node(&mut self, template: &Value) -> gtk::Box {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let name = template["name"].as_str().unwrap();
    add_class_to_widget(&container, &*format!("{}-template", &name));

    if let Some(nodes) = template["content"].as_array() {
      for node in nodes {
        let mut node = Tree::new(node.clone());
        node.setup(true);

        container.pack_start(&node.layout, false, true, 0);
      }
    }

    container
  }

  fn render_section(&mut self, section: Vec<Value>) -> gtk::Box {
    let paragraph_container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let mut textview = create_textview();

    let pack_paragraph = |container: &Box, textview: TextView, ranges: Vec<(String, String)>| {
      let textview_rendered = get_styled_textview(textview, ranges);
      container.pack_start(&textview_rendered, false, true, 0);
      return create_textview();
    };

    for (index, node) in section.iter().enumerate() {
      let node_type = node["type"].as_str().unwrap();

      match node_type {
        "template" => {
          let ranges = self.ranges.clone();
          let _textview = textview.clone();
          
          textview = pack_paragraph(
            &paragraph_container,
            _textview, ranges
          );

          let template = self.get_template_node(node);
          paragraph_container.pack_start(&template, false, true, 0);
        },

        "heading" => {
          let ranges = self.ranges.clone();
          let _textview = textview.clone();
          
          textview = pack_paragraph(
            &paragraph_container,
            _textview, ranges
          );

          let title = node["title"].as_str().unwrap();
          let heading = gtk::Label::new(Some(title));
          add_class_to_widget(&heading, "header");

          paragraph_container.pack_start(&heading, false, true, 0);
        },

        _ => { // Basic Text nodes
          let end_of_node = index == section.len() - 1;
          let (_, next_textview) = self.insert_text_node(node, textview.clone());
          textview = next_textview;
          
          if end_of_node {
            let ranges = self.ranges.clone();
            let _textview = textview.clone();
            
            textview = pack_paragraph(
              &paragraph_container,
              _textview, ranges
            );
          }
        }
      }
    }

    paragraph_container
  }
}