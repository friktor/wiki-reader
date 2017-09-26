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

fn heading_tag() -> gtk::TextTag {
  let tag = gtk::TextTag::new(Some("heading"));
  tag.set_property_weight(600);
  tag.set_property_size(25);
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

pub fn apply_tags(buffer: &gtk::TextBuffer) {
  if let Some(table) = buffer.get_tag_table() {
    let link = link_tag();
    table.add(&link);

    let heading = heading_tag();
    table.add(&heading);

    let text = text_tag();
    table.add(&text);

    let bold = bold_tag();
    table.add(&bold);
  }
}
