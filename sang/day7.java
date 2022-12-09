
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day7 {
    private static final long LIMIT = 100_000;
    public static void main(String[] args) {
        try {
            List<String> lines = Files.readAllLines(Paths.get("day7.txt"));
            FileNode root = new FileNode("/", null);
            FileNode curr = root;
            for (String line : lines) {
                String[] strs = line.split(" ");
                if (strs[0].equals("$") && strs[1].equals("cd")) {
                    if (strs[2].equals("/")) {
                        curr = root;
                    } else if (strs[2].equals("..")) {
                        if (curr != root) {
                            curr = curr.parent;
                        }
                    } else {
                        curr = curr.children.get(strs[2]);
                    }
                } else if (Character.isDigit(strs[0].charAt(0))) {
                    FileNode file = new FileNode(strs[1], Long.parseLong(strs[0]), curr);
                    curr.children.put(strs[1], file);
                } else if (strs[0].equals("dir")) {
                    FileNode folder = new FileNode(strs[1], curr);
                    curr.children.put(strs[1], folder);
                }
            }

            long part1 = part1(root);
            System.out.println("part1 = " + part1);
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public static long part1(FileNode root) {
        List<FileNode> result = new ArrayList<>();
        dfs(root, result);
        System.out.println("result = " + result);
        return result
                .stream()
                .map(file -> file.size)
                .reduce(Long::sum)
                .orElse(0L);
    }

    private static long dfs(FileNode curr, List<FileNode> smallDirectories) {
        if (!curr.isFolder()) {
            return curr.size;
        }

        long totalSize = 0;
        for (FileNode child : curr.children.values()) {
            totalSize += dfs(child, smallDirectories);
        }

        curr.size= totalSize;

        if (curr.size <= LIMIT) {
            smallDirectories.add(curr);
            System.out.println("curr = " + curr);
        }

        return curr.size;
    }

    static class FileNode {
        long size;
        String name;
        Map<String, FileNode> children;
        FileNode parent;

        public FileNode(String name, FileNode parent) {
            size = 0;
            this.name = name;
            children = new HashMap<>();
            this.parent = parent;
        }

        public FileNode(String name, long size, FileNode parent) {
            this.size = size;
            this.name = name;
            children = null;
            this.parent = parent;
        }

        public boolean isFolder() {
            return this.children != null;
        }
    }
}
