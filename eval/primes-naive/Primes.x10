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

	    val LIMIT = 50000;

	    for (i in 2..LIMIT) {
	        var isPrime:Boolean = true;
	        for (j in 2..(i - 1)) {
	            if (i % j == 0) {
                    isPrime = false;
	            }
	        }
	        if (isPrime) {
	            Console.OUT.println("" + i);
	        }
	    }
	}
};
