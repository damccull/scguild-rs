BEGIN;

CREATE TABLE IF NOT EXISTS public.manufacturers (
    id uuid NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    code TEXT NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS public.ship_models (
    id uuid NOT NULL,
    class_name TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    manufacturer_id uuid NOT NULL,
    focus TEXT,
    career TEXT NOT NULL,
    role TEXT NOT NULL,
    size INTEGER NOT NULL,
    cargo_amount INTEGER NOT NULL,
    crew INTEGER NOT NULL,
    weapon_crew INTEGER NOT NULL,
    operations_crew INTEGER NOT NULL,
    mass INTEGER NOT NULL,
    is_spaceship BOOLEAN NOT NULL,
    is_vehicle BOOLEAN NOT NULL,
    is_gravlev BOOLEAN NOT NULL,
    PRIMARY KEY (id)
);

ALTER TABLE IF EXISTS public.ship_models
    ADD CONSTRAINT manufacturer_id
    FOREIGN KEY (manufacturer_id)
    REFERENCES public.manufacturers (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION
    NOT VALID;

CREATE TABLE IF NOT EXISTS public.users (
    id uuid NOT NULL,
    discord_id text,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS public.user_ships (
    id uuid NOT NULL,
    user_id uuid NOT NULL,
    ship_model_id uuid NOT NULL,
    name text,
    PRIMARY KEY (id)
);

ALTER TABLE IF EXISTS public.user_ships
    ADD CONSTRAINT user_id
    FOREIGN KEY (user_id)
    REFERENCES public.users (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION
    NOT VALID;

ALTER TABLE IF EXISTS public.user_ships
    ADD CONSTRAINT ship_model_id
    FOREIGN KEY (ship_model_id)
    REFERENCES public.ship_models (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION
    NOT VALID;

END;