INSERT INTO wallets (address, balance)
VALUES
    ('0x1234567890abcdef1234567890abcdef12345678', 1000),
    ('0xabcdef1234567890abcdef1234567890abcdef12', 500)
ON CONFLICT (address) DO UPDATE SET balance = EXCLUDED.balance;