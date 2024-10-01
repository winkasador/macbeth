package net.winkassador.macbeth.chess;

public class Position {

    public static final int EMPTY = -1, PAWN = 0, KNIGHT = 1, BISHOP = 2, ROOK = 3, QUEEN = 4, KING = 5;
    public static final char[] PIECE_NOTATION = new char[] {'P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k'};

    public static final String[] RANKS = new String[] {"1", "2", "3", "4", "5", "6", "7", "8"};
    public static final String[] FILES = new String[] {"a", "b", "c", "d", "e", "f", "g", "h"};

    private final long[] bitboards;
    private boolean[] castlingRights;
    private long occupationBitboard;
    private int halfMoveClock, fullMoveClock;
    private int enPassantIndex;
    private boolean isWhiteToMove;

    /**
     * Creates an empty position with no data.
     */
    public Position() {
        this.bitboards = new long[12];
        this.castlingRights = new boolean[4];
        this.isWhiteToMove = true;
        this.enPassantIndex = -1;
        this.occupationBitboard = 0L;
    }

    public boolean isSquareOccupied(int index) {
        return (occupationBitboard & (1L << index)) != 0;
    }

    public int getPieceAt(int index) {
        if(!isSquareOccupied(index)) return -1; // faster cause no 12 checks for empty squares
        else {
            for(int i = 0; i < bitboards.length; i++) {
                if((bitboards[i] & (1L << index)) != 0) return i;
            }
        }

        return -1;
    }

    public boolean setPieceAt(int index, int piece) {
        if(isSquareOccupied(index)) return false;

        occupationBitboard = occupationBitboard | 1L << index;
        bitboards[piece] = bitboards[piece] | 1L << index;

        return true;
    }

    public int indexFromBoardPosition(int file, int rank) {
        return 8 * rank + file;
    }

    public boolean isWhiteToMove() {
        return this.isWhiteToMove;
    }

    public int getEnPassantIndex() {
        return this.enPassantIndex;
    }

    public int getHalfMoveClock() {
        return this.halfMoveClock;
    }

    public int getFullMoveClock() {
        return this.fullMoveClock;
    }

    public void setCastlingRights(boolean... castlingRights) {
        this.castlingRights = castlingRights;
    }

    public String getSquareCoordinateName(int index) {
        int rank = index / 8;
        int file = index - rank * 8;

        return getSquareCoordinateName(file, rank);
    }

    public boolean[] getCastlingRights() {
        return this.castlingRights;
    }

    public String getSquareCoordinateName(int file, int rank) {
        return FILES[file] + RANKS[rank];
    }

    public int getIndexFromCoordinateName(String coordinate) {
        if(coordinate.equals("-")) return -1;

        char fileChar = coordinate.charAt(0);
        char rankChar = coordinate.charAt(1);

        int file = Character.getNumericValue(Character.toUpperCase(fileChar)) - 10; // ...'9' = 9, 'A' = 10, 'B' = 11...
        int rank = Character.getNumericValue(rankChar) - 1;

        return indexFromBoardPosition(file, rank);
    }

    // Castling Rights...

    public void setIsWhiteToMove(boolean isWhiteToMove) {
        this.isWhiteToMove = isWhiteToMove;
    }

    public void setEnPassantIndex(int index) {
        this.enPassantIndex = index;
    }

    public void setFullMoveClock(int fullMoveClock) {
        this.fullMoveClock = fullMoveClock;
    }

    public void setHalfMoveClock(int halfMoveClock) {
        this.halfMoveClock = halfMoveClock;
    }

}
