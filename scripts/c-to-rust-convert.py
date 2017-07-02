import sys

def handle_function_definition(line):
    args = line.split("(")[1].split(")")[0]
    args = args.replace("void*", "c_void")
    args = args.split(",")
    c_signature = line.split("(")[0].rsplit(" ", 1)
    return_type, function_name = c_signature

    signature = "pub fn " + function_name + "("
    arg_names = []
    for arg in args:
        if arg.strip() == "":
            continue
        arg_type = arg.strip().rsplit(" ", 1)[0].strip()

        if arg_type.endswith("*"):
            arg_type = "*" + arg_type.rsplit("*")[0]

        arg_name = arg.strip().rsplit(" ", 1)[1]

        if arg_name.endswith("[]"):
            arg_name = arg_name.rsplit("[]", 1)[0]
            arg_type = "*" + arg_type

        if arg_name == "type":
            arg_name = "_type"

        signature += arg_name + ": " + arg_type + ", "

        arg_names.append(arg_name)

    if signature.endswith(", "):
        signature = signature.rsplit(", ", 1)[0]

    signature += ") -> " + return_type

    function_definition = signature + " {\n    __" + function_name + "("
    for arg_name in arg_names:
        function_definition += arg_name + ", "

    if function_definition.endswith(", "):
        function_definition = function_definition.rsplit(", ", 1)[0]

    function_definition += ")\n}"

    return function_definition + "\n", signature


def main():

    source = sys.argv[1]

    with open(source, "r") as f:
        content = f.read()

    # Convert
    content = content.replace("/**", "///")
    content = content.replace(" */", "///")
    content = content.replace(" *", "///")
    content = content.replace("\\brief ", "")
    content = content.replace("(void)", "()")

    lines = content.split("\n")
    new_content = ""
    signatures = []

    for i in range(0, len(lines)):
        line = lines[i]
        next_line = None if i == len(lines) - 1 else lines[i + 1]
        prev_line = None if i == 0 else lines[i - 1]

        if prev_line is not None and line.strip() == "///" and prev_line.strip() == "":
            continue

        elif next_line is not None and line.strip() == "///" and next_line.strip() == "":
            continue

        elif line.startswith("/// \\return "):
            if not prev_line.strip() == "///":
                new_content += "\n///"
            new_content += "/// # Return Value\n///\n"
            new_content += line.replace("\\return ", "")

        elif line.startswith("/// \\note "):
            if not prev_line.strip() == "///":
                new_content += "\n///"
            new_content += "/// # Note\n///\n"
            new_content += line.replace("\\note ", "") + "\n"

        elif line.startswith("/// \\param "):
            if not prev_line.startswith("/// \\param"):
                if not prev_line.strip() == "///":
                    new_content += "\n///"
                new_content += "/// # Arguments\n///\n"
            line = line.split("/// \\param ", 1)[1].split(" ", 1)
            line = "/// * `" + line[0] + "` - " + line[1]
            new_content += line + "\n"

        elif line.strip().endswith(";") and not line.strip().startswith("///"):
            function_definition, signature = handle_function_definition(line)
            new_content += function_definition + "\n"
            signatures.append(signature)

        else:
            new_content += line + "\n"

    extern_def = "extern {\n\n"

    for signature in signatures:
        signature = signature.replace("pub fn ", "pub fn __")
        signature = signature.replace("pub fn", "    fn")
        name = signature.split("(")[0].rsplit("__", 1)[1]

        extern_def += f"    #[link_name=\"{name}\"]\n"
        extern_def += signature + ";\n\n"

    extern_def += "}\n\n"

    with open(source + ".new.rs", 'w') as f:
        f.write(extern_def + new_content)

if __name__ == "__main__":
    main()
