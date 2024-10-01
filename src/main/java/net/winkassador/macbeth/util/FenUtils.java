package net.winkassador.macbeth.util;

import net.winkassador.macbeth.chess.Position;
import net.winkassador.macbeth.exception.FenException;

import java.util.Objects;

public class FenUtils {

    public static Position positionFromFen(String fen) {
        if(fen == null || fen.isEmpty()) throw new FenException("Fen can't be null or empty.", fen);
        fen = fen.trim();

        String[] parts = fen.split(" ");

        if(parts.length > 6) {
            throw new FenException("Fen is too long.", fen);
        }

        String pieces = ArrayUtils.getValueOrDefault(parts, 0, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        String colorToMove = ArrayUtils.getValueOrDefault(parts, 1, "w");
        String castlingRights = ArrayUtils.getValueOrDefault(parts, 2, "KQkq");
        String enPassantSquare = ArrayUtils.getValueOrDefault(parts, 3, "-");
        String halfMoveClock = ArrayUtils.getValueOrDefault(parts, 4, "0");
        String fullMoveClock = ArrayUtils.getValueOrDefault(parts, 5, "1");

        Position position = new Position();

        boolean haveSeenWhiteKing = false;
        boolean haveSeenBlackKing = false;

        int fenReaderPosition = 0;
        int rank = 7, file = 0;
        for(int i = 0; i < pieces.length(); i++) {
            char c = pieces.charAt(i);

            if(file > 8) throw new FenException("Invalid Piece Notation", fen, i - 1);

            if(Character.isDigit(c)) {
                int whitespaceNumber = Character.getNumericValue(c);

                if(whitespaceNumber > 8 || whitespaceNumber < 0) {
                    throw new FenException("Whitespace Number in Piece Notation cannot be negative, or larger than 8.", fen, i);
                }

                file += whitespaceNumber;
                continue;
            }

            int piece;
            int colorOffset = Character.isUpperCase(c) ? 0 : 6;

            switch (c) {
                case '/' -> { rank--; file = 0; continue; }
                case 'P', 'p' -> {
                    if((colorOffset == 0 && rank == 7) || (colorOffset != 0 && rank == 0)) throw new FenException("Pawns can't be on the promotion rank.", fen, i);
                    piece = Position.PAWN;
                }
                case 'N', 'n' -> piece = Position.KNIGHT;
                case 'B', 'b' -> piece = Position.BISHOP;
                case 'R', 'r' -> piece = Position.ROOK;
                case 'Q', 'q' -> piece = Position.QUEEN;
                case 'K', 'k' -> {
                    if(colorOffset == 0) {
                        if(haveSeenWhiteKing) throw new FenException("Can't have more than one of each type of king.", fen, i);
                        haveSeenWhiteKing = true;
                    }
                    else {
                        if(haveSeenBlackKing) throw new FenException("Can't have more than one of each type of king.", fen, i);
                        haveSeenBlackKing = true;
                    }
                    piece = Position.KING;
                }
                default -> throw new FenException("Unknown piece in setup.", fen, i);
            }

            position.setPieceAt(position.indexFromBoardPosition(file, rank), piece + colorOffset);

            file++;
        }

        fenReaderPosition += pieces.length() + 1;

        if(!haveSeenWhiteKing || !haveSeenBlackKing) {
            throw new FenException("Must have exactly one of each King.", fen);
        }

        if(!Objects.equals(colorToMove, "w") && !Objects.equals(colorToMove, "b")) {
            throw new FenException("Color to play must be either 'w' or 'b'. ", fen, fenReaderPosition);
        }
        position.setIsWhiteToMove(Objects.equals(colorToMove, "w"));

        fenReaderPosition++;

        boolean[] castlingRightsArray = new boolean[4];
        if(!castlingRights.equals("-")) {
            for(char c : castlingRights.toCharArray()) {
                if(c == 'K') {
                    fenReaderPosition++;
                    if(castlingRightsArray[0]) throw new FenException("Invalid Castling Rights", fen, fenReaderPosition);
                    castlingRightsArray[0] = true;
                }
                else if(c == 'Q') {
                    fenReaderPosition++;
                    if(castlingRightsArray[1]) throw new FenException("Invalid Castling Rights", fen, fenReaderPosition);
                    castlingRightsArray[1] = true;
                }
                else if(c == 'k') {
                    fenReaderPosition++;
                    if(castlingRightsArray[2]) throw new FenException("Invalid Castling Rights", fen, fenReaderPosition);
                    castlingRightsArray[2] = true;
                }
                else if(c == 'q') {
                    fenReaderPosition++;
                    if(castlingRightsArray[3]) throw new FenException("Invalid Castling Rights", fen, fenReaderPosition);
                    castlingRightsArray[3] = true;
                }
                else {
                    fenReaderPosition++;
                    throw new FenException("Invalid Character in Castling Rights.", fen, fenReaderPosition);
                }
            }
        }
        position.setCastlingRights(castlingRightsArray);

        fenReaderPosition += 2;

        if(!enPassantSquare.equals("-")) {
            fenReaderPosition++;
            if(enPassantSquare.length() != 2) throw new FenException("Invalid En Passant Location.", fen, fenReaderPosition);
            char enPassantFile = enPassantSquare.charAt(0), enPassantRank = enPassantSquare.charAt(1);
            if(enPassantFile > 'h' || enPassantFile < 'a' || enPassantRank < '1' || enPassantRank > '8') throw new IllegalArgumentException("Invalid En Passant Location.");
            if(enPassantRank != '3' && enPassantRank != '6') throw new IllegalArgumentException("En Passant on square where En Passant can not happen");
        }
        position.setEnPassantIndex(position.getIndexFromCoordinateName(enPassantSquare));
        if(!enPassantSquare.equals("-")) {
            int index = position.getIndexFromCoordinateName(enPassantSquare);
            if(!position.isWhiteToMove()) {
                int p = position.getPieceAt(index + 8);
                if(p != Position.PAWN) throw new IllegalArgumentException("No Pawn Next to Specified En Passant Square.");
            }
            else {
                int p = position.getPieceAt(index - 8);
                if(p != Position.PAWN + 6) throw new IllegalArgumentException("No Pawn Next to Specified En Passant Square.");
            }
        }

        if(!NumberUtils.isInteger(halfMoveClock)) {
            throw new FenException("Half-Move Clock is not a valid number.", fen, fenReaderPosition);
        }
        int halfMoveClockInt = Integer.parseInt(halfMoveClock);
        if(halfMoveClockInt < 0) throw new IllegalArgumentException("Half-Move Clock may not be less than 0.");
        position.setHalfMoveClock(halfMoveClockInt);

        if(!NumberUtils.isInteger(fullMoveClock)) {
            throw new IllegalArgumentException("Full-Move Clock is not a valid number. (" + halfMoveClock + ")");
        }
        int fullMoveClockInt = Integer.parseInt(fullMoveClock);
        if(fullMoveClockInt < 0) throw new IllegalArgumentException("Full-Move Clock may not be less than 0.");
        position.setFullMoveClock(fullMoveClockInt);

        return position;
    }

    public static String fenFromPosition(Position position) {
        StringBuilder b = new StringBuilder();

        int whitespaceCounter = 0;
        for(int rank = 7; rank >= 0; rank--) {
            if(rank != 7) b.append("/");

            for(int file = 0; file < 8; file++) {
                int piece = position.getPieceAt(position.indexFromBoardPosition(file, rank));
                if(piece == -1) {
                    whitespaceCounter++;
                }
                else {
                    if(whitespaceCounter > 0) {
                        b.append(whitespaceCounter);
                        whitespaceCounter = 0;
                    }

                    b.append(Position.PIECE_NOTATION[piece]);
                }
            }

            if(whitespaceCounter > 0) {
                b.append(whitespaceCounter);
                whitespaceCounter = 0;
            }
        }

        b.append(" ").append(position.isWhiteToMove() ? "w" : "b");

        String castlingRightsString = "";
        boolean[] castlingRights = position.getCastlingRights();
        if(castlingRights[0]) castlingRightsString += "K";
        if(castlingRights[1]) castlingRightsString += "Q";
        if(castlingRights[2]) castlingRightsString += "k";
        if(castlingRights[3]) castlingRightsString += "q";

        if(castlingRightsString.isEmpty()) castlingRightsString = "-";

        b.append(" ").append(castlingRightsString);
        b.append(" ").append(position.getEnPassantIndex() == -1 ? "-" : position.getSquareCoordinateName(position.getEnPassantIndex()));
        b.append(" ").append(position.getHalfMoveClock());
        b.append(" ").append(position.getFullMoveClock());

        return b.toString();
    }

}
