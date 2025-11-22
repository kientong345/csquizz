-- Add up migration script here
INSERT INTO categories (cat_name, cat_image_url, cat_description)
VALUES
    ('Data Structures', 'https://images.shiksha.com/mediadata/ugcDocuments/images/wordpressImages/2020_05_2167870_21e8.jpg', 'Test your knowledge on arrays, linked lists, trees, and more.'),
    ('Algorithms', 'https://www.snexplores.org/wp-content/uploads/2020/11/1030_algorithm_explainer-1028x579.jpg', 'Challenge yourself with sorting, searching, and graph algorithms.'),
    ('Operating Systems', 'https://cloudpso.com/wp-content/uploads/2024/01/ops2.jpg', 'Dive into concepts like processes, memory management, and concurrency.'),
    ('Networking', 'https://www.microsoft.com/en-us/research/wp-content/uploads/2018/08/01_MSR_SIGCOMM_Data_Network_1400x788.png', 'Explore the fundamentals of network protocols and layers.'),
    ('Databases', 'https://techvccloud.mediacdn.vn/2020/11/4/database-la-gi-2-16044569615001962544461.png', 'Understand SQL, normalization, and database design principles.'),
    ('Artificial Intelligence', 'https://engineering.fb.com/wp-content/uploads/2019/05/grid-AI.jpg', 'Get started with the basic concepts of AI and machine learning.'),
    ('Software Engineering', 'https://investin.org/cdn/shop/articles/software-engineering-skills_resize_md.jpg', 'Learn about software development methodologies and best practices.'),
    ('Cybersecurity', 'https://www.iare.ac.in/sites/default/files/department_images/Cybersecurity.jpg', 'Test your knowledge on security principles and practices.'),
    ('Web Development', 'https://spec.nith.ac.in/BLOGS/a1%20(5).jpg', 'Explore front-end and back-end web development concepts.'),
    ('Programming Languages', 'https://binarapps.com/wp-content/uploads/2021/09/Top-10-Programming-Languages-of-the-Future.png', 'Understand different programming paradigms and language features.')
;
