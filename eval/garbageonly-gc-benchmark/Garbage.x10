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

import x10.io.Console;

class Garbage {
	public static def main(Array[String]) {

	    val GARBAGE_SIZE = 5000;
        val ITERATIONS = 1000000;

        for (i in 0..ITERATIONS) {
             new Rail[int](GARBAGE_SIZE, 0);
        }

        Console.OUT.println("Done");

	}
};
