import x10.io.Console;

class NaivePrimes {
	public static def main(Array[String]) {

	    val LIMIT = 10000;

	    for (i in 2..LIMIT) {
	        var isPrime = true;
	        for (j in 2..i) {
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
