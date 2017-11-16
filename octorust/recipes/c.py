"""
Copyright 2017 Hermann Krumrey <hermann@krumreyh.com>

This file is part of octorust.

octorust is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

octorust is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with octorust.  If not, see <http://www.gnu.org/licenses/>.
"""

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
