import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;

public class Day21B {

    final static String humn = "humn";
    final static String root = "root";

    public static void main(String[] args) {
        solvePart2();
    }

    public static void solvePart2() {
//        String path = "aoc/2022/day21-test-rename.txt";
        String path = "aoc/2022/day21.txt";
        Info info = processInput(path);
        System.out.println("part2");

        while (true) {
            for (Monkey m : info.monkeys.values()) {
                System.out.println("current: " + m.name);
                Monkey m1 = info.monkeys.get(m.dep1);
                Monkey m2 = info.monkeys.get(m.dep2);
                if (!m.isCalced && !m.name.equals(humn)) {
                    // try to calculate current monkey if we have info for both m1 and m2
                    if (m1.isCalced && m2.isCalced) {
                        long value = m.calculate(m1.value, m2.value);
                        m.isCalced = true;
                        info.monkeyNumber.put(m.name, value);
                        System.out.println("Calculated [" + m.name + "]: " + m.value);
                    }
                }

                if (m.isCalced && m.type == YellType.OPERATION) {
                    // both monkeys are known
                    // or both monkeys are not known
                    if ((m1.isCalced && m2.isCalced) || (!m1.isCalced && !m2.isCalced)) continue;

                    if (!m1.isCalced) {
                        // know m2 only
                        m1.value = m.getMonkey1(m2.value);
                        m1.isCalced = true;
                        info.monkeyNumber.put(m1.name, m1.value);
                        System.out.println("Calculated [" + m1.name + "]: " + m1.value);
                    } else {
                        // know m1 only
                        m2.value = m.getMonkey2(m1.value);
                        m2.isCalced = true;
                        info.monkeyNumber.put(m2.name, m2.value);
                        System.out.println("Calculated [" + m2.name + "]: " + m2.value);
                    }
                }

                if (info.monkeyNumber.containsKey(humn)) {
                    System.out.println("humn = " + info.monkeyNumber.get(humn));
                    return;
                }
            }

        }
    }

    public static Info processInput(String path) {
        Info info = new Info();
        try {
            List<String> lines = Files.readAllLines(Paths.get(path));
            for (String line : lines) {
                String[] lineInput = line.split(": ");
                String name = lineInput[0];
                Monkey monkey;
                if (Character.isDigit(lineInput[1].charAt(0))) {
                    // number monkey
                    long value = Long.parseLong(lineInput[1]);
                    monkey = new Monkey(name, true, YellType.NUMBER, value);
                    if (name.equals(humn)) {
                        monkey.isCalced = false;
                    } else {
                        info.monkeyNumber.put(monkey.name, value);
                    }
                } else {
                    // operation monkey
                    String[] expression = lineInput[1].split(" ");
                    monkey = new Monkey(name, false, YellType.OPERATION, expression[1].charAt(0), expression[0], expression[2]);
                    if (name.equals(root)) {
                        monkey.op = '=';
                        monkey.isCalced = true;
                    }
                }
                info.monkeys.put(monkey.name, monkey);
            }

        } catch (Exception e) {
            e.printStackTrace();
        }
        return info;
    }

    static class Monkey {
        String name;
        long value;
        boolean isCalced;
        YellType type;
        char op;
        String dep1;
        String dep2;

        // Operation
        public Monkey(String name, boolean isCalced, YellType type, char op, String dep1, String dep2) {
            this.name = name;
            this.isCalced = isCalced;
            this.type = type;
            this.op = op;
            this.dep1 = dep1;
            this.dep2 = dep2;
        }

        // Number
        public Monkey(String name, boolean isCalced, YellType type, long value) {
            this.name = name;
            this.isCalced = isCalced;
            this.type = type;
            this.value = value;
        }

        public long calculate(long monkey1, long monkey2) {
            switch (op) {
                case '+' -> value = monkey1 + monkey2;
                case '-' -> value = monkey1 - monkey2;
                case '*' -> value = monkey1 * monkey2;
                case '/' -> value = monkey1 / monkey2;
                case '=' -> value = monkey1;
                default -> throw new IllegalArgumentException("Invalid op. This shouldn't be called.");
            }
            return value;
        }

        public long getMonkey1(long monkey2) {
            long result;
            switch (op) {
                case '+' -> result = value - monkey2;
                case '-' -> result = value + monkey2;
                case '*' -> result = value / monkey2;
                case '/' -> result = value * monkey2;
                case '=' -> result = monkey2;
                default -> throw new IllegalArgumentException("Invalid op. This shouldn't be called.");
            }

            return result;
        }

        public long getMonkey2(long monkey1) {
            long result;
            switch (op) {
                case '+' -> result = value - monkey1;
                case '-' -> result = monkey1 - value;
                case '*' -> result = value / monkey1;
                case '/' -> result = monkey1 / value;
                case '=' -> result = monkey1;
                default -> throw new IllegalArgumentException("Invalid op. This shouldn't be called.");
            }

            return result;
        }
    }

    enum YellType {
        NUMBER, OPERATION
    }

    static class Info {
        Map<String, Monkey> monkeys;
        Map<String, Long> monkeyNumber;

        public Info() {
            monkeys = new HashMap<>();
            monkeyNumber = new HashMap<>();
        }
    }
}
