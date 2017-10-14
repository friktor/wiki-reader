#!/usr/local/bin/python3
# -*- coding: utf-8 -*-

import format_ast
import json
import sys

# Get source wikicode from pipe
text = ""
for line in sys.stdin:
  text += line

formatted = format_ast.parse(text)
sys.stdout.write(json.dumps(formatted))