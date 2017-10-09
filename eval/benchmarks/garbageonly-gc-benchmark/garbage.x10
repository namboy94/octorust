import x10.io.Console;

class GarbageOnly {
	public static def main(Array[String]) {

	    val GARBAGE_SIZE = 10000;
        val ITERATIONS = 1000000;

        for (i in 0..ITERATIONS) {
            int[] garbage = [0; GARBAGE_SIZE];
        }

        Console.OUT.println("Done");

	}
};
