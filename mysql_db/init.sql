/* Create tables on container start up */
CREATE TABLE tables (
    id INTEGER UNSIGNED PRIMARY KEY,
    seats INTEGER UNSIGNED NOT NULL
);
CREATE TABLE items (
    id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    table_id INTEGER UNSIGNED NOT NULL, 
    item VARCHAR(90) NOT NULL, 
    cook_time TINYINT UNSIGNED NOT NULL,
    customer_id VARCHAR(90),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    INDEX idx_item (item),
    INDEX idx_customer_id (customer_id),
    FOREIGN KEY (table_id) REFERENCES tables (id) 
        ON DELETE CASCADE 
        ON UPDATE CASCADE
);
-- NOTE: sqlx will not know index and fk columns are NOT NULLABLE without explicitly setting it 

-- Sample inserts
INSERT INTO tables (id, seats) VALUES (1, 4), (2, 2), (3, 5);

INSERT INTO
    items (
        table_id, item, cook_time, customer_id 
    )
VALUES 
    (1, 'Bun Cha', 10, 'Barack Obama'), 
    (1, 'Hanoi Beer', 5, 'Barack Obama'),
    (1, 'Bun Cha', 10, 'Anthony Bourdain'),
    (1, 'Tiger Beer', 5, 'Anthony Bourdain'),
    (2, 'Pho', 15, 'Denis Chen'),
    (2, 'Pho', 15, 'Denis Chen')