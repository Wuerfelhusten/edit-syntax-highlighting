// JavaScript/TypeScript Syntax Highlighting Demo

/**
 * Multi-line comment
 * with documentation
 */

// Import statements
import { Component } from 'react';
import type { Props } from './types';

// Constants and variables
const API_URL = 'https://api.example.com';
let count = 0;
var legacy = true;

// Class definition
class Counter extends Component {
    constructor(props) {
        super(props);
        this.state = { count: 0 };
    }

    // Async method
    async fetchData() {
        try {
            const response = await fetch(API_URL);
            const data = await response.json();
            return data;
        } catch (error) {
            console.error('Error:', error);
            throw error;
        }
    }

    // Arrow function
    increment = () => {
        this.setState({ count: this.state.count + 1 });
    };
}

// Modern async/await
async function* generator() {
    yield 1;
    yield 2;
    yield 3;
}

// Template literals
const name = 'World';
const greeting = `Hello, ${name}!`;
const multiline = `
    This is a
    multi-line string
`;

// Destructuring
const { x, y } = { x: 10, y: 20 };
const [first, ...rest] = [1, 2, 3, 4, 5];

// Numbers in various formats
const decimal = 42;
const hex = 0xFF;
const binary = 0b1010;
const octal = 0o755;
const float = 3.14159;
const scientific = 1.5e-10;

// Conditional and loops
if (count > 0) {
    console.log('Positive');
} else if (count < 0) {
    console.log('Negative');
} else {
    console.log('Zero');
}

for (let i = 0; i < 10; i++) {
    if (i % 2 === 0) continue;
    console.log(i);
}

// Modern features
const obj = { name, greeting };
const spread = { ...obj, extra: true };
const optional = obj?.deeply?.nested?.value;
const nullish = value ?? 'default';

// Export
export default Counter;
export { greeting, Counter as MyCounter };
