# config parser
Tool to parse configs or log files based on custom json schema. Originally built for juniper set config, it can be used to parse any config or log that has repeating elements such as

set interfaces et-0/0/56 description "description"

set interfaces et-0/0/56 hold-time up 100

set interfaces et-0/0/56 hold-time down 0 


example:

./config_parser --config your_set_config --regex regex.json --structure structure.json



