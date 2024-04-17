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
    make_times(data)

def make_times(data):
    units = {
            'ns': 1,
            'µs': 1000,
            'ms': 1000000
        }
    for key in data:
        for sample in data[key]:
            #if sample['unit'] not in units:
            #    units[sample['unit']] = 0
            sample['raw'] = float(sample['value']) 
            sample['adjusted'] = sample['raw'] * units[sample['unit']]
    pprint(data)
    # pprint(units)


if __name__ == "__main__":
    runit(re.compile(r'(INFO)\s[^:]+:(.*?)\{.*?\|\|([\d.]+)(.*)\|\|'))
