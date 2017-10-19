use utils::add_class_to_widget;
use layout::template::Template;
use layout::tags::apply_tags;
use serde_json::Value;
use gtk;

use gtk::{ TextView, Box, Button };
use gtk::TextBufferExt;
use gtk::TextViewExt;
use gtk::ButtonExt;
use gtk::BoxExt;

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
  pub layout: Box,
  tree: Value
}

impl Tree {
  pub fn new(tree: Value) -> Tree {
    Tree {
      layout: Box::new(gtk::Orientation::Vertical, 0),
      ranges: vec![],
      tree
    }
  }

  pub fn setup(&mut self, single_section: bool) {
    let textview = TextView::new();
    
    textview.set_wrap_mode(gtk::WrapMode::Word);
    add_class_to_widget(&textview, "article");
    textview.set_editable(false);

    if single_section {
      if let Some(section) = self.tree.clone().as_array() {
        self.render_section(&textview, section.clone());
      }
    } else {
      if let Some(sections) = self.tree.clone().as_array() {
        for section in sections {
          if let Some(section) = section.clone().as_array() {
            self.render_section(&textview, section.clone());
          }
        }
      }
    }

    let layout = get_styled_textview(textview, self.ranges.clone());
    self.layout.pack_start(&layout, false, true, 0);
  }

  fn get_tag_node(&mut self, textview: &TextView, node: &Value) -> TextView {
    let content_nodes = node["properties"].as_array().unwrap();
    let tag_name = node["closing_tag"].as_str().unwrap();
    let mut result_textview = textview.clone();

    let result_content = self.render_section(&result_textview, content_nodes.clone());
    result_textview = result_content;
    result_textview
  }

  // Return text nodes for insert and tag type
  fn get_text_node(&self, node: &Value) -> (String, String) {
    let node_type = node["type"].as_str().expect("Cannot get node type");
    
    let text_key = match node_type {
      "link" | "heading" => "title",
      "text" => "text",
      "wikilink" => {
        if node["text"].is_null() 
          { "title" } 
            else
          { "text" }
      },
      _ => ""
    };

    let mut tag = String::new();
    if node_type == "wikilink" || node_type == "link" {
      tag.push_str("link");
    } else if node_type == "heading" {
      let level = node["level"].as_i64().expect("Cant load heading level");
      tag = format!("heading{}", level);
    } else {
      tag.push_str(node_type.clone());
    }

    let _text = node[text_key].as_str().expect("Not found text field");
    let text = String::from(_text);

    (text, tag)
  }

  fn get_template_node(&mut self, template: &Value) -> (gtk::Box, bool) {
    let content_nodes = template["content"].as_array().unwrap();
    let params = template["params"].as_object().unwrap();
    let name = template["name"].as_str().unwrap();

    let mut content: Vec<Tree> = vec![];
    for node in content_nodes {
      let mut node = Tree::new(node.clone());
      node.setup(true);
      content.push(node);
    }

    let template = Template {
      layout: gtk::Box::new(gtk::Orientation::Vertical, 0),
      name: String::from(name),
      params: params.clone(),
      content
    };

    let is_inline = template.is_inline();
    template.setup();

    (template.layout, is_inline)
  }

  fn render_section(&mut self, textview: &TextView, section: Vec<Value>) -> TextView {
    let mut textview = textview.clone();

    fn get_anchor(b: &gtk::TextBuffer, i: &mut gtk::TextIter) -> gtk::TextChildAnchor {
      b.create_child_anchor(i).expect("failed create anchor")
    };
    
    for node in &section {
      let buffer = textview.get_buffer().expect("failed getting text buffer");
      let node_type = node["type"].as_str().unwrap();
      let mut end_iterator = buffer.get_end_iter();

      match node_type {
        "link" | "text" | "wikilink" | "heading" => {
          let (text, tag) = self.get_text_node(node);

          if tag == "link" {
            let button = Button::new_with_label(&*text);
            add_class_to_widget(&button, "link");

            button.connect_clicked(move |_| {
              println!("clicked to button");
            });

            let anchor = get_anchor(&buffer, &mut end_iterator);
            textview.add_child_at_anchor(&button, &anchor);
          } else {
            buffer.insert(&mut end_iterator, &*text);
            self.ranges.push((text, tag));
          }
        },
        
        "template" => {
          let (container, is_inline) = self.get_template_node(node);

          if is_inline {
            let anchor = get_anchor(&buffer, &mut end_iterator);
            textview.add_child_at_anchor(&container, &anchor);
          } else {
            
          }
        },

        "tag" => {
          let _textview = self.get_tag_node(&textview, node);
          textview = _textview;
        },
        
        _ => {}
      }      
    }

    textview
  }
}