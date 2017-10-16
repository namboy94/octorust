import x10.io.Console;

class Primes {
	public static def main(Array[String]) {

	    val LIMIT = 10000;
	    val removed = []; // 10000 - 2 => false

	    for (i in 2..sqrt(LIMIT)) {
	        if (!removed[i - 2]) {
	            Console.OUT.println("" + i);
	        }
	        var j = i * i;
	        while (j < LIMIT) {
	            removed[j - 2] = true;
	            j += i;
	        }
	    }
	    for (i in (sqrt(LIMIT) + 1)..LIMIT) {
	        if (!removed[i - 2]) {
	            Console.OUT.println("" + i);
	        }
	    }
	}
};
