package net.winkassador.macbeth;

public class Main {

    public static void main(String[] args) {
        if(args.length != 1) {
            showUsage();
            return;
        }

        String fen = args[0];
    }

    private static void showUsage() {
        System.out.println("Usage: macbeth.exe [FEN]");
    }

}
