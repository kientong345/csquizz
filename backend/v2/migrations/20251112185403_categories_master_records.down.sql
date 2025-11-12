-- Add down migration script here
DELETE FROM categories
WHERE name = ANY(ARRAY[
    'Data Structures',
    'Algorithms',
    'Operating Systems',
    'Networking',
    'Databases',
    'Artificial Intelligence',
    'Software Engineering',
    'Cybersecurity',
    'Web Development',
    'Programming Languages'
]);