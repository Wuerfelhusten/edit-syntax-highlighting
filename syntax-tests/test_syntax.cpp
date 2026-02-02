// C++ Syntax Test File
// Testing C++ syntax highlighting with various language features

#include <iostream>
#include <vector>
#include <string>
#include <memory>
#include <algorithm>
#include <map>
#include <optional>

// Namespace
namespace myapp {
    namespace utils {
        constexpr int MAX_SIZE = 1024;
    }
}

using namespace std;
using myapp::utils::MAX_SIZE;

// Class definition
class Person {
private:
    string name_;
    int age_;
    double salary_;
    
public:
    // Constructors
    Person() : name_(""), age_(0), salary_(0.0) {}
    
    Person(const string& name, int age) 
        : name_(name), age_(age), salary_(0.0) {}
    
    // Copy constructor
    Person(const Person& other) = default;
    
    // Move constructor
    Person(Person&& other) noexcept = default;
    
    // Destructor
    virtual ~Person() = default;
    
    // Getters
    const string& getName() const { return name_; }
    int getAge() const { return age_; }
    
    // Setters
    void setName(const string& name) { name_ = name; }
    void setAge(int age) { age_ = age; }
    
    // Virtual function
    virtual void display() const {
        cout << "Name: " << name_ << ", Age: " << age_ << endl;
    }
    
    // Static method
    static Person createDefault() {
        return Person("Unknown", 0);
    }
    
    // Operator overloading
    bool operator==(const Person& other) const {
        return name_ == other.name_ && age_ == other.age_;
    }
};

// Derived class
class Employee : public Person {
private:
    string department_;
    
public:
    Employee(const string& name, int age, const string& dept)
        : Person(name, age), department_(dept) {}
    
    void display() const override {
        Person::display();
        cout << "Department: " << department_ << endl;
    }
};

// Template class
template<typename T>
class Container {
private:
    vector<T> data_;
    
public:
    void add(const T& item) {
        data_.push_back(item);
    }
    
    void add(T&& item) {
        data_.push_back(move(item));
    }
    
    size_t size() const { return data_.size(); }
    
    T& operator[](size_t index) { return data_[index]; }
    const T& operator[](size_t index) const { return data_[index]; }
};

// Template function
template<typename T>
T max_value(T a, T b) {
    return (a > b) ? a : b;
}

// C++11 features
auto lambda = [](int x, int y) -> int {
    return x + y;
};

auto generic_lambda = [](auto x, auto y) {
    return x * y;
};

// C++14 binary literals
int binary = 0b1010'1011;
int hex = 0xDEAD'BEEF;

// C++17 structured bindings
pair<int, string> get_pair() {
    return {42, "answer"};
}

// C++20 concepts (if supported)
template<typename T>
concept Numeric = is_arithmetic_v<T>;

template<Numeric T>
T add(T a, T b) {
    return a + b;
}

// Raw string literal
const char* json = R"({
    "name": "John Doe",
    "age": 30,
    "active": true
})";

// Main function
int main() {
    // Number literals
    int decimal = 42;
    int hex_num = 0xFF;
    double pi = 3.14159;
    float small = 1.5f;
    long long big = 1234567890LL;
    
    // String literals
    string str = "Hello, C++!";
    string raw = R"(This is a
multiline string
with "quotes")";
    
    // Boolean and nullptr
    bool flag = true;
    bool success = false;
    int* ptr = nullptr;
    
    // Smart pointers
    unique_ptr<Person> person1 = make_unique<Person>("Alice", 25);
    shared_ptr<Person> person2 = make_shared<Person>("Bob", 30);
    weak_ptr<Person> person3 = person2;
    
    // STL containers
    vector<int> numbers = {1, 2, 3, 4, 5};
    map<string, int> ages = {{"Alice", 25}, {"Bob", 30}};
    optional<int> maybe_value = 42;
    
    // Range-based for loop
    for (const auto& num : numbers) {
        cout << num << " ";
    }
    cout << endl;
    
    // Algorithm with lambda
    auto it = find_if(numbers.begin(), numbers.end(), 
                      [](int x) { return x > 3; });
    
    if (it != numbers.end()) {
        cout << "Found: " << *it << endl;
    }
    
    // Structured binding (C++17)
    auto [value, text] = get_pair();
    cout << "Value: " << value << ", Text: " << text << endl;
    
    // Exception handling
    try {
        throw runtime_error("Test exception");
    } catch (const exception& e) {
        cerr << "Caught exception: " << e.what() << endl;
    }
    
    // Type casting
    double d = 3.14;
    int i = static_cast<int>(d);
    void* void_ptr = static_cast<void*>(&i);
    
    // Template usage
    Container<int> int_container;
    int_container.add(10);
    int_container.add(20);
    
    // Lambda with capture
    int x = 10;
    auto capture_lambda = [x](int y) { return x + y; };
    int result = capture_lambda(5);
    
    // Move semantics
    string s1 = "Hello";
    string s2 = move(s1);  // s1 is now empty
    
    // constexpr
    constexpr int compile_time = 100;
    
    // decltype
    decltype(compile_time) same_type = 200;
    
    // Type alias
    using IntVector = vector<int>;
    IntVector vec = {1, 2, 3};
    
    // Namespace usage
    cout << "Max size: " << myapp::utils::MAX_SIZE << endl;
    
    return 0;
}

// Function template specialization
template<>
int max_value<int>(int a, int b) {
    return (a > b) ? a : b;
}

// Operator overloading (free function)
ostream& operator<<(ostream& os, const Person& p) {
    os << p.getName() << " (" << p.getAge() << ")";
    return os;
}
