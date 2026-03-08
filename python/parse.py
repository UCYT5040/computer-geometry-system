def parse_given(given: str, properties: dict):
    objects = {}

    for line in given.splitlines():
        if not line.strip():
            continue

        # Split the line into object name and properties
        if ":" not in line:
            continue  # Skip lines that don't have the expected format

        obj_name, props_value_str = line.split(":", 1)
        obj_name = obj_name.strip()
        props_str, value_str = props_value_str.split("=", 1)
        props_str = props_str.strip()

        if obj_name not in objects:
            objects[obj_name] = {}

        props = props_str.split(".")

        path = []
        for prop in props:
            path.append(prop)
            path_str = ".".join(path)

            if path_str not in properties:
                raise ValueError(
                    f"Property '{path_str}' is not defined in the registry"
                )

            # If the property is not of type `defined`, grab the value, then stop
            if properties[path_str]["type"] != "defined":
                value = value_str.strip()
                objects[obj_name][path_str] = value
                break

    return objects
