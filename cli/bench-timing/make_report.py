#!/usr/bin/env python3

import re

from pprint import pprint

print("running")

def runit(pattern):
    data = {}
    with open("/Users/alan/Documents/Neopoligen/log.log") as infile:
        lines = infile.readlines()
        for line in lines:
            matches = re.search(pattern, line)
            if matches:
                func = matches.group(2)
                value = matches.group(3)
                unit = matches.group(4)
                if func not in data:
                    data[func] = []
                data[func].append({ "unit": unit, "value": value})
    pprint(data)



if __name__ == "__main__":
    runit(re.compile(r'(INFO)\s[^:]+:(.*?)\{.*?\|\|([\d.]+)(.*)\|\|'))
