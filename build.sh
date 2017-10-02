sassc --sass resources/styles/main.sass > bundles/main.css
pug resources/views -o resources/ui -E xml -P
glib-compile-resources --target=bundles/bundle.gresource --sourcedir=resources/ resources/bundle.xml