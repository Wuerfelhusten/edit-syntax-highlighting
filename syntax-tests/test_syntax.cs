// C# Syntax Test File
// Testing C# syntax highlighting with various language features

using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using System.Text;

#nullable enable

namespace MyApp.Models
{
    // Attribute usage
    [Serializable]
    public class Person
    {
        // Auto-implemented properties
        public string Name { get; set; } = string.Empty;
        public int Age { get; set; }
        public double Salary { get; private set; }
        
        // Expression-bodied member
        public string FullInfo => $"{Name} is {Age} years old";
        
        // Constructor
        public Person(string name, int age)
        {
            Name = name;
            Age = age;
        }
        
        // Method
        public void UpdateSalary(double newSalary)
        {
            Salary = newSalary;
        }
        
        // Virtual method
        public virtual void Display()
        {
            Console.WriteLine($"Name: {Name}, Age: {Age}");
        }
    }
    
    // Derived class
    public class Employee : Person
    {
        public string Department { get; set; } = string.Empty;
        
        public Employee(string name, int age, string department) 
            : base(name, age)
        {
            Department = department;
        }
        
        public override void Display()
        {
            base.Display();
            Console.WriteLine($"Department: {Department}");
        }
    }
    
    // Interface
    public interface IRepository<T>
    {
        Task<T?> GetByIdAsync(int id);
        Task<IEnumerable<T>> GetAllAsync();
        Task AddAsync(T item);
        Task UpdateAsync(T item);
        Task DeleteAsync(int id);
    }
    
    // Generic class
    public class Repository<T> : IRepository<T> where T : class
    {
        private readonly List<T> _items = new();
        
        public async Task<T?> GetByIdAsync(int id)
        {
            await Task.Delay(10);
            return _items.Count > id ? _items[id] : null;
        }
        
        public async Task<IEnumerable<T>> GetAllAsync()
        {
            await Task.Delay(10);
            return _items;
        }
        
        public async Task AddAsync(T item)
        {
            await Task.Delay(10);
            _items.Add(item);
        }
        
        public async Task UpdateAsync(T item)
        {
            await Task.Delay(10);
        }
        
        public async Task DeleteAsync(int id)
        {
            await Task.Delay(10);
            if (id < _items.Count)
            {
                _items.RemoveAt(id);
            }
        }
    }
    
    // Record type (C# 9.0)
    public record PersonRecord(string Name, int Age);
    
    // Record with methods
    public record EmployeeRecord(string Name, int Age, string Department)
    {
        public string GetInfo() => $"{Name} works in {Department}";
    }
    
    // Enum
    public enum Status
    {
        Pending = 0,
        Active = 1,
        Completed = 2,
        Cancelled = 3
    }
    
    // Struct
    public struct Point
    {
        public int X { get; set; }
        public int Y { get; set; }
        
        public Point(int x, int y)
        {
            X = x;
            Y = y;
        }
        
        public readonly double DistanceFromOrigin()
        {
            return Math.Sqrt(X * X + Y * Y);
        }
    }
}

namespace MyApp
{
    class Program
    {
        static async Task Main(string[] args)
        {
            // Number literals
            int decimal = 42;
            int hex = 0xFF;
            int binary = 0b1010_1011;
            long bigNum = 1234567890L;
            ulong unsignedBig = 1234567890UL;
            float pi = 3.14159f;
            double e = 2.718281828;
            decimal money = 99.99m;
            
            // String literals
            string str = "Hello, C#!";
            string verbatim = @"C:\Users\Name\Documents";
            string interpolated = $"The answer is {decimal}";
            string verbatimInterpolated = $@"Path: C:\Users\{Environment.UserName}";
            
            // Raw string literal (C# 11)
            string json = """
                {
                    "name": "John Doe",
                    "age": 30
                }
                """;
            
            // Character literal
            char ch = 'A';
            char escape = '\n';
            char unicode = '\u0041';
            
            // Boolean and null
            bool flag = true;
            bool success = false;
            string? nullableString = null;
            int? nullableInt = null;
            
            // Array initialization
            int[] numbers = { 1, 2, 3, 4, 5 };
            string[] names = new string[] { "Alice", "Bob", "Charlie" };
            int[,] matrix = new int[3, 3];
            
            // Collection initialization
            var list = new List<int> { 1, 2, 3, 4, 5 };
            var dict = new Dictionary<string, int>
            {
                ["one"] = 1,
                ["two"] = 2,
                ["three"] = 3
            };
            
            // Object initialization
            var person = new Models.Person("Alice", 25)
            {
                Salary = 50000
            };
            
            // If-else statement
            if (decimal > 40)
            {
                Console.WriteLine("Greater than 40");
            }
            else if (decimal > 30)
            {
                Console.WriteLine("Greater than 30");
            }
            else
            {
                Console.WriteLine("30 or less");
            }
            
            // Switch statement
            switch (decimal)
            {
                case 0:
                    Console.WriteLine("Zero");
                    break;
                case 42:
                    Console.WriteLine("The answer");
                    break;
                default:
                    Console.WriteLine("Other number");
                    break;
            }
            
            // Switch expression (C# 8.0)
            var description = decimal switch
            {
                0 => "Zero",
                42 => "The answer",
                _ => "Other number"
            };
            
            // Pattern matching
            if (person is Models.Person p && p.Age > 18)
            {
                Console.WriteLine($"{p.Name} is an adult");
            }
            
            // For loops
            for (int i = 0; i < 10; i++)
            {
                Console.Write($"{i} ");
            }
            Console.WriteLine();
            
            // Foreach loop
            foreach (var num in numbers)
            {
                Console.Write($"{num} ");
            }
            Console.WriteLine();
            
            // While loop
            int count = 0;
            while (count < 5)
            {
                count++;
            }
            
            // Do-while loop
            do
            {
                count--;
            } while (count > 0);
            
            // LINQ queries
            var evenNumbers = numbers.Where(n => n % 2 == 0).ToList();
            var sum = numbers.Sum();
            var average = numbers.Average();
            
            // LINQ query syntax
            var query = from n in numbers
                        where n > 2
                        orderby n descending
                        select n * 2;
            
            // Lambda expressions
            Func<int, int, int> add = (x, y) => x + y;
            Action<string> print = msg => Console.WriteLine(msg);
            
            // Async/await
            await ProcessDataAsync();
            
            // Exception handling
            try
            {
                throw new InvalidOperationException("Test exception");
            }
            catch (InvalidOperationException ex)
            {
                Console.WriteLine($"Caught: {ex.Message}");
            }
            catch (Exception ex) when (ex.Message.Contains("Test"))
            {
                Console.WriteLine("Filtered catch");
            }
            finally
            {
                Console.WriteLine("Cleanup");
            }
            
            // Null-coalescing operators
            string? maybeNull = null;
            string notNull = maybeNull ?? "default value";
            int? nullableValue = null;
            int value = nullableValue ?? 0;
            
            // Null-conditional operator
            string? length = maybeNull?.Length.ToString();
            
            // Tuple
            var tuple = (Name: "Alice", Age: 25);
            Console.WriteLine($"{tuple.Name} is {tuple.Age}");
            
            // Deconstruction
            var (name, age) = tuple;
            
            // Local function
            int LocalAdd(int a, int b)
            {
                return a + b;
            }
            
            int result = LocalAdd(5, 3);
            
            // Using declaration (C# 8.0)
            using var stream = new System.IO.MemoryStream();
            
            // Target-typed new (C# 9.0)
            Models.Point point = new(10, 20);
            
            // Init-only properties (C# 9.0)
            var record = new Models.PersonRecord("Bob", 30);
            
            // With expression (C# 9.0)
            var modified = record with { Age = 31 };
            
            Console.WriteLine("Program completed");
        }
        
        static async Task ProcessDataAsync()
        {
            await Task.Delay(100);
            Console.WriteLine("Data processed asynchronously");
        }
    }
    
    // Extension methods
    public static class StringExtensions
    {
        public static string Reverse(this string str)
        {
            char[] chars = str.ToCharArray();
            Array.Reverse(chars);
            return new string(chars);
        }
        
        public static bool IsNullOrEmpty(this string? str)
        {
            return string.IsNullOrEmpty(str);
        }
    }
}
