sassc --sass resources/styles/main.sass > assets/main.css
pug resources/views -o resources/ui -E xml -P
glib-compile-resources --target=assets/assets.gresource --sourcedir=resources/ resources/bundle.xml