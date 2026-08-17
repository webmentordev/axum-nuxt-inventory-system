CREATE TYPE barcode_type AS ENUM ('code128', 'ean13', 'upc_a', 'qr');

CREATE TABLE
    barcodes (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        product_id UUID REFERENCES products (id) ON DELETE CASCADE,
        code VARCHAR(100) NOT NULL,
        type barcode_type NOT NULL DEFAULT 'code128',
        is_sold BOOLEAN NOT NULL DEFAULT FALSE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        CONSTRAINT uq_barcodes_code UNIQUE (code)
    );

CREATE INDEX idx_barcodes_product_id ON barcodes (product_id);

CREATE INDEX idx_barcodes_is_sold ON barcodes (is_sold);