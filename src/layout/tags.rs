use std::str::FromStr;
use serde_json::Value;
use pango;
use gdk;
use gtk;

use gtk::TextTagTableExt;
use gtk::TextBufferExt;
use gtk::TextTagExt;

// TODO: creating macros for fast creating tags with properties
// text_tag!("name_tag", { background: "#FFF", color: "#EEE", weight: 500, size: 20 })

fn link_tag() -> gtk::TextTag {
  let tag = gtk::TextTag::new(Some("link"));
  tag.set_property_underline(pango::Underline::Single);
  let color = gdk::RGBA::from_str("#2196F3").unwrap();
  tag.set_property_foreground_rgba(Some(&color));
  tag
}

fn heading_tag(level: i32) -> gtk::TextTag {
  let name = format!("heading{}", &level);
  let tag = gtk::TextTag::new(Some(&*name));

  let color = gdk::RGBA::from_str("#263238").unwrap();
  tag.set_property_foreground_rgba(Some(&color));  
  
  match level {
    1 => tag.set_property_size_points(25.0),
    2 => tag.set_property_size_points(22.0),
    3 => tag.set_property_size_points(19.0),
    4 => tag.set_property_size_points(16.0),
    5 => tag.set_property_size_points(14.0),
    6 => tag.set_property_size_points(12.0),
    _ => {}
  }

  tag
}

fn text_tag() -> gtk::TextTag {
  let tag = gtk::TextTag::new(Some("text"));
  tag
}

fn bold_tag() -> gtk::TextTag {
  let tag = gtk::TextTag::new(Some("b"));
  tag.set_property_weight(600);
  tag
}

fn list_tag() -> gtk::TextTag {
  let tag = gtk::TextTag::new(Some("li"));
  tag
}

fn del_tag() -> gtk::TextTag {
  let tag = gtk::TextTag::new(Some("del"));
  tag
}

fn ref_tag() -> gtk::TextTag {
  let tag = gtk::TextTag::new(Some("ref"));
  tag
}

fn references_tag() -> gtk::TextTag {
  let tag = gtk::TextTag::new(Some("references"));
  tag
}

fn sub_tag() -> gtk::TextTag {
  let tag = gtk::TextTag::new(Some("sub"));
  tag
}

fn italic_tag() -> gtk::TextTag {
  let tag = gtk::TextTag::new(Some("i"));
  tag.set_property_style(pango::Style::Italic);
  tag
}

pub fn apply_tags(buffer: &gtk::TextBuffer) {
  if let Some(table) = buffer.get_tag_table() {
    let link = link_tag();
    table.add(&link);

    (1..6).for_each(|level| {
      let heading = heading_tag(level);
      table.add(&heading);
    });

    let text = text_tag();
    table.add(&text);

    let bold = bold_tag();
    table.add(&bold);

    let list = list_tag();
    table.add(&list);

    let del = del_tag();
    table.add(&del);

    let ref_t = ref_tag();
    table.add(&ref_t);

    let sub = sub_tag();
    table.add(&sub);

    let italic = italic_tag();
    table.add(&italic);

    let references = references_tag();
    table.add(&references);
  }
}
