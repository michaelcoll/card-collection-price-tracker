CREATE TABLE set_name
(
    set_code VARCHAR(3) PRIMARY KEY,
    name     VARCHAR(255) NOT NULL
);

INSERT INTO set_name (set_code, name)
VALUES ('ECL', 'Lorwyn Eclipsed'),
       ('FDN', 'Foundations');

CREATE TABLE card
(
    set_code         VARCHAR(3) NOT NULL,
    collector_number INTEGER    NOT NULL,
    language_code    VARCHAR(2) NOT NULL,
    foil             BOOLEAN    NOT NULL,
    quantity         INTEGER    NOT NULL,
    purchase_price   INTEGER    NOT NULL,


    CONSTRAINT card_pk PRIMARY KEY (set_code, collector_number, language_code, foil),
    CONSTRAINT card_set_name_fk FOREIGN KEY (set_code) REFERENCES set_name (set_code)
);

CREATE TABLE collection_price_history
(
    date  DATE PRIMARY KEY,
    low   INTEGER NOT NULL,
    trend INTEGER NOT NULL,
    avg1  INTEGER NOT NULL,
    avg7  INTEGER NOT NULL,
    avg30 INTEGER NOT NULL
);
