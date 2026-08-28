-- A harmless, representative backup bundled for `restore-drill demo`.
CREATE TABLE drill_orders (
  id integer PRIMARY KEY,
  status text NOT NULL,
  total_cents integer NOT NULL
);
INSERT INTO drill_orders (id, status, total_cents) VALUES
  (101, 'paid', 4200),
  (102, 'packed', 1850),
  (103, 'shipped', 7600);
