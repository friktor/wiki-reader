sassc --sass resources/styles/main.sass > assets/main.css
pug resources/views -o resources/ui -E xml -P
rm -rf assets/i18n && cp -rf resources/i18n assets/i18n
glib-compile-resources --target=assets/assets.gresource --sourcedir=resources/ resources/bundle.xml