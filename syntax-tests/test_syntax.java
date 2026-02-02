// Java Syntax Test File
// Testing Java syntax highlighting with various language features

package com.example.myapp;

import java.util.*;
import java.io.*;
import java.util.stream.*;
import java.util.concurrent.*;

/**
 * Javadoc comment for the main class.
 * This demonstrates Java syntax highlighting.
 * 
 * @author Test Author
 * @version 1.0
 */
public class SyntaxTest {
    
    // Class-level constants
    private static final int MAX_SIZE = 1024;
    private static final String APP_NAME = "TestApp";
    public static final double PI = 3.14159;
    
    // Instance variables
    private String name;
    private int age;
    protected double salary;
    public boolean isActive;
    
    // Annotations
    @Override
    @Deprecated
    @SuppressWarnings("unchecked")
    public String toString() {
        return String.format("Person: %s, Age: %d", name, age);
    }
    
    // Constructor
    public SyntaxTest(String name, int age) {
        this.name = name;
        this.age = age;
        this.isActive = true;
    }
    
    // Number literals
    public void testNumbers() {
        // Decimal
        int decimal = 42;
        long bigNum = 1234567890L;
        
        // Hex
        int hex = 0xFF;
        int hexLong = 0xDEADBEEFL;
        
        // Binary (Java 7+)
        int binary = 0b1010_1011;
        
        // Octal
        int octal = 077;
        
        // Floating point
        float pi = 3.14159f;
        double e = 2.718281828;
        double scientific = 1.23e10;
        float scientificFloat = 4.56e-7f;
    }
    
    // String literals
    public void testStrings() {
        String str = "Hello, Java!";
        String escaped = "Line 1\nLine 2\tTabbed";
        String unicode = "Unicode: \u0041";
        
        // Text blocks (Java 15+)
        String json = """
            {
                "name": "John Doe",
                "age": 30,
                "active": true
            }
            """;
        
        String html = """
            <html>
                <body>
                    <h1>Title</h1>
                </body>
            </html>
            """;
    }
    
    // Character literals
    public void testChars() {
        char ch = 'A';
        char newline = '\n';
        char tab = '\t';
        char unicodeChar = '\u0041';
    }
    
    // Control flow
    public void testControlFlow(int value) {
        // If-else
        if (value > 100) {
            System.out.println("Greater than 100");
        } else if (value > 50) {
            System.out.println("Greater than 50");
        } else {
            System.out.println("50 or less");
        }
        
        // Switch statement
        switch (value) {
            case 0:
                System.out.println("Zero");
                break;
            case 1:
                System.out.println("One");
                break;
            default:
                System.out.println("Other");
                break;
        }
        
        // Switch expression (Java 14+)
        String result = switch (value) {
            case 0 -> "Zero";
            case 1 -> "One";
            default -> "Other";
        };
        
        // For loops
        for (int i = 0; i < 10; i++) {
            System.out.print(i + " ");
        }
        
        // Enhanced for loop
        int[] numbers = {1, 2, 3, 4, 5};
        for (int num : numbers) {
            System.out.print(num + " ");
        }
        
        // While loop
        int count = 0;
        while (count < 5) {
            count++;
        }
        
        // Do-while loop
        do {
            count--;
        } while (count > 0);
    }
    
    // Exception handling
    public void testExceptions() {
        try {
            throw new RuntimeException("Test exception");
        } catch (RuntimeException e) {
            System.err.println("Caught: " + e.getMessage());
        } catch (Exception e) {
            System.err.println("General exception");
        } finally {
            System.out.println("Cleanup");
        }
        
        // Try-with-resources
        try (BufferedReader reader = new BufferedReader(new FileReader("file.txt"))) {
            String line = reader.readLine();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
    
    // Generics
    public <T> T identity(T value) {
        return value;
    }
    
    public <T extends Comparable<T>> T max(T a, T b) {
        return a.compareTo(b) > 0 ? a : b;
    }
    
    // Lambda expressions (Java 8+)
    public void testLambdas() {
        // Simple lambda
        Runnable r = () -> System.out.println("Hello");
        
        // Lambda with parameters
        Comparator<Integer> comp = (a, b) -> a.compareTo(b);
        
        // Lambda with block
        Function<String, Integer> parser = (str) -> {
            return Integer.parseInt(str);
        };
        
        // Method reference
        List<String> names = Arrays.asList("Alice", "Bob", "Charlie");
        names.forEach(System.out::println);
    }
    
    // Stream API (Java 8+)
    public void testStreams() {
        List<Integer> numbers = Arrays.asList(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
        
        // Filter and map
        List<Integer> evenSquares = numbers.stream()
            .filter(n -> n % 2 == 0)
            .map(n -> n * n)
            .collect(Collectors.toList());
        
        // Reduce
        int sum = numbers.stream()
            .reduce(0, Integer::sum);
        
        // Count
        long count = numbers.stream()
            .filter(n -> n > 5)
            .count();
    }
    
    // Records (Java 14+)
    record Person(String name, int age) {
        // Compact constructor
        public Person {
            if (age < 0) {
                throw new IllegalArgumentException("Age cannot be negative");
            }
        }
        
        // Additional method
        public String getInfo() {
            return name + " is " + age + " years old";
        }
    }
    
    // Sealed classes (Java 17+)
    sealed interface Shape permits Circle, Rectangle, Triangle {
        double area();
    }
    
    record Circle(double radius) implements Shape {
        public double area() {
            return Math.PI * radius * radius;
        }
    }
    
    record Rectangle(double width, double height) implements Shape {
        public double area() {
            return width * height;
        }
    }
    
    record Triangle(double base, double height) implements Shape {
        public double area() {
            return 0.5 * base * height;
        }
    }
    
    // Pattern matching (Java 16+)
    public String formatShape(Shape shape) {
        return switch (shape) {
            case Circle c -> "Circle with radius " + c.radius();
            case Rectangle r -> "Rectangle " + r.width() + "x" + r.height();
            case Triangle t -> "Triangle with base " + t.base();
        };
    }
    
    // Var keyword (Java 10+)
    public void testVar() {
        var str = "Type inferred string";
        var number = 42;
        var list = new ArrayList<String>();
        var map = Map.of("key1", 1, "key2", 2);
    }
    
    // Inner classes
    public class InnerClass {
        private int value;
        
        public InnerClass(int value) {
            this.value = value;
        }
        
        public void display() {
            System.out.println("Inner class value: " + value);
        }
    }
    
    // Static nested class
    public static class StaticNested {
        private static int counter = 0;
        
        public static void increment() {
            counter++;
        }
    }
    
    // Anonymous class
    public void testAnonymousClass() {
        Runnable r = new Runnable() {
            @Override
            public void run() {
                System.out.println("Anonymous class");
            }
        };
        r.run();
    }
    
    // Main method
    public static void main(String[] args) {
        System.out.println("Starting Java Syntax Test");
        
        // Create instance
        SyntaxTest test = new SyntaxTest("Alice", 25);
        System.out.println(test);
        
        // Boolean literals
        boolean flag = true;
        boolean success = false;
        
        // Null
        String nullString = null;
        
        // Array creation
        int[] intArray = new int[10];
        String[] stringArray = {"one", "two", "three"};
        int[][] matrix = new int[3][3];
        
        // Collections
        List<String> list = new ArrayList<>();
        list.add("Item 1");
        list.add("Item 2");
        
        Map<String, Integer> map = new HashMap<>();
        map.put("key1", 100);
        map.put("key2", 200);
        
        Set<Integer> set = new HashSet<>();
        set.add(1);
        set.add(2);
        
        // Operators
        int a = 10, b = 20;
        int sum = a + b;
        int diff = a - b;
        int product = a * b;
        int quotient = b / a;
        int remainder = b % a;
        
        // Bitwise operators
        int bitwiseAnd = a & b;
        int bitwiseOr = a | b;
        int bitwiseXor = a ^ b;
        int bitwiseNot = ~a;
        int leftShift = a << 2;
        int rightShift = b >> 2;
        int unsignedRightShift = b >>> 2;
        
        // Logical operators
        boolean and = true && false;
        boolean or = true || false;
        boolean not = !true;
        
        // Comparison operators
        boolean equals = (a == b);
        boolean notEquals = (a != b);
        boolean greaterThan = (a > b);
        boolean lessThan = (a < b);
        boolean greaterOrEqual = (a >= b);
        boolean lessOrEqual = (a <= b);
        
        // Ternary operator
        int max = (a > b) ? a : b;
        
        // instanceof operator
        Object obj = "test";
        if (obj instanceof String) {
            String str = (String) obj;
            System.out.println("String length: " + str.length());
        }
        
        System.out.println("Java Syntax Test completed");
    }
}

// Interface
interface Drawable {
    void draw();
    default void print() {
        System.out.println("Printing drawable");
    }
}

// Enum
enum Color {
    RED, GREEN, BLUE;
    
    public String getHexCode() {
        return switch (this) {
            case RED -> "#FF0000";
            case GREEN -> "#00FF00";
            case BLUE -> "#0000FF";
        };
    }
}

// Abstract class
abstract class Animal {
    protected String name;
    
    public Animal(String name) {
        this.name = name;
    }
    
    public abstract void makeSound();
    
    public void sleep() {
        System.out.println(name + " is sleeping");
    }
}

// Concrete class extending abstract class
class Dog extends Animal {
    public Dog(String name) {
        super(name);
    }
    
    @Override
    public void makeSound() {
        System.out.println("Woof!");
    }
}
