#!/usr/bin/env python3

# NOTE: The entire "spans" directory is made
# from this file. No changes should be made
# in it directly

# This is my python script for making all the
# standards (i.e. not key/value) span files for
# the theme. It creates basic ones for span type
# html Span. It also creates all the custom
# spans that are part of the theme (e.g. internal
# `ilink`` spans).
#
# The script also produces the documentation
# content pages for the site. I'm hard coding the
# page IDs into the script. Without that, the page
# IDs would move every time the script was run
#
# The files in `custom-standard-templates`` are for
# spans that have varying requirements (either for
# definition or the example usage). The files in that
# folder contain the content for both the span template
# and the # example .neo file. They are split on a
# `~~~~~~~~~~`` string and output to their respective
# locations


import glob
import os
from shutil import copy2
from pathlib import Path 


theme_directory = "../../spans"
pages_directory = "../../../../../pages/spans/examples"
custom_directory = "custom-standard-templates"
internal_directory = "internal-spans"

Path(pages_directory).mkdir(parents=True, exist_ok=True)


spans = [
    ("abbr", "The Abbreviation Span", "2bqpxumq"),
    ("b", "The Bring Attention To Span", "2bqparow"),
    ("bdi", "The Bidirectional Isolate Span", "2bqpbnoc"),
    ("bdo", "The Bidirectional Text Override Span", "2bql3gdk"),
    ("button", "The Button Span", "2bqpd2kj"),
    ("canvas", "The Graphics Canvas Span", "2bqpdzlm"),
    ("cite", "The Citation Span", "2bqpgehv"),
    ("data", "The Data Span", "2bqpgsdh"),
    ("del", "The Deleted Text Span", "2bqphhlz"),
    ("dfn", "The Definition Span", "2bqpiljy"),
    ("em", "The Emphasis Span", "2bqpinq8"),
    ("embed", "The Embed External Content Span", "2bqpinq8"),
    ("i", "The Idiomatic Text Span", "2bqpjg4i"),
    ("iframe", "The Inline Frame Span", "2bqpjb3e"),
    ("input", "The Input (Form Input) Span", "2bqpjsut"),
    ("ins", "The Inserted Text Span", "2bqpk8yk"),
    ("kbd", "The Keyboard Input Span", "2bqpkqmg"),
    ("mark", "The Mark Text Span", "2bqpkczs"),
    ("meter", "The HTML Meter Span", "2bqpkwr9"),
    ("output", "The Output Span", "2bqplnvt"),
    ("pre", "The Preformatted Text Span", "2bqpmstw"),
    ("progress", "The Progress Indicator Span", "2bqpmjjr"),
    ("q", "The Inline Quotation Span", "2bqpn1xm"),
    ("s", "The Strikethrough Span", "2bqpnlnq"),
    ("samp", "The Sample Output Span", "2bqpnpva"),
    ("script", "The Script Span", "2bqpne7p"),
    ("small", "the side comment Span", "2bqpo6vt"),
    ("span", "The Content Span Span", "2bqpoiv3"),
    ("strong", "The Strong Importance Span", "2bqpozfl"),
    ("sub", "The Subscript Span", "2bqpoojf"),
    ("sup", "The Superscript Span", "2bqppepu"),
    ("time", "The (Date) Time Span", "2bqppzjo"),
    ("u", "The Unarticulated Annotation (Underline) Span", "2bqpqgqa"),
    ("var", "The Variable Span", "2bqpqxec"),
]


jinja_template = """{# type: standard #}
{%- import "includes/macros.jinja" as macros -%}

<SPANKEY {{- macros.span_attributes(page_id, span.content) -}}>
{% for span in span.content.spans %}
{{- macros.output_spans(page_id, span, {}) -}}
{% endfor %}
</SPANKEY>
"""

neo_template = """-- title

Span: SPANKEY 

-- subtitle

TEXT

-- p

Basic Example: 

<<SPANKEY|Lorem ipsum>>


Attribute Example: 

<<SPANKEY|Lorem ipsum|class: green>>


-- metadata
-- date: 2024-02-03 02:53:47
-- id: KSUID
-- status: published

"""

for span in spans:
    jinja_path = f"{theme_directory}/{span[0]}.jinja"
    jinja_text = jinja_template.replace("SPANKEY", span[0])
    with open(jinja_path, "w") as _out:
        _out.write(jinja_text)

    neo_path = f"{pages_directory}/{span[0]}.neo"
    neo_text = neo_template.replace("SPANKEY", span[0]).replace(
        "TEXT", span[1]).replace("KSUID", span[2])
    with open(neo_path, "w") as _out2:
        _out2.write(neo_text)


# Load the custom templates

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


# Copy over the internal spans

internal_list = [
    file for file in glob.glob(f"{internal_directory}/*")
    if os.path.isfile(file)
]

for internal_span in internal_list:
    internal_output_path = internal_span.replace(
        internal_directory, theme_directory)
    copy2(internal_span, internal_output_path)
