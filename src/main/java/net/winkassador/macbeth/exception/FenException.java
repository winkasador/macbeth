package net.winkassador.macbeth.exception;

public class FenException extends RuntimeException {

  public String fen;
  public int position;

  public FenException(String message, String fen) {
    super(message);

    this.fen = fen;
    this.position = -1;
  }

  public FenException(String message, String fen, int position) {
    super(message);
    this.fen = fen;
    this.position = position;
  }

}
