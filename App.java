import java.util.concurrent.CompletableFuture;

public class App {
    public static void main(String[] args) {
        // spawn thread
        CompletableFuture
        .supplyAsync(() -> {
            return "some string";
        })
        .thenApply(some -> {
            return some.replace('o', 'a');
        })
        // join
        .whenComplete((v, e) -> {
            if (e != null) {
                System.err.println(e);
            } else {
                System.out.println(v);
            }
        });
    }
}

