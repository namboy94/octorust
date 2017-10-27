import x10.io.Console;

import invasic.constraints.PEQuantity;
import invasic.constraints.TileSharing;
import invasic.IncarnationID;
import invasic.Claim;

class Primes {
    public static def main(Array[String]) {

        val pes = 1;
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
