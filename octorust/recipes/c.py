from octorust.recipes.c_obj import compile_c_object
from octorust.util.config import Config
from octorust.util.files import cleanup
from octorust.linking.octopos_link import link_app


def compile_c(config: Config):
    """
    Compiles a C file and links it to OctoPOS
    :param config: The configuration to use while compiling
    :return: None
    """
    object_file = compile_c_object(config, config.source)
    link_app(config, [object_file])
    cleanup([object_file], config)
