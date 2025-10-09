-- Clear existing data to prevent conflicts on re-running the script
TRUNCATE TABLE users, categories, quizzes, questions, options, results, user_answers RESTART IDENTITY CASCADE;

-- =================================================================
-- USERS
-- =================================================================
-- Note: Passwords are encrypted. 'admin' password is 'admin123', 'testuser' password is 'user123'
INSERT INTO users (username, email, password_hash, role) VALUES
('admin', 'admin@quizbank.com', '$2b$12$p/tWlC5S42y2N6Kz2JzL1.RPwZJc2jV/bJv5efe3h3dG4n2a.g/S2', 'admin'),
('testuser', 'user@quizbank.com', '$2b$12$8.iEAO2h2N.JgHk4QvODUuWcIkdJcrfC5x2wz0Ea.A5uL2S.I/2/G', 'user');

-- =================================================================
-- CATEGORIES
-- =================================================================
INSERT INTO categories (id, name, description) VALUES
(1, 'Data Structures', 'Questions about fundamental data structures like arrays, linked lists, trees, etc.'),
(2, 'Algorithms', 'Questions about sorting, searching, and other common algorithms.'),
(3, 'Networking', 'Questions about network protocols, layers, and concepts.');

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
INSERT INTO questions (id, quiz_id, question_type, question_text, explanation) VALUES
(1, 1, 'single-choice', 'What is the time complexity for accessing an element in an array by its index?', 'Direct access via index is a constant time operation, regardless of the array size.');
INSERT INTO options (question_id, option_text, is_correct) VALUES
(1, 'O(1)', TRUE),
(1, 'O(n)', FALSE),
(1, 'O(log n)', FALSE),
(1, 'O(n^2)', FALSE);

-- Question 2 (Multiple Choice)
INSERT INTO questions (id, quiz_id, question_type, question_text, explanation) VALUES
(2, 1, 'multiple-choice', 'Which of the following are mutable in Java?', 'Strings are immutable, while StringBuilder and StringBuffer are mutable.');
INSERT INTO options (question_id, option_text, is_correct) VALUES
(2, 'String', FALSE),
(2, 'StringBuilder', TRUE),
(2, 'StringBuffer', TRUE),
(2, 'All of the above', FALSE);

-- Question 3 (Text Entry)
INSERT INTO questions (id, quiz_id, question_type, question_text, correct_text_answer, explanation) VALUES
(3, 1, 'text-entry', 'In C++, which header must be included to use std::vector?', '<vector>', 'The <vector> header contains the definition for the std::vector class template.');

-- Quiz 2: Introduction to Sorting
-- Question 4 (Single Choice)
INSERT INTO questions (id, quiz_id, question_type, question_text, explanation) VALUES
(4, 2, 'single-choice', 'What is the worst-case time complexity of Bubble Sort?', 'Bubble Sort has a worst-case complexity of O(n^2) when the array is sorted in reverse order.');
INSERT INTO options (question_id, option_text, is_correct) VALUES
(4, 'O(n log n)', FALSE),
(4, 'O(n)', FALSE),
(4, 'O(n^2)', TRUE),
(4, 'O(1)', FALSE);

-- Question 5 (Single Choice with image)
INSERT INTO questions (id, quiz_id, question_type, question_text, image_url, explanation) VALUES
(5, 2, 'single-choice', 'Which sorting algorithm does the following diagram represent?', 'https://upload.wikimedia.org/wikipedia/commons/c/c8/Bubble-sort-example-300px.gif', 'The diagram shows adjacent elements being repeatedly compared and swapped, which is characteristic of Bubble Sort.');
INSERT INTO options (question_id, option_text, is_correct) VALUES
(5, 'Insertion Sort', FALSE),
(5, 'Selection Sort', FALSE),
(5, 'Bubble Sort', TRUE),
(5, 'Merge Sort', FALSE);


-- Quiz 3: The OSI Model
-- Question 6 (Single Choice)
INSERT INTO questions (id, quiz_id, question_type, question_text, explanation) VALUES
(6, 3, 'single-choice', 'Which layer of the OSI model is responsible for routing packets between networks?', 'The Network Layer (Layer 3) handles logical addressing and routing.');
INSERT INTO options (question_id, option_text, is_correct) VALUES
(6, 'Data Link Layer', FALSE),
(6, 'Network Layer', TRUE),
(6, 'Transport Layer', FALSE),
(6, 'Physical Layer', FALSE);

-- Question 7 (Single Choice)
INSERT INTO questions (id, quiz_id, question_type, question_text, explanation) VALUES
(7, 3, 'single-choice', 'HTTP (Hypertext Transfer Protocol) operates at which layer of the OSI model?', 'HTTP is an application protocol, so it resides at the Application Layer (Layer 7).');
INSERT INTO options (question_id, option_text, is_correct) VALUES
(7, 'Session Layer', FALSE),
(7, 'Presentation Layer', FALSE),
(7, 'Transport Layer', FALSE),
(7, 'Application Layer', TRUE);
