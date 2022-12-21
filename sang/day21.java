
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;
import java.util.function.ToLongBiFunction;

public class Day21A {

    final static int MAX_DEPTH = 999_999;

    public static void main(String[] args) {
        solvePart1();
    }

    public static void solvePart1() {
        Graph graph = processInput("aoc/2022/day21.txt");
        Set<String> visited = new HashSet<>();

        Queue<Node> q = new LinkedList<>();
        for (String numNode : graph.numberNodes) {
            visited.add(numNode);
            for (String neighbor : graph.matrix.getOrDefault(numNode, new ArrayList<>())) {
                q.offer(graph.map.get(neighbor));
            }
        }

        while (!q.isEmpty()) {
            Node node = q.poll();

            if (visited.contains(node.name)) continue;

            Node c1 = graph.map.get(node.operands[0]);
            Node c2 = graph.map.get(node.operands[1]);
            int maxDepth = Math.max(c1.depth, c2.depth);
            node.depth = maxDepth + 1;

            if (!visited.contains(c1.name) || !visited.contains(c2.name)) {
                q.add(node);
                continue;
            }

            visited.add(node.name);
            node.value = node.consumer.applyAsLong(c1.value, c2.value);

            for (String child : graph.matrix.getOrDefault(node.name, new ArrayList<>())) {
                if (!visited.contains(child)) {
                    q.offer(graph.map.get(child));
                }
            }
        }

        System.out.println("result: " + graph.map.get("root").value);
    }

    public static Graph processInput(String path) {
        Graph graph = new Graph();
        try {
            List<String> lines = Files.readAllLines(Paths.get(path));
            for (String line : lines) {
                String[] strings = line.split(": ");
                String current = strings[0];
                Node node;
                if (Character.isDigit(strings[1].charAt(0))) {
                    node = new Node(current, Integer.parseInt(strings[1]));
                    graph.numberNodes.add(current);
                } else {
                    String[] operation = strings[1].split(" ");
                    String p1 = operation[0];
                    String p2 = operation[2];

                    node = new Node(current, operation[1], new String[]{p1, p2});

                    List<String> currentChildren = graph.matrix.getOrDefault(current, new ArrayList<>());
                    currentChildren.add(p1);
                    currentChildren.add(p2);
                    graph.matrix.put(current, currentChildren);

                    List<String> neighbor1 = graph.matrix.getOrDefault(p1, new ArrayList<>());
                    neighbor1.add(current);
                    graph.matrix.put(p1, neighbor1);

                    List<String> neighbor2 = graph.matrix.getOrDefault(p2, new ArrayList<>());
                    neighbor2.add(current);
                    graph.matrix.put(p2, neighbor2);
                }
                graph.map.put(strings[0], node);
            }

        } catch (Exception e) {
            e.printStackTrace();
        }
        return graph;
    }

    static class Graph {
        Map<String, Node> map;
        Map<String, List<String>> matrix;
        List<String> numberNodes;

        public Graph() {
            map = new HashMap<>();
            matrix = new HashMap<>();
            numberNodes = new ArrayList<>();
        }
    }

    static class Node {
        String name;
        long value;
        ToLongBiFunction<Long, Long> consumer;
        String[] operands;
        int depth;

        public Node(String name, long value) {
            this.name = name;
            this.value = value;
            depth = 0;
        }

        public Node(String name, String operator, String[] operands) {
            this.name = name;
            consumer = (i1, i2) -> switch (operator) {
                case "+" -> i1 + i2;
                case "-" -> i1 - i2;
                case "*" -> i1 * i2;
                case "/" -> i1 / i2;
                default -> Integer.MIN_VALUE;
            };
            this.operands = operands;
            depth = MAX_DEPTH;
        }
    }
}
