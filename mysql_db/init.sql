/* Create tables on container start up */
CREATE TABLE tables (
    id INTEGER UNSIGNED PRIMARY KEY,
    seats INTEGER UNSIGNED NOT NULL
);
CREATE TABLE table_items (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    table_id INTEGER UNSIGNED,
    item VARCHAR(90) NOT NULL,
    cook_time TINYINT UNSIGNED NOT NULL,
    customer_id VARCHAR(90) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(90) NOT NULL,
    INDEX idx_item (item),
    INDEX idx_customer_id (customer_id),
    FOREIGN KEY (table_id) REFERENCES tables (id)
);

-- Sample inserts
INSERT INTO tables (id, seats) VALUES (1, 4), (2, 2), (3, 5);

INSERT INTO
    table_items (
        table_id, item, cook_time, customer_id, created_by
    )
VALUES 
    (1, 'Bun Cha', 10, 'Barack Obama', 'Annie'),
    (1, 'Hanoi Beer', 5, 'Barack Obama', 'Annie'),
    (1, 'Bun Cha', 10, 'Anthony Bourdain', 'Annie'),
    (1, 'Tiger Beer', 5, 'Anthony Bourdain', 'Annie'),
    (3, 'Pho', 15, 'Denis Chen', 'Roger')