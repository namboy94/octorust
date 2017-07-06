import os
import sys
from typing import List, Tuple


function_signatures = []


def main():
    if len(sys.argv) < 2 or not os.path.isfile(sys.argv[1]):
        print("Please provide a path to the target file as argument")
        sys.exit(1)

    target = sys.argv[1]
    converted_file = target + ".new.rs" if len(sys.argv) < 3 else sys.argv[2]

    with open(target, 'r') as f:
        content = f.read()

    new_content = convert(content.split("\n"))

    with open(converted_file, 'w') as f:
        f.write(new_content)


def convert(content: List[str]) -> str:

    blocks = determine_code_blocks(content)
    processed_blocks = []
    processed = ""

    for block in blocks:
        processed_blocks.append(handle_code_block(block))

    for block in processed_blocks:
        for line in block:
            processed += line + "\n"
        processed += "\n"

    dependencies = "use octo_bindings::octo_types::*;\n\n"
    extern_block = generate_extern_block()
    return dependencies + extern_block + processed


def determine_code_blocks(content: List[str]) -> List[List[str]]:

    blocks = []
    current_block = []

    for line in content:

        if line.strip() != "":
            current_block.append(line)
        elif len(current_block) > 0:
            blocks.append(current_block)
            current_block = []

    return blocks


def handle_code_block(block: List[str]) -> List[str]:

    comments_converted = convert_comments(block)
    processed, signature = convert_function(comments_converted)
    function_signatures.append(signature)
    return processed


def convert_comments(block: List[str]) -> List[str]:

    # TODO Work on this

    converted = []

    for i in range(0, len(block)):

        line = block[i]
        next_line = None if i >= len(block) - 1 else block[i + 1]
        prev_line = None if i == 0 else block[i - 1]

        if not (line.startswith("/**")
                or line.startswith(" */")
                or line.startswith(" *")
                or line.startswith("*/")):
            converted.append(line)
            continue

        # Convert to Rust Doc Comments
        line = line.replace("/**", "///")
        line = line.replace(" */", "///")
        line = line.replace("*/", "///")
        line = line.replace(" *", "///")
        line = line.replace("\\brief ", "")
        line = line.replace("\\file ", "")

        # Trim unnecessary lines
        if line.replace("///", "").strip() == "":
            if prev_line is None or prev_line.strip() == "":
                continue
            if next_line is None or next_line.strip() == "":
                continue
            if next_line is not None and not next_line.startswith("///"):
                continue

        # Convert params, return values etc.
        if line.replace("///", "").strip().startswith("\\return "):
            converted.append("/// # Return Value")
            converted.append("///")
            line = line.replace("\\return ", "")
            line += "\n///"

        elif line.replace("///", "").strip().startswith("\\param "):
            if prev_line is None or \
                    not prev_line.replace("*", "").strip().startswith(
                        "\\param "):
                converted.append("///\n/// # Arguments")
                converted.append("///")
            line = line.replace("\\param ", "").replace("///", "").strip()
            varname = line.split(" ", 1)[0]
            line = "/// * `" + varname + "` - " + line.split(" ", 1)[1]

            if next_line is None or \
                    not next_line.replace("*", "").strip().startswith(
                        "\\param "):
                line += "\n///"

        elif line.replace("///", "").strip().startswith("\\note "):

            if prev_line is None or \
                    not prev_line.replace("*", "").strip().startswith(
                        "\\note "):
                converted.append("///\n/// # Note\n///")
            line = "/// " + line.split("note", 1)[1]

        converted.append(line)

    return converted


def convert_function(block: List[str]) -> Tuple[List[str], Tuple[str, str, List[Tuple[str, str]]]] or None:

    index = -1

    function_line = ""
    for i, line in enumerate(block):
        line = line.split("//")[0]
        if not line.startswith("///") and line.strip().endswith(";"):
            function_line = line
            index = i

    if function_line == "":
        return block, None
    else:

        function_line = function_line.replace("(void)", "()")

        function_name = function_line.split("(", 1)[0].rsplit(" ", 1)[1]
        return_type = function_line.split(function_name, 1)[0].strip()
        return_type = determine_type("", return_type)[1]
        params = function_line.split("(", 1)[1].rsplit(")", 1)[0].split(",")

        processed_params = []

        for param in params:
            if param == "":
                continue

            param_name = param.strip().rsplit(" ", 1)[1].strip()
            param_type = param.strip().rsplit(" ", 1)[0].strip()
            param_name, param_type = determine_type(param_name, param_type)
            processed_params.append((param_name, param_type))

        signature = (function_name, return_type, processed_params)
        definition = create_function_definition(signature)
        block[index] = definition

        return block, signature


def create_function_definition(signature: Tuple[str, str, List[Tuple[str, str]]]) -> str:

    definition = "pub fn " + signature[0] + "("
    for arg in signature[2]:
        definition += arg[0] + ": " + arg[1] + ", "
    if definition.endswith(", "):
        definition = definition[0:-2]
    definition += ")"

    if signature[1] != "void":
        definition += " -> " + signature[1]

    definition += " {\n    unsafe {\n        __" + signature[0] + "("

    for arg in signature[2]:
        definition += arg[0] + ", "
    if definition.endswith(", "):
        definition = definition[0:-2]
    definition += ")\n    }\n}"

    return definition


def determine_type(name: str, c_type: str) -> Tuple[str, str]:

    if c_type.endswith("*"):
        c_type = "*" + c_type[0:-1].strip()
    elif name.endswith("[]"):
        name = name[0:-2].strip()
        c_type = "*mut " + c_type

    if name == "type":
        name = "_type"

    if c_type == "int":
        c_type = "i32"
    elif c_type == "uint32_t":
        c_type = "u32"
    elif c_type == "uint8_t":
        c_type = "u8"
    elif c_type == "uint16_t":
        c_type = "u16"
    elif c_type == "size_t":
        c_type = "usize"

    return name, c_type


def generate_extern_block() -> str:
    extern = "extern {\n"

    for signature in function_signatures:

        if signature is None:
            continue

        extern += "    #[link_name=\"" + signature[0] + "\"]\n"
        extern += "    fn __" + signature[0] + "("
        for param in signature[2]:
            extern += param[0] + ": " + param[1] + ", "
        if extern.endswith(", "):
            extern = extern[0:-2]
        extern += ")"

        if signature[1] != "void":
            extern += " -> " + signature[1]

        extern += ";\n\n"

    extern += "}\n\n"

    return extern

if __name__ == "__main__":
    main()
