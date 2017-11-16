#!/bin/bash
# Copyright 2017 Hermann Krumrey <hermann@krumreyh.com>
#
# This file is part of octorust.
#
# octorust is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# octorust is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with octorust.  If not, see <http://www.gnu.org/licenses/>.

temci short exec -wd "octorust -i 2017-06-07 startup/startup -o out" --runs 50 --out temci_output/compile_rust.yaml
temci short exec -wd "octorust -i 2017-06-07 startup/startup --release -o out" --runs 50 --out temci_output/compile_rust_opt.yaml
temci short exec -wd "octorust -i 2017-06-07 startup/startup.c -o out" --runs 50 --out temci_output/compile_c.yaml
temci short exec -wd "octorust -i 2017-06-07 startup/startup.c --release -o out" --runs 50 --out temci_output/compile_c_opt.yaml
temci short exec -wd "x10firm startup/Startup.x10 -mtarget=i686-invasic-irtss -o out" --runs 50 --out temci_output/compile_x10.yaml
temci short exec -wd "x10firm startup/Startup.x10 -mtarget=i686-invasic-irtss -o out -O3" --runs 50 --out temci_output/compile_x10_opt.yaml
