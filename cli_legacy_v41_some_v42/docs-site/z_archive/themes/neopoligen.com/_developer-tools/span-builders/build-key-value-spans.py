#!/usr/bin/env python3

# For now this just copies content from the
# custom-key-value-templates folder, splits
# it and then make the span and the docs
# page


import glob
import os
from shutil import copy2
from pathlib import Path 


theme_directory = "../../spans"
pages_directory = "../../../../../pages/spans/examples"
custom_directory = "custom-key-value-templates"
Path(pages_directory).mkdir(parents=True, exist_ok=True)

# load the spans

file_list = [
    file for file in glob.glob(f"{custom_directory}/*")
    if os.path.isfile(file)
]

for initial_path in file_list:
    path_parts = initial_path.split("/")
    name_parts = path_parts[1].split(".")
    theme_output_path = f"{theme_directory}/{path_parts[1]}"
    pages_output_path = f"{pages_directory}/{name_parts[0]}.neo"
    with open(initial_path) as _file:
        initial_contents = _file.read()
        content_parts = initial_contents.split("~~~~~~~~~~")

        with open(theme_output_path, "w") as _theme_out:
            _theme_out.write(content_parts[0].strip())

        with open(pages_output_path, "w") as _theme_out:
            _theme_out.write(content_parts[1].strip())

