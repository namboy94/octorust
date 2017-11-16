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
