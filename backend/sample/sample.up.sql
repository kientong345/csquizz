-- Clear existing data to prevent conflicts on re-running the script
TRUNCATE TABLE users, categories, quizzes, questions, results, user_answers RESTART IDENTITY CASCADE;

-- =================================================================
-- USERS
-- =================================================================
-- Note: Passwords are encrypted. 'admin' password is 'admin123', 'testuser' password is 'user123'
INSERT INTO users (display_name, email, password_hash, role) VALUES
('bocchi_the_dev', 'bocchi345@gmail.com', '$2b$12$p/tWlC5S42y2N6Kz2JzL1.RPwZJc2jV/bJv5efe3h3dG4n2a.g/S2', 'admin'),
('super_user', 'su123@gmail.com', '$2b$12$8.iEAO2h2N.JgHk4QvODUuWcIkdJcrfC5x2wz0Ea.A5uL2S.I/2/G', 'user');

-- =================================================================
-- CATEGORIES
-- =================================================================
INSERT INTO categories (name, description) VALUES
('Data Structures', 'Questions about fundamental data structures like arrays, linked lists, trees, etc.'),
('Algorithms', 'Questions about sorting, searching, and other common algorithms.'),
('Networking', 'Questions about network protocols, layers, and concepts.');

-- =================================================================
-- QUIZZES
-- =================================================================
INSERT INTO quizzes (id, title, description, category, difficulty, created_by) VALUES
(1, 'Array and String Basics', 'Test your knowledge on basic array and string manipulations.', 1, 'easy', 1),
(2, 'Introduction to Sorting', 'Questions about simple sorting algorithms like Bubble Sort and Insertion Sort.', 2, 'easy', 1),
(3, 'The OSI Model', 'Questions about the 7 layers of the OSI model.', 3, 'medium', 1);

-- =================================================================
-- QUESTIONS & OPTIONS
-- =================================================================

-- Quiz 1: Array and String Basics
-- Question 1 (Single Choice)
INSERT INTO questions (id, quiz_id, question_type, question_text, answer_key) VALUES
(1, 1, 'single-choice', 'What is the time complexity for accessing an element in an array by its index?',
'[
    {
        "content": "O(1)",
        "is_correct": true,
        "explanation": "Direct access via index is a constant time operation, regardless of the array size."
    },
    {
        "content": "O(n)",
        "is_correct": false
    },
    {
        "content": "O(log n)",
        "is_correct": false
    },
    {
        "content": "O(n^2)",
        "is_correct": false
    }
]');

-- Question 2 (Multiple Choice)
INSERT INTO questions (id, quiz_id, question_type, question_text, answer_key) VALUES
(2, 1, 'multiple-choice', 'Which of the following are mutable in Java?',
'[
    {
        "content": "String",
        "is_correct": false
    },
    {
        "content": "StringBuilder",
        "is_correct": true,
        "explanation": "StringBuilder is mutable and allows modifications without creating new objects."
    },
    {
        "content": "StringBuffer",
        "is_correct": true,
        "explanation": "StringBuffer is mutable and thread-safe, allowing modifications."
    },
    {
        "content": "All of the above",
        "is_correct": false
    }
]');

-- Question 3 (Text Entry)
INSERT INTO questions (id, quiz_id, question_type, question_text, answer_key) VALUES
(3, 1, 'text-entry', 'In C++, which header must be included to use std::vector?',
'{
    "correct_entry": "<vector>",
    "explanation": "The <vector> header is required to use the std::vector class, which provides a dynamic array implementation."
}');

-- Quiz 2: Introduction to Sorting
-- Question 4 (Single Choice)
INSERT INTO questions (id, quiz_id, question_type, question_text, answer_key) VALUES
(4, 2, 'single-choice', 'What is the worst-case time complexity of Bubble Sort?',
'[
    {
        "content": "O(n log n)",
        "is_correct": false
    },
    {
        "content": "O(n)",
        "is_correct": false
    },
    {
        "content": "O(n^2)",
        "is_correct": true,
        "explanation": "In the worst case, Bubble Sort requires n-1 passes through the array, with each pass involving n comparisons."
    },
    {
        "content": "O(1)",
        "is_correct": false
    }
]');

-- Question 5 (Single Choice with image)
INSERT INTO questions (id, quiz_id, question_type, question_text, image_url, answer_key) VALUES
(5, 2, 'single-choice', 'Which sorting algorithm does the following diagram represent?', 'https://upload.wikimedia.org/wikipedia/commons/c/c8/Bubble-sort-example-300px.gif',
'[
    {
        "content": "Insertion Sort",
        "is_correct": false
    },
    {
        "content": "Selection Sort",
        "is_correct": false
    },
    {
        "content": "Bubble Sort",
        "is_correct": true,
        "explanation": "The diagram illustrates the process of Bubble Sort, where larger elements bubble to the end of the list through successive swaps."
    },
    {
        "content": "Merge Sort",
        "is_correct": false
    }
]');

-- Quiz 3: The OSI Model
-- Question 6 (Single Choice)
INSERT INTO questions (id, quiz_id, question_type, question_text, answer_key) VALUES
(6, 3, 'single-choice', 'Which layer of the OSI model is responsible for routing packets between networks?',
'[
    {
        "content": "Data Link Layer",
        "is_correct": false
    },
    {
        "content": "Network Layer",
        "is_correct": true,
        "explanation": "The Network Layer (Layer 3) is responsible for logical addressing and routing of packets across different networks."
    },
    {
        "content": "Transport Layer",
        "is_correct": false
    },
    {
        "content": "Physical Layer",
        "is_correct": false
    }
]');

-- Question 7 (Single Choice)
INSERT INTO questions (id, quiz_id, question_type, question_text, answer_key) VALUES
(7, 3, 'single-choice', 'HTTP (Hypertext Transfer Protocol) operates at which layer of the OSI model?',
'[
    {
        "content": "Session Layer",
        "is_correct": false
    },
    {
        "content": "Presentation Layer",
        "is_correct": false
    },
    {
        "content": "Transport Layer",
        "is_correct": false
    },
    {
        "content": "Application Layer",
        "is_correct": true,
        "explanation": "HTTP is an application protocol, so it resides at the Application Layer (Layer 7)."
    }
]');
