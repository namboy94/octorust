import x10.io.Console;

class Primes {
	public static def main(Array[String]) {

	    val LIMIT = 10000;

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
