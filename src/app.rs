use components::headerbar::AppHeaderBar;
use utils::navigator::Navigator;
use utils::get_resources_path;
use utils::get_i18n_locale;
use std::cell::RefCell;
use std::rc::Rc;

use fluent::MessageContext;
use fluent_locale::Locale;
use gtk::prelude::*;
use gdk::Screen;
use gtk;

pub struct Application<'a> {
  i18n: Rc<RefCell<MessageContext<'a>>>,
  navigator: Rc<RefCell<Navigator>>,
  locale: Rc<RefCell<Locale>>,
  pub window: gtk::Window,
  settings: gtk::Settings,
  builder: gtk::Builder,
}

impl <'a>Application<'a> {
  pub fn new() -> Application<'a> {
    let builder = gtk::Builder::new_from_resource("/org/gtk/wikireader/ui/app.xml");
    let window: gtk::Window = builder.get_object("app_window").unwrap();

    let (locale, i18n_text) = get_i18n_locale();
    
    let i18n = Rc::new(RefCell::new(MessageContext::new(&[ "i18n" ])));
    let locale = Rc::new(RefCell::new(locale));
    i18n.borrow_mut().add_messages(&*i18n_text);

    let navigator = Rc::new(RefCell::new(Navigator::new()));
    let settings = gtk::Settings::get_default().unwrap();

    let events = navigator.borrow().get_events();
    let headerbar = AppHeaderBar::new(events, i18n.clone());

    headerbar.setup();
    window.set_titlebar(&headerbar.headerbar);

    // final blocks
    Application {
      navigator,
      settings,
      builder,
      locale,
      window,
      i18n
    }
  }

  fn setup_navigator(&self) {    
    let container: gtk::Box = self.builder.get_object("app_container").unwrap();
    let navigator = self.navigator.clone();
    let events = navigator.borrow().get_events();

    navigator.borrow().setup(self.i18n.clone());

    // Pages content prepare
    let stack = navigator.borrow().stack.clone();
    container.pack_end(&stack, true, true, 0);

    // Global Listeners
    events.borrow_mut().subscribe(move |event| {
      match event {
        _ => {}
      }
    });
  }

  fn setup_settings(&self) {
    self.settings.set_property_gtk_theme_name(Some("Arc-Darker"));
    self.settings.set_property_gtk_enable_animations(true);

    let screen = Screen::get_default().unwrap();
    let provider = gtk::CssProvider::new();

    let css_path = format!("{}/main.css", get_resources_path());
    provider.load_from_path(&*css_path).unwrap();

    gtk::StyleContext::add_provider_for_screen(
      &screen, &provider,
      gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );
  }

  pub fn setup(&self) {
    self.setup_navigator();
    self.setup_settings();
  }
}