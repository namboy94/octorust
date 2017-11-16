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

import invasic.constraints.PEQuantity;
import invasic.constraints.TileSharing;
import invasic.IncarnationID;
import invasic.Claim;

class Primes {
    public static def main(Array[String]) {

        val pes = 8;
        val limit = 500000;

        val ilet = (id: IncarnationID ):Rail[Boolean] => {

            val step = limit / pes;
            var lower_limit: int = id.ordinal * step + 1;
            var upper_limit: int = (id.ordinal + 1) * step;

            if (lower_limit == 1) {
                lower_limit += 1;
            }

            if (id.ordinal == (pes - 1)) {
                upper_limit = limit;
            }

            val primes_array = new Rail[Boolean](upper_limit - lower_limit + 1);
            for (i in lower_limit..(upper_limit - 1)) {
                primes_array(i - lower_limit) = Primes.isPrime(i);
            }

            return primes_array;
        };
        val constraints = new PEQuantity(pes, pes) && TileSharing.WITH_OTHER_APPLICATIONS;

        val claim = Claim.invade(constraints);
        val result = claim.infect[Rail[Boolean]](ilet);

        var offset: int = 2;
        for (i in 0..(pes - 1)) {
            for (j in 0..(result(i).size - 1)) {
                if (result(i)(j)) {
                    Console.OUT.println("" + (offset + j));
                }
            }
            if (offset == 2) {
                offset--;
            }
            offset += limit / pes;
        }
    }

    public static def isPrime(number: int): Boolean {
        for (i in 2..(number - 1)) {
            if (number % i == 0) {
                return false;
            }
        }
        return true;
    }
};
