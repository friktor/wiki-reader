use utils::add_class_to_widget;
use layout::template::Template;
use layout::tags::apply_tags;
use serde_json::Value;
use std::cell::RefCell;
use std::ops::FnMut;
use std::rc::Rc;
use gtk;

use gdk_pixbuf::{ Pixbuf, Colorspace };
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

// fn get_screenshot_button() -> Button {
//   let button = Button::new_with_label("Capture Screenshot");
//   button
// }

#[derive(Clone)]
pub enum ArticleTreeEvent {
  ExternalLink(String),
  WikiLink(String)
}

#[derive(Clone)]
pub struct Tree {
  pub events: Rc<RefCell<TreeEventEmitter>>,
  ranges: Vec<(String, String)>,
  textview: TextView,
  pub layout: Box,
  tree: Value
}

#[derive(Clone)]
pub struct TreeEventEmitter {
  listeners: Vec<Rc<RefCell<FnMut(ArticleTreeEvent)>>>
}

impl TreeEventEmitter {
  pub fn new() -> TreeEventEmitter {
    TreeEventEmitter { listeners: Vec::new() }
  }

  pub fn subscribe<F: FnMut(ArticleTreeEvent)+'static>(&mut self, listener: F) {
    let cell = Rc::new(RefCell::new(listener));
    self.listeners.push(cell); 
  }

  pub fn push(&mut self, event: ArticleTreeEvent) {
    for listener in self.listeners.iter() {
      let mut closure = listener.borrow_mut();
      let _e = event.clone();
      (&mut *closure)(_e);
    }
  }
}

impl Tree {
  pub fn new(tree: Value) -> Tree {
    Tree {
      events: Rc::new(RefCell::new(TreeEventEmitter::new())),
      layout: Box::new(gtk::Orientation::Vertical, 0),
      textview: TextView::new(),
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
        self.render_section(&textview, section.clone(), None);
      }
    } else {
      if let Some(sections) = self.tree.clone().as_array() {
        for section in sections {
          if let Some(section) = section.clone().as_array() {
            self.render_section(&textview, section.clone(), None);
          }
        }
      }
    }

    // let screenshot_button = get_screenshot_button();
    // self.layout.pack_start(&screenshot_button, false, true, 0);

    let layout = get_styled_textview(textview, self.ranges.clone());
    self.textview = layout;
    self.layout.pack_start(&self.textview, false, true, 0);
  }

  // Return text nodes for insert and tag type
  fn get_text_node(&self, node: &Value) -> (String, String) {
    let node_type = node["type"].as_str().expect("Cannot get node type");
    
    let text_key = match node_type {
      "link" => "title",
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

  fn get_tag_node(&mut self, textview: &TextView, node: &Value, tag_name: String) -> TextView {
    let content_nodes = node["properties"].as_array().unwrap();
    let mut result_textview = textview.clone();

    let result_content = self.render_section(&result_textview, content_nodes.clone(), Some(tag_name));
    result_textview = result_content;
    result_textview
  }

  fn get_link_button(&self, node_type: &str, text: String, link: &str) -> Button {
    let button = Button::new_with_label(&*text);
    add_class_to_widget(&button, "link");
    
    let _type = String::from(node_type);
    let link = String::from(link);

    let events = self.events.clone();
    button.connect_clicked(move |_| {
      let link_event = match &*_type {
        "wikilink" => ArticleTreeEvent::WikiLink(link.clone()),
        _ => ArticleTreeEvent::ExternalLink(link.clone())
      };
        
      events.borrow_mut().push(link_event);
    });

    button
  }

  fn render_section(
    &mut self, textview: &TextView,
    section: Vec<Value>,
    as_tag: Option<String>
  ) -> TextView {
    let mut textview = textview.clone();

    fn get_anchor(b: &gtk::TextBuffer, i: &mut gtk::TextIter) -> gtk::TextChildAnchor {
      b.create_child_anchor(i).expect("failed create anchor")
    };
    
    for node in &section {
      let buffer = textview.get_buffer().expect("failed getting text buffer");
      let node_type = node["type"].as_str().unwrap();
      let mut end_iterator = buffer.get_end_iter();

      match node_type.clone() {
        "link" | "text" | "wikilink" => {
          let (text, tag) = self.get_text_node(node);

          if tag == "link" {
            let link = match node_type {
              "wikilink" => node["title"].as_str().unwrap(),
              _ => node["url"].as_str().unwrap()
            };

            let button = self.get_link_button(node_type.clone(), text.clone(), link.clone());
            let anchor = get_anchor(&buffer, &mut end_iterator);

            if let Some(tag_name) = as_tag.clone() {
              add_class_to_widget(&button, &*tag_name);
            }
            
            textview.add_child_at_anchor(&button, &anchor);
          } else {
            buffer.insert(&mut end_iterator, &*text);
            self.ranges.push((text.clone(), tag));
            
            if let Some(tag_name) = as_tag.clone() {
              self.ranges.push((text, tag_name));
            }
          }
        },
        
        "template" => {
          let (container, is_inline) = self.get_template_node(node);

          let anchor = get_anchor(&buffer, &mut end_iterator);
          textview.add_child_at_anchor(&container, &anchor);
        },

        "tag" | "heading" => {
          let tag_name = match node_type {
            "tag" => {
              let closing_tag = node["closing_tag"].as_str().unwrap();
              String::from(closing_tag)
            },
            
            "heading" => {
              let level = node["level"].as_i64().expect("Cant load heading level");
              format!("heading{}", level)
            },

            _ => String::from("")
          };

          let _textview = self.get_tag_node(&textview, node, tag_name);
          textview = _textview;
        },
        
        _ => {}
      }      
    }

    textview
  }
}