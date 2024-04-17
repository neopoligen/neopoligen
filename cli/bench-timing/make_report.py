#!/usr/bin/env python3

import re

print("running")

pattern = re.compile('(INFO)\s[^:]+:(.*?)\{.*?\|\|([\d.]+)(.*)\|\|')

with open("/Users/alan/Documents/Neopoligen/log.log") as _in:
    lines = _in.readlines()
    for line in lines:
        matches = re.search(pattern, line)
        if matches:
            print(f"{matches.group(2)} - {matches.group(3)} - {matches.group(4)}")



