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

class Primes {
	public static def main(Array[String]) {

	    val LIMIT = 1690000;
	    val removed = new Rail[Boolean](LIMIT - 2, false);

	    val root = 1300;
	    //val root = Int.operator_as(Math.sqrt(Double.implicit_operator_as(LIMIT)));

	    for (i in 2..root) {
	        if (!removed(i - 2)) {
	            Console.OUT.println(i);
	        }
	        var j:int = i * i;
	        while (j < LIMIT) {
	            removed(j - 2) = true;
	            j += i;
	        }
	    }
	    for (i in (root + 1)..LIMIT) {
	        if (!removed(i - 2)) {
	            Console.OUT.println(i);
	        }
	    }
	}
};
