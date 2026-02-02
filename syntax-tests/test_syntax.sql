-- SQL Syntax Test

-- Single line comment

/*
 * Multi-line comment
 * Database schema example
 */

-- Create database
CREATE DATABASE IF NOT EXISTS demo_db;
USE demo_db;

-- Create tables
CREATE TABLE users (
    id INT PRIMARY KEY AUTO_INCREMENT,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) NOT NULL,
    password_hash CHAR(64) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE
);

CREATE TABLE posts (
    id INT PRIMARY KEY AUTO_INCREMENT,
    user_id INT NOT NULL,
    title VARCHAR(200) NOT NULL,
    content TEXT,
    published_at DATETIME,
    view_count INT DEFAULT 0,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    INDEX idx_user_id (user_id),
    INDEX idx_published_at (published_at)
);

CREATE TABLE comments (
    id INT PRIMARY KEY AUTO_INCREMENT,
    post_id INT NOT NULL,
    user_id INT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Insert data
INSERT INTO users (username, email, password_hash) 
VALUES 
    ('alice', 'alice@example.com', 'hash1'),
    ('bob', 'bob@example.com', 'hash2'),
    ('charlie', 'charlie@example.com', 'hash3');

INSERT INTO posts (user_id, title, content, published_at)
SELECT id, 'First Post', 'Content here', NOW()
FROM users WHERE username = 'alice';

-- Update data
UPDATE users 
SET email = 'newemail@example.com', 
    updated_at = NOW()
WHERE username = 'alice';

UPDATE posts 
SET view_count = view_count + 1 
WHERE id = 1;

-- Select queries
SELECT * FROM users;

SELECT u.username, COUNT(p.id) as post_count
FROM users u
LEFT JOIN posts p ON u.id = p.user_id
GROUP BY u.id, u.username
HAVING post_count > 0
ORDER BY post_count DESC;

-- Subqueries
SELECT username, email
FROM users
WHERE id IN (
    SELECT DISTINCT user_id 
    FROM posts 
    WHERE published_at > DATE_SUB(NOW(), INTERVAL 7 DAY)
);

-- Join examples
SELECT 
    u.username,
    p.title,
    p.published_at,
    COUNT(c.id) as comment_count
FROM users u
INNER JOIN posts p ON u.id = p.user_id
LEFT JOIN comments c ON p.id = c.post_id
WHERE p.published_at IS NOT NULL
GROUP BY p.id
ORDER BY p.published_at DESC
LIMIT 10;

-- Aggregate functions
SELECT 
    user_id,
    COUNT(*) as total_posts,
    SUM(view_count) as total_views,
    AVG(view_count) as avg_views,
    MAX(view_count) as max_views,
    MIN(view_count) as min_views
FROM posts
GROUP BY user_id;

-- Window functions
SELECT 
    username,
    email,
    ROW_NUMBER() OVER (ORDER BY created_at) as row_num,
    RANK() OVER (ORDER BY created_at) as rank_num,
    LAG(email) OVER (ORDER BY created_at) as prev_email
FROM users;

-- Transactions
START TRANSACTION;

UPDATE users SET is_active = FALSE WHERE id = 1;
DELETE FROM posts WHERE user_id = 1;

COMMIT;
-- or ROLLBACK;

-- Create views
CREATE VIEW active_users_with_posts AS
SELECT 
    u.id,
    u.username,
    u.email,
    COUNT(p.id) as post_count
FROM users u
LEFT JOIN posts p ON u.id = p.user_id
WHERE u.is_active = TRUE
GROUP BY u.id;

-- Alter table
ALTER TABLE users 
ADD COLUMN last_login TIMESTAMP,
ADD INDEX idx_last_login (last_login);

-- Drop statements
DROP VIEW IF EXISTS active_users_with_posts;
DROP TABLE IF EXISTS comments;
DROP TABLE IF EXISTS posts;
DROP TABLE IF EXISTS users;
DROP DATABASE IF EXISTS demo_db;
