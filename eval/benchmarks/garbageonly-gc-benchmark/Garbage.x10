import x10.io.Console;

class Garbage {
	public static def main(Array[String]) {

	    val GARBAGE_SIZE = 1000;
        val ITERATIONS = 100000;

        for (i in 0..ITERATIONS) {
            int[] garbage = [0; GARBAGE_SIZE];
        }

        Console.OUT.println("Done");

	}
};
