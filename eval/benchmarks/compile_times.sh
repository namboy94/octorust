#!/bin/bash

temci short exec -wd "octorust -i 2017-06-07 startup/startup -o out" --runs 50 --out temci_output/compile_rust.yaml
temci short exec -wd "octorust -i 2017-06-07 startup/startup --release -o out" --runs 50 --out temci_output/compile_rust_opt.yaml
temci short exec -wd "octorust -i 2017-06-07 startup/startup.c -o out" --runs 50 --out temci_output/compile_c.yaml
temci short exec -wd "octorust -i 2017-06-07 startup/startup.c --release -o out" --runs 50 --out temci_output/compile_c_opt.yaml
temci short exec -wd "x10firm startup/Startup.x10 -mtarget=i686-invasic-irtss -o out" --runs 50 --out temci_output/compile_x10.yaml
temci short exec -wd "x10firm startup/Startup.x10 -mtarget=i686-invasic-irtss -o out -O3" --runs 50 --out temci_output/compile_x10_opt.yaml
