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
#include <stdlib.h>


void main_ilet(claim_t claim) {

    int GARBAGE_SIZE = 5000;
    int ITERATIONS = 1000000;

    for (int i = 0; i < ITERATIONS; i++) {

        int *array = (int*) malloc(GARBAGE_SIZE * sizeof(int));
        for (int j = 0; j < GARBAGE_SIZE; j++) {
            array[j] = 0;
        }
		free(array);

	}

    printf("Done");
    shutdown(0);
}
