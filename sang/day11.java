import java.math.BigInteger;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Iterator;
import java.util.List;
import java.util.stream.Collectors;

public class Day11 {
    static final int ROUNDS = 20;
    static final int ROUNDS_2 = 10_000;

    static long lcm;

    public static void main(String[] args) {
        try {
            String input = Files.readString(Path.of("aoc/2022/day11.txt"));
            List<Monkey> monkeyList = processInput(input, false);
            lcm = 1;
            for (Monkey monkey: monkeyList) {
                lcm *= monkey.divisible;
            }

//            System.out.println("part 1 = " + getPart1(monkeyList));
            System.out.println("part 2 = " + getPart2(monkeyList));
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public static BigInteger getPart2(List<Monkey> monkeyList) {
        for (int i = 0; i < ROUNDS_2; i++) {
            for (Monkey monkey : monkeyList) {
                monkey.takeTurn().forEach(action -> {
                    monkeyList.get(action.destinationMonkey).receive(action.value);
                });
            }
            if (i == 0 || i == 20 || (i + 1) % 1000 == 0) {
                String s = String.format("0, 1, 2, 3 - %s, %s, %s, %s",
                        monkeyList.get(0),
                        monkeyList.get(1),
                        monkeyList.get(2),
                        monkeyList.get(3)
                );
                System.out.println(s);
            }
        }

        monkeyList.sort((m1, m2) -> Long.compare(m2.inspectCount, m1.inspectCount));
        System.out.println(monkeyList.get(0) + ", " + monkeyList.get(1));
        return BigInteger.valueOf(monkeyList.get(0).inspectCount)
                .multiply(BigInteger.valueOf(monkeyList.get(1).inspectCount));
    }

    public static long getPart1(List<Monkey> monkeyList) {
        for (int i = 0; i < ROUNDS; i++) {
            for (Monkey monkey : monkeyList) {
                monkey.takeTurn().forEach(action -> {
                    monkeyList.get(action.destinationMonkey).receive(action.value);
                });
            }
        }

        monkeyList.sort((m1, m2) -> Long.compare(m2.inspectCount, m1.inspectCount));
        return monkeyList.get(0).inspectCount * monkeyList.get(1).inspectCount;
    }

    public static List<Monkey> processInput(String input, boolean isPart1) {
        return Arrays.stream(input.split("\n\n")).map(str -> {
            Iterator<String> iterator = str.lines().iterator();
            iterator.next();
            List<Long> items = Arrays.stream(iterator.next().split(": ")[1].split(", "))
                    .map(Long::valueOf)
                    .collect(Collectors.toList());

            String[] expression = iterator.next().split("= old ")[1].split(" ");
            String operator = expression[0];

            Closure closure = (number) -> {
                long operand = expression[1].equals("old") ? number : Long.parseLong(expression[1]);
                long result;
                switch (operator) {
                    case "+" -> result = number + operand;

                    case "-" -> result = number - operand;

                    case "*" -> result = number * operand;

                    case "/" -> result = number / operand;

                    default -> result = 0;
                }

                return isPart1 ? result / 3 : result;
            };

            String[] line = iterator.next().split(" ");
            long divisible = Long.parseLong(line[line.length - 1]);
            line = iterator.next().split(" ");
            int trueIndex = Integer.parseInt(line[line.length - 1]);
            line = iterator.next().split(" ");
            int falseIndex = Integer.parseInt(line[line.length - 1]);

            return new Monkey(items, divisible, trueIndex, falseIndex, closure);
        }).collect(Collectors.toList());
    }

    static class Monkey {
        List<Long> items;
        long inspectCount;
        long divisible;
        int trueIndex;
        int falseIndex;
        Closure closure;

        public Monkey(List<Long> items, long divisible, int trueIndex, int falseIndex, Closure closure) {
            this.items = items;
            this.divisible = divisible;
            this.trueIndex = trueIndex;
            this.falseIndex = falseIndex;
            this.closure = closure;
            inspectCount = 0;
        }

        public Action throwItem(long item) {
            long result = closure.operate(item);
            int destination = result % divisible == 0 ? trueIndex : falseIndex;
            inspectCount++;
            return new Action(destination, result);
        }

        public void receive(long item) {
//            items.add(item % divisible);
            items.add(item % lcm);
        }

        public List<Action> takeTurn() {
            List<Action> actions = new ArrayList<>();
            for (long item : items) {
                actions.add(throwItem(item));
            }
            items.clear();
            return actions;
        }

        @Override
        public String toString() {
            return "Monkey{" +
                    "inspectCount=" + inspectCount +
                    '}';
        }
    }

    static class Action {
        int destinationMonkey;
        long value;

        public Action(int destinationMonkey, long value) {
            this.destinationMonkey = destinationMonkey;
            this.value = value;
        }
    }

    @FunctionalInterface
    public interface Closure {
        long operate(long input);
    }
}
