BEGIN;


CREATE TABLE IF NOT EXISTS public.ship_models
(
    id uuid NOT NULL,
    class_name text NOT NULL UNIQUE,
    name text NOT NULL,
    description text,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS public.users
(
    id uuid NOT NULL,
    discord_id text,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS public.user_ships
(
    id uuid NOT NULL,
    user_id uuid NOT NULL,
    ship_model_id uuid NOT NULL,
    name text,
    PRIMARY KEY (id)
);

ALTER TABLE IF EXISTS public.user_ships
    ADD CONSTRAINT user_id FOREIGN KEY (user_id)
    REFERENCES public.users (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION
    NOT VALID;


ALTER TABLE IF EXISTS public.user_ships
    ADD CONSTRAINT ship_model_id FOREIGN KEY (ship_model_id)
    REFERENCES public.ship_models (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION
    NOT VALID;

END;