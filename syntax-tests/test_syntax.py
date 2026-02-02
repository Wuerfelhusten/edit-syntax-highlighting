# Python Syntax Highlighting Demo

@decorator
def example_function(name: str, count: int = 0) -> dict:
    """
    Docstring with triple quotes
    """
    # Line comment
    result = {
        'name': name,
        'count': count,
        'enabled': True,
        'value': None,
    }
    
    # F-string
    message = f"Hello {name}, count is {count}"
    
    # Different number formats
    decimal = 42
    hex_num = 0xFF
    binary = 0b1010
    float_num = 3.14e-10
    
    for i in range(10):
        if i % 2 == 0:
            result['count'] += i
        elif i > 5:
            break
        else:
            continue
    
    return result

class ExampleClass:
    def __init__(self, name):
        self.name = name
    
    async def process(self):
        await asyncio.sleep(1)
        return self.name

if __name__ == "__main__":
    example = example_function("test", 10)
    print(example)
