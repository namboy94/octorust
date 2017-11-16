/*
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
*/

#include <octopos.h>
#include <stdio.h>

void main_ilet(claim_t claim) {

    int LIMIT = 1690000;
    int removed[1690000 - 2] = {};
    int root = 1300;

    for (int i = 2; i < LIMIT; i++) {
        removed[i - 2] = 0;
    }

    for (int i = 2; i < root; i++) {
        if (removed[i - 2] == 0) {
            printf("%d\n", i);
        }
        for (int j = i * i; j < LIMIT; j += i) {
            removed[j - 2] = 1;
        }
    }

    for (int i = root + 1; i < LIMIT; i++) {
        if (removed[i - 2] == 0) {
            printf("%d\n", i);
        }
    }
    shutdown(0);
}
