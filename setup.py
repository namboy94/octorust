# imports
import os
import shutil
from setuptools import setup, find_packages


setup(name="octorust",
      version="0.0.1",
      description="A OctoPOS rust compiler/build tool",
      author="Hermann Krumrey",
      author_email="hermann@krumreyh.com",
      packages=find_packages(),
      install_requires=[],
      test_suite='nose.collector',
      tests_require=['nose'],
      scripts=["bin/octorust"],
      zip_safe=False)

#  Custom dependency installation
deps = os.path.join(os.path.expanduser("~"), ".octorust")
octolib_dep = os.path.join(deps, "octolib")

if os.path.isdir(octolib_dep):
    shutil.rmtree(octolib_dep)
shutil.copytree("octolib", octolib_dep)