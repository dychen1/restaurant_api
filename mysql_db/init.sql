/* Create tables on container start up */
CREATE TABLE
    tables (
        id INTEGER UNSIGNED PRIMARY KEY,
        seats INTEGER UNSIGNED NOT NULL
    );

CREATE TABLE
    orders (
        id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT,
        table_id INTEGER UNSIGNED,
        customer_id VARCHAR(90),
        is_active BOOLEAN DEFAULT TRUE,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
        created_by VARCHAR(90) NOT NULL,
        FOREIGN KEY (table_id) REFERENCES tables(id)
    );

CREATE TABLE
    order_items (
        order_id INTEGER UNSIGNED,
        item VARCHAR(90) NOT NULL,
        number_of_items SMALLINT UNSIGNED DEFAULT 1,
        cook_time TINYINT UNSIGNED NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        -- Could add index for created_at
        created_by VARCHAR(90) NOT NULL,
        INDEX idx_item (item),
        FOREIGN KEY (order_id) REFERENCES orders(id)
    );

-- Sample inserts
INSERT INTO tables (id, seats) VALUES (1, 4), (2, 2), (3, 5);

INSERT INTO
    orders (
        table_id,
        customer_id,
        is_active,
        created_by
    )
VALUES (1, 'Barack', TRUE, 'Annie'), (1, 'Anthony', TRUE, 'Annie'), (2, 'Denis', TRUE, 'Roger'), (
        1,
        'PriorCustomer',
        FALSE,
        'Annie'
    );

INSERT INTO
    order_items (
        order_id,
        item,
        number_of_items,
        cook_time,
        created_by
    )
VALUES (1, 'Bun Cha', 2, 10, 'Annie'), (2, 'Hanoi Beer', 2, 5, 'Annie'), (3, 'Pho', 1, 15, 'Roger')