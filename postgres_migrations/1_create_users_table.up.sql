CREATE TABLE "users" (
    id bigserial PRIMARY KEY,
    username varchar(256) NOT NULL,
    first_name varchar(256) NOT NULL,
    last_name varchar(256) NOT NULL,
    email varchar(256) NOT NULL UNIQUE,
    encrypted_password varchar(256) NOT NULL,
    phone varchar(256) NOT NULL,
    user_status int NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

