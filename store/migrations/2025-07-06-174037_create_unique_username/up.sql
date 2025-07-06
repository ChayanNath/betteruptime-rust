-- Your SQL goes here
ALTER TABLE "user"
ADD CONSTRAINT "user_username_key" UNIQUE ("username");