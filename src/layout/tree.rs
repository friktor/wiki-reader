use utils::add_class_to_widget;
use layout::tags::apply_tags;
use serde_json::Value;
use gtk;

use gtk::TextBufferExt;
use gtk::TextViewExt;
use gtk::BoxExt;

pub trait Node {
  fn get_content(&self) -> gtk::Box;
  fn get_type(&self) -> String; // Text, Template
  fn setup(&self);
}

fn create_textview() -> gtk::TextView {
  let textview = gtk::TextView::new();
  add_class_to_widget(&textview, "article");

  textview.set_wrap_mode(gtk::WrapMode::Word);
  textview.set_editable(false);

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

  pub fn setup(&mut self) {
    if let Some(sections) = self.tree.clone().as_array() {
      for section in sections {
        if let Some(section) = section.as_array() {
          let section_content = self.render_section(section.clone());
          self.layout.pack_start(&section_content, false, true, 0);
        }
      }
    }
  }

  fn insert_tag_node(&mut self, chunk: &Value, textview: gtk::TextView) {
    let buffer = textview.get_buffer().unwrap();
    let mut end_iter = buffer.get_end_iter();
    let mut tag_text = String::new();
    
    if let Some(nodes) = chunk["content"].as_array() {
      for chunk in nodes {
        let node_type = chunk["type"].as_str().unwrap();

        if node_type != "template" {

        }
      }
    }
  }

  fn insert_text_nodes(&mut self, chunk: &Value, textview: &mut gtk::TextView) {
    let node_type = chunk["type"].as_str().unwrap();
    let buffer = textview.get_buffer().unwrap();
    let mut end_iter = buffer.get_end_iter();

    let text = match node_type {
      "wikilink" => chunk["title"].as_str(),
      "heading" => chunk["title"].as_str(),
      "link" => chunk["title"].as_str(),
      "text" => chunk["text"].as_str(),
      "filelink" => Some(""),
      _ => Some("")
    };

    match node_type {
      "tag" => self.insert_tag_node(&chunk, textview.clone()),

      _ => {
        let text = text.unwrap();
        let mut tag_name = String::from(node_type.clone());
        if node_type == "wikilink" || node_type == "link" {
          tag_name = String::from("link");
        }

        buffer.insert(&mut end_iter, &text);
        
        self.ranges.push((
          String::from(text),
          tag_name
        ));
      }
    }
  }

  fn get_text_node(&mut self, textview: &mut gtk::TextView) -> gtk::TextView {
    let node = textview.clone();
    let buffer = node.get_buffer().unwrap();
    let start_iter = buffer.get_start_iter();

    apply_tags(&buffer);

    for range in &self.ranges {
      let text = &range.0;
      let tag = &range.1;

      if let Some(ranges) = start_iter.forward_search(&*text, gtk::TEXT_SEARCH_TEXT_ONLY, None) {
        let start_range = ranges.0;
        let end_range = ranges.1;

        buffer.apply_tag_by_name(&*tag, &start_range, &end_range);
      }
    }

    *textview = create_textview();
    node
  }

  fn get_template_node(&mut self, scheme: &Value) -> gtk::Box {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

    container
  }

  fn render_section(&mut self, section: Vec<Value>) -> gtk::Box {
    let paragraph_container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    
    for paragraph in section {
      if let Some(paragraph) = paragraph.as_array() {
        let mut textview = create_textview();

        for (index, chunk) in paragraph.iter().enumerate() {
          let node_type = chunk["type"].as_str().unwrap();

          if node_type != "template" {
            self.insert_text_nodes(chunk, &mut textview);
            
            if index == paragraph.len() - 1 {
              let text_node = self.get_text_node(&mut textview);
              paragraph_container.pack_start(&text_node, false, true, 0);
            }
          } else {
            let text_node = self.get_text_node(&mut textview);
            paragraph_container.pack_start(&text_node, false, true, 0);

            let template = self.get_template_node(chunk);
            paragraph_container.pack_start(&template, false, true, 0);
          }
        }
      }
    }

    paragraph_container
  }
}