import x10.io.Console;

class Primes {
	public static def main(Array[String]) {

	    val LIMIT = 1690000;
	    val removed = new Rail[Boolean](LIMIT - 2, false);

	    val root = 1300
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
