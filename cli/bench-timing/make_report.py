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
    make_report(data)

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
    #pprint(data)
    # pprint(units)

def make_report(data):
    stuff = {}
    for key in data:
        if key not in stuff:
            stuff[key] = {
                "min": 100000000000000000000,
                "max": 0,
            }
        for value in data[key]:
            if stuff[key]["min"] > value["adjusted"]:
                stuff[key]["min"] = value["adjusted"]
            if stuff[key]["max"] < value["adjusted"]:
                stuff[key]["max"] = value["adjusted"]

    for key in data:
        print(f"{key} - {stuff[key]['min']} - {stuff[key]['max']}")




if __name__ == "__main__":
    runit(re.compile(r'(INFO)\s[^:]+:(.*?)\{.*?\|\|([\d.]+)(.*)\|\|'))
