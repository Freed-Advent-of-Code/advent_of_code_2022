
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Iterator;
import java.util.List;
import java.util.stream.Collectors;

public class Day11 {
    static final int ROUNDS = 20;

    public static void main(String[] args) {
        try {
            String input = Files.readString(Path.of("aoc/2022/day11.txt"));
            List<Monkey> monkeyList = processInput(input);
            for (int i = 0; i < ROUNDS; i++) {
                for (Monkey monkey : monkeyList) {
                    monkey.takeTurn().forEach(action -> {
                        monkeyList.get(action.destinationMonkey).receive(action.value);
                    });
                }
            }

            monkeyList.sort((m1, m2) -> m2.inspectCount - m1.inspectCount);
            System.out.println("monkeyList = " + monkeyList);
            System.out.println("getPart1(monkeyList) = " + getPart1(monkeyList));
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public static int getPart1(List<Monkey> monkeyList) {
        return monkeyList.get(0).inspectCount * monkeyList.get(1).inspectCount;
    }

    public static List<Monkey> processInput(String input) {
        return Arrays.stream(input.split("\n\n")).map(str -> {
            Iterator<String> iterator = str.lines().iterator();
            iterator.next();
            List<Integer> items = Arrays.stream(iterator.next().split(": ")[1].split(", "))
                    .map(Integer::valueOf)
                    .collect(Collectors.toList());

            String[] expression = iterator.next().split("= old ")[1].split(" ");
            String operator = expression[0];

            Closure closure = (number) -> {
                int operand = expression[1].equals("old") ? number : Integer.parseInt(expression[1]);
                int result;
                switch (operator) {
                    case "+" -> result = number + operand;

                    case "-" -> result = number - operand;

                    case "*" -> result = number * operand;

                    case "/" -> result = number / operand;

                    default -> result = 0;
                }

                return result / 3;
            };

            String[] line = iterator.next().split(" ");
            int divisible = Integer.parseInt(line[line.length - 1]);
            line = iterator.next().split(" ");
            int trueIndex = Integer.parseInt(line[line.length - 1]);
            line = iterator.next().split(" ");
            int falseIndex = Integer.parseInt(line[line.length - 1]);

            return new Monkey(items, divisible, trueIndex, falseIndex, closure);
        }).collect(Collectors.toList());
    }

    static class Monkey {
        List<Integer> items;
        int inspectCount;
        int divisible;
        int trueIndex;
        int falseIndex;
        Closure closure;

        public Monkey(List<Integer> items, int divisible, int trueIndex, int falseIndex, Closure closure) {
            this.items = items;
            this.divisible = divisible;
            this.trueIndex = trueIndex;
            this.falseIndex = falseIndex;
            this.closure = closure;
            inspectCount = 0;
        }

        public Action throwItem(int item) {
            int result = closure.operate(item);
            int destination = result % divisible == 0 ? trueIndex : falseIndex;
            inspectCount++;
            return new Action(destination, result);
        }

        public void receive(int item) {
            items.add(item);
        }

        public List<Action> takeTurn() {
            List<Action> actions = new ArrayList<>();
            for (int item : items) {
                actions.add(throwItem(item));
            }
            items.clear();
            return actions;
        }

        @Override
        public String toString() {
            return "Monkey{" +
                    "items=" + items +
                    ", inspectCount=" + inspectCount +
                    '}';
        }
    }

    static class Action {
        int destinationMonkey;
        int value;

        public Action(int destinationMonkey, int value) {
            this.destinationMonkey = destinationMonkey;
            this.value = value;
        }
    }

    @FunctionalInterface
    public interface Closure {
        int operate(int input);
    }
}
