# TODO
- Adding i18n to project
- Adding progressive searched home page
  - popover selection of target resources
  - autosuggest for search input
  - info blocks for home

## API Examples

#### Request content page by title
http://lurkmore.to/api.php
  ?action=query
  &titles=Вещества
  &prop=revisions
  &rvprop=content
  &format=json

#### Parse Content
http://lurkmore.to/api.php
  ?action=parse
  &format=json
  &page=Вещества
  &utf8=1

#### Image info
http://lurkmore.to/api.php
  ?action=query
  &titles=File:Ment%202.jpg
  &prop=imageinfo
  &format=json
  &iiprop=timestamp|user|url


# Next features
- Writing new styles for native view in mac os & gnome
- Adding hotreloading styles handler for debug mode
- Button with selection previous articles
- Creating named article bookmarks group
  - adding saving tagged selection in articles