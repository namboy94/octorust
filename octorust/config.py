"""
Author: Hermann Krumrey <hermann@krumreyh.com> (2017)
"""


class Config(object):
    """
    A class that provides a collection of options needed in the compilation
    process.
    """

    def __init__(self, arch: str, variant: str, source: str, out: str):
        """
        Creates a new Config object
        :param arch: The target architecture
        :param variant: The target variant
        :param source: The input source file
        :param out: The output file
        """
        self.arch = arch
        self.variant = variant
        self.source = source
        self.out = out

        # Generate names
        self.rust_static_lib = "lib" + self.source.rsplit(".rs", 1)[0] + ".a"
