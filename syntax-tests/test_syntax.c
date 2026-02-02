// C Syntax Test File
// Testing C syntax highlighting with various language features

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>

#define MAX_SIZE 1024
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define DEBUG_PRINT(fmt, ...) \
    fprintf(stderr, fmt, ##__VA_ARGS__)

// Type definitions
typedef struct {
    char *name;
    int age;
    double salary;
} Person;

typedef enum {
    STATUS_OK = 0,
    STATUS_ERROR = -1,
    STATUS_PENDING = 1
} Status;

// Function declarations
void process_data(const char *input, size_t len);
int calculate_sum(int *array, size_t count);
Person *create_person(const char *name, int age);

// Global variables
static int global_counter = 0;
const char *APP_NAME = "TestApp";
volatile bool is_running = true;

/* Multi-line comment
 * demonstrating block comments
 * with multiple lines
 */

// Number literals
int decimal = 42;
int hex = 0x2A;
int octal = 052;
int binary = 0b101010;  // C23
unsigned long big_num = 1234567890UL;
float pi = 3.14159f;
double e = 2.718281828;

// Character and string literals
char ch = 'A';
char escape = '\n';
char hex_char = '\x41';
const char *message = "Hello, World!";
const char *multiline = "This is a \
long string that spans \
multiple lines";

// Boolean and NULL
bool flag = true;
bool success = false;
void *ptr = NULL;

// Control flow
int main(int argc, char *argv[]) {
    // If-else statement
    if (argc > 1) {
        printf("Arguments provided: %d\n", argc - 1);
    } else {
        printf("No arguments\n");
    }
    
    // For loop
    for (int i = 0; i < 10; i++) {
        printf("%d ", i);
    }
    printf("\n");
    
    // While loop
    int count = 0;
    while (count < 5) {
        count++;
    }
    
    // Do-while loop
    do {
        count--;
    } while (count > 0);
    
    // Switch statement
    switch (argc) {
        case 1:
            printf("One argument\n");
            break;
        case 2:
            printf("Two arguments\n");
            break;
        default:
            printf("Many arguments\n");
            break;
    }
    
    // Pointer operations
    int value = 42;
    int *ptr = &value;
    int deref = *ptr;
    
    // Array operations
    int numbers[] = {1, 2, 3, 4, 5};
    size_t array_size = sizeof(numbers) / sizeof(numbers[0]);
    
    // Dynamic memory allocation
    Person *person = (Person *)malloc(sizeof(Person));
    if (person != NULL) {
        person->name = "John Doe";
        person->age = 30;
        person->salary = 75000.50;
        free(person);
    }
    
    // Bitwise operations
    unsigned int mask = 0xFF00;
    unsigned int result = value & mask;
    result = result | 0x00FF;
    result ^= 0xFFFF;
    result = ~result;
    result = result << 2;
    result = result >> 1;
    
    // Ternary operator
    int max = (value > 100) ? value : 100;
    
    // Goto and labels
    if (value < 0) {
        goto error_handler;
    }
    
    return 0;
    
error_handler:
    fprintf(stderr, "Error occurred\n");
    return 1;
}

// Function implementations
void process_data(const char *input, size_t len) {
    for (size_t i = 0; i < len; i++) {
        putchar(input[i]);
    }
}

int calculate_sum(int *array, size_t count) {
    int sum = 0;
    for (size_t i = 0; i < count; i++) {
        sum += array[i];
    }
    return sum;
}

Person *create_person(const char *name, int age) {
    Person *p = malloc(sizeof(Person));
    if (p) {
        p->name = strdup(name);
        p->age = age;
        p->salary = 0.0;
    }
    return p;
}

// C11 features
_Static_assert(sizeof(int) >= 4, "int must be at least 4 bytes");

// C23 features (if supported)
typeof(5) number = 10;
_BitInt(128) large_integer = 0;
