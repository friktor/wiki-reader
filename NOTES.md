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