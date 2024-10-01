package net.winkassador.macbeth;

import net.winkassador.macbeth.exception.FenException;
import net.winkassador.macbeth.util.FenUtils;

public class Main {

    public static void main(String[] args) {
        if(args.length != 1) {
            showUsage();
        }

        String fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        try {
            System.out.println(FenUtils.fenFromPosition(FenUtils.positionFromFen(fen)));
        }
        catch (FenException e) {
            System.out.println("The fen you entered is not valid.");
            System.out.println();
            System.out.printf("Fen: \"%s\"\n", e.fen);
            if(e.position == -1) {
                System.out.println("Why: " + e.getMessage());
            }
            else {
                System.out.printf("%" + (e.position + 5) + "s\"%s\" <-- %s\n", "", e.fen.charAt(e.position), e.getMessage());
            }

            System.out.println();
        }

        System.out.println();
    }

    private static void showUsage() {
        System.out.println("Usage: macbeth.exe [FEN]");
    }

}
