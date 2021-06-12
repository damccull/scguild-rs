-- manufacturers table
CREATE TABLE manufacturers (
    id           INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name         TEXT    NOT NULL,
    abbreviation TEXT    NOT NULL,
    description  TEXT
);

CREATE INDEX manufacturers_name_idx ON manufacturers (
    name
);

CREATE INDEX manufacturers_abbreviation_idx ON manufacturers (
    abbreviation
);


-- ship_blueprints table

CREATE TABLE ship_blueprints (
    id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name            TEXT    NOT NULL,
    manufacturer_id INTEGER NOT NULL,
    description     TEXT,
    CONSTRAINT ship_blueprints_FK FOREIGN KEY (
        manufacturer_id
    )
    REFERENCES manufacturers (id) ON DELETE RESTRICT
                                  ON UPDATE RESTRICT
);

CREATE INDEX ship_blueprints_name_idx ON ship_blueprints (
    name
);


-- ship_blueprint_classifications table

CREATE TABLE ship_blueprint_classifications (
    id          INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name        TEXT    NOT NULL,
    description TEXT
);

CREATE INDEX ship_blueprint_classifications_id_IDX ON ship_blueprint_classifications (
    id
);

CREATE INDEX ship_blueprint_classifications_name_IDX ON ship_blueprint_classifications (
    name
);


-- ship_blueprint_variants table

CREATE TABLE ship_blueprint_variants (
    id                INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name              TEXT    NOT NULL,
    ship_blueprint_id INTEGER NOT NULL,
    description       TEXT,
    CONSTRAINT ship_blueprint_variants_FK FOREIGN KEY (
        ship_blueprint_id
    )
    REFERENCES ship_blueprints (id) ON DELETE RESTRICT
                                    ON UPDATE RESTRICT
);

CREATE INDEX ship_blueprint_variants_id_IDX ON ship_blueprint_variants (
    id
);

CREATE INDEX ship_blueprint_variants_name_IDX ON ship_blueprint_variants (
    name
);

CREATE INDEX ship_blueprint_variants_ship_blueprint_id_IDX ON ship_blueprint_variants (
    ship_blueprint_id
);


-- ship_blueprint_variant_join_classification table

CREATE TABLE ship_blueprint_variant_join_classification (
    id                               INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    ship_blueprint_variant_id        INTEGER NOT NULL,
    ship_blueprint_classification_id INTEGER NOT NULL,
    CONSTRAINT ship_blueprint_variant_join_classification_UN UNIQUE (
        ship_blueprint_variant_id,
        ship_blueprint_classification_id
    ),
    CONSTRAINT ship_blueprint_variant_join_classification_FK FOREIGN KEY (
        ship_blueprint_variant_id
    )
    REFERENCES ship_blueprint_variants (id) ON DELETE RESTRICT
                                            ON UPDATE RESTRICT,
    CONSTRAINT ship_blueprint_variant_join_classification_FK_1 FOREIGN KEY (
        ship_blueprint_classification_id
    )
    REFERENCES ship_blueprint_classifications (id) ON DELETE RESTRICT
                                                   ON UPDATE RESTRICT
);

CREATE INDEX ship_blueprint_variant_join_classification_id_IDX ON ship_blueprint_variant_join_classification (
    id
);

CREATE INDEX ship_blueprint_variant_join_classification_ship_blueprint_classification_id_IDX ON ship_blueprint_variant_join_classification (
    ship_blueprint_classification_id
);

CREATE INDEX ship_blueprint_variant_join_classification_ship_blueprint_variant_id_IDX ON ship_blueprint_variant_join_classification (
    ship_blueprint_variant_id
);
