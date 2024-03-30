-- Add `status` Column to `subscriptions` Table
ALTER TABLE subscriptions
    ADD COLUMN status TEXT NULL;
