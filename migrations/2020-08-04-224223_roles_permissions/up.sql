-- Your SQL goes here

CREATE TABLE roles (
  id VARCHAR(36) PRIMARY KEY,
  name VARCHAR(32)
);

CREATE TABLE permissions (
  id VARCHAR(36) PRIMARY KEY,
  name VARCHAR(32)
);

CREATE TABLE roles_permissions (
  id VARCHAR(36) PRIMARY KEY,
  role_id VARCHAR(36) REFERENCES roles(id),
  permission_id VARCHAR(36) REFERENCES permissions(id)
);

CREATE TABLE users_roles (
  id VARCHAR(36) PRIMARY KEY,
  user_id VARCHAR(36) REFERENCES users(id),
  role_id VARCHAR(36) REFERENCES roles(id),
  created_at TIMESTAMPTZ DEFAULT now() at time zone 'utc'
);