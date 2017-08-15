sassc --sass resources/styles/main.sass > bundles/main.css
node build_ui.js
glib-compile-resources --target=bundles/bundle.gresource --sourcedir=resources/ resources/bundle.xml