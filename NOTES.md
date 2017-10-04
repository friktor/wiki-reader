## API Examples
http://lurkmore.to/api.php
  ?action=query
  &titles=File:Ment%202.jpg
  &prop=imageinfo
  &format=json
  &iiprop=timestamp|user|url

## Building
Debug runing app
```
cargo run --features="debug"
```

Release build assets, and bundle package
```
./build.sh && cargo bundle -r
```

Building only assets
```
./build.sh
```

## Next features
- Writing new styles for native view in mac os & gnome
- Adding hotreloading styles handler for debug mode
- Button with selection previous articles
- Creating named article bookmarks group
  - adding saving tagged selection in articles
- Adding about page
- Adding debug page for live render wiki-code 
- Add handlers for menu buttons
- Packing application: brew, deb, rpm, and if possible - mac standalone app
- Adding i18n to project
- Adding progressive searched home page
  - popover selection of target resources
  - autosuggest for search input
  - info blocks for home