INSERT INTO users (name, email, password_hash, role)
VALUES (
    'Administrator',
    'admin@mail.com',
    '$argon2id$v=19$m=19456,t=2,p=1$dHJhZGluZy1sYWItc2VlZA$uttQ5JYeFwLLPaCnxQIQES3Sjc9JUGiNQzZO1rK0TQE',
    'ADMIN'
)
ON CONFLICT (email) DO NOTHING;
