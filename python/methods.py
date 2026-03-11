from typing import List
from os import listdir
from os.path import isfile, join
from json import load as json_load

properties_registry = {}


class RelationsRegistry:
    relations_registry = {} # RID: ...
    property_relations = {} # property.path: RID

    def add_relation(self, relation):
        rid = len(self.relations_registry)
        self.relations_registry[rid] = relation
        relation_properties = []
        if relation["type"] in ["equal", "notEqual"]:
            relation_properties = [relation["propertyA"], relation["propertyB"]]
        elif relation["type"] in ["implies", "notImplies"]:
            relation_properties = [relation["propertyA"]]
        elif relation["type"] == "equation":
            relation_properties = list(relation["properties"].values())
        elif relation["type"] == "expression":
            relation_properties = [relation["output"]]
        for prop in relation_properties:
            self.property_relations[prop] = rid

relations_registry = RelationsRegistry()

def load_properties(properties: list, path: List[str] = []):
    global properties_registry
    for prop in properties:
        prop_path = path.copy()
        prop_path.append(prop["name"])
        prop_path_str = ".".join(prop_path)
        prop_data = {"type": prop["type"]}
        if prop["type"] == "defined" and "properties" in prop:
            load_properties(prop["properties"], prop_path)
        elif prop["type"] == "list" and "length" in prop:
            # Special length property
            properties_registry[f"{'.'.join(path)}.{prop['length']}"] = {
                "type": "number"
            }

            # TODO: Add relationship between the list and the length property

        properties_registry[prop_path_str] = prop_data


def load_relations(relations: list):
    global relations_registry
    for relation in relations:
        relations_registry.add_relation(relation)


def load_method(method):
    if "properties" in method:
        load_properties(method["properties"])


def load_all_methods(path: str):
    method_files = [f for f in listdir(path) if isfile(join(path, f))]
    for method_file in method_files:
        with open(join(path, method_file), "r") as f:
            method_data = json_load(f)
            load_method(method_data)
