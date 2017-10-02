use components::headerbar::AppHeaderBar;
use components::sidebar::AppSidebar;
use utils::navigator::Navigator;
use utils::traits::Event;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;
use gdk::Screen;
use gtk;

pub struct Application {
  pub window: gtk::Window,
  settings: gtk::Settings,
  builder: gtk::Builder,

  navigator: Rc<RefCell<Navigator>>,
  headerbar: AppHeaderBar,
  sidebar: AppSidebar,
}

impl Application {
  pub fn new() -> Application {
    let builder = gtk::Builder::new_from_resource("/org/gtk/wikireader/ui/app.xml");
    let window: gtk::Window = builder.get_object("app_window").unwrap();
    let navigator = Rc::new(RefCell::new(Navigator::new()));
    let settings = gtk::Settings::get_default().unwrap();

    let headerbar = AppHeaderBar::new(navigator.borrow().get_events());
    headerbar.setup();

    let sidebar = AppSidebar::new(navigator.borrow().get_events());
    sidebar.setup();
    
    // final blocks
    Application {
      navigator,
      headerbar,
      sidebar,

      settings,
      builder,
      window
    }
  }

  fn setup_navigator(&self) {    
    let container: gtk::Box = self.builder.get_object("app_container").unwrap();
    let navigator = self.navigator.clone();
    let events = navigator.borrow().get_events();

    navigator.borrow().setup();

    // Sidebar prepares
    let sidebar = self.sidebar.container.clone();
    container.pack_start(&sidebar, false, true, 0);

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

  fn setup_headerbar(&self) {
    let headerbar = self.headerbar.headerbar.clone();
    self.window.set_titlebar(&headerbar);
  }

  fn setup_settings(&self) {
    self.settings.set_property_gtk_decoration_layout(Some("close")); // remove window controls
    self.settings.set_property_gtk_theme_name(Some("Arc-Dark"));
    self.settings.set_property_gtk_enable_animations(true);

    let screen = Screen::get_default().unwrap();
    let provider = gtk::CssProvider::new();
    provider.load_from_path("./bundles/main.css").unwrap();

    gtk::StyleContext::add_provider_for_screen(
      &screen, &provider,
      gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );
  }

  pub fn setup(&self) {
    self.setup_navigator();
    self.setup_headerbar();
    self.setup_settings();
  }
}