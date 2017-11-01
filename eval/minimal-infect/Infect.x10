import x10.io.Console;

import invasic.constraints.PEQuantity;
import invasic.constraints.TileSharing;
import invasic.IncarnationID;
import invasic.Claim;

class Infect {
	public static def main(Array[String]) {

	    val ilet = (id: IncarnationID) => {
	        Console.OUT.println("Hello World!");
	    };

	    val constraints = new PEQuantity(4, 4)
	        && TileSharing.WITH_OTHER_APPLICATIONS;
        val claim = Claim.invade(constraints);
        claim.infect(ilet);
        claim.retreat();
	}
};
