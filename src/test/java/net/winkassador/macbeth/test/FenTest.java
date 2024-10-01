package net.winkassador.macbeth.test;

import net.winkassador.macbeth.util.FenUtils;
import org.junit.jupiter.api.Test;

public class FenTest {

    private static final String[] validTestFenStrings = new String[] {
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", // Default
            "r1bqk2r/pppp1ppp/2n5/8/2BPn3/5N2/PP1B1PPP/R2Q1RK1 b kq - 1 9", // Giuoco Piano Game: Center Attack
            "r2qkbnr/pp2pppp/2n5/2PpP3/6b1/5N2/PPP2PPP/RNBQKB1R w KQkq - 3 6", // Caro-Kann Defense: Advance, Botvinnik-Carls Defense
            "3r1rk1/3nq2p/p1n1p1pb/1pp1P3/3p1P1P/P1P1BNPB/1P6/R2QR1K1 w - - 0 23", // Kasparov v. Petursson (2004)
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP4/Q1n2N2/P4PPP/3R1K1R w - - 4 18", // Game of the Century
            "rnbqkbnr/pp1ppppp/8/8/1Pp1P3/5P2/P1PP2PP/RNBQKBNR b KQkq b3 0 3", // Random Position with En Passant.
    };

    private static final String[] invalidTestFenStrings = new String[] {
            "rnbqkbnr/pppppmpp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", // Invalid Character in Piece Setup
            "", // Empty
            null, // Null
            "djkyhse jmoweimt osnidj nibe awrwq ibar42hbhi awtjknes je etjsnt ne jnnjjn jn jn", // Random Characters
            "rnbqkbnr/pppppppp/8/9/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", // Too much empty space.
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR - KQkq - 0 1", // Player to move is not w or b.
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - -6 1", // Negative half-move clock.
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq h9 0 1", // En passant square does not exist.
            "rnbq1bnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1", // No king
            "rnbqkknr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", // Too many kings.
            "rPbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1", // Pawn on promotion rank.
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKpNR w KQkq - 0 1", // Pawn on other promotion rank.
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 5", // Extra data on the end of the FEN.
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq a4 0 1", // En passant on rank where en passant can not happen.
            "rnbqkbnr/pppp1ppp/8/4p3/3PP3/8/PPP2PPP/RNBQKBNR b KQkq f3 0 2", // En passant for pawn that has not moved.
    };

    @Test
    public void testFenReadAndWrite() {
        for(String testString : validTestFenStrings) {
            try {
                String output = FenUtils.fenFromPosition(FenUtils.positionFromFen(testString));

                if(!output.equals(testString)) {
                    throw new AssertionError(String.format("Error whilst comparing FEN!\nInput Fen:  %s\nOutput Fen: %s", testString, output));
                }
            }
            catch (IllegalArgumentException e) {
                System.out.println("Error whilst compiling FEN!");
                System.out.println("FEN: " + testString);
                throw e;
            }
        }
    }

    @Test
    public void testInvalidFen() {
        for(String testString : invalidTestFenStrings) {
            try {
                FenUtils.fenFromPosition(FenUtils.positionFromFen(testString));
            }
            catch (Exception e) {
                // Invalid FEN should throw an error, but it should specifically be
                // an IllegalArgumentException with a message about what was wrong.
                assert e instanceof IllegalArgumentException;
                continue;
            }

            throw new RuntimeException("Invalid FEN did not cause error. (" + testString + ")");
        }
    }

}
