-- Add migration script here

/* `user` is reserved keyword in postgres.
   using "user" with double quotes requires string escaping all the time.
   `user_` is the current best option.
*/
create table if not exists user_ (
   id text primary key,
   name text not null unique,
);
